use std::str::FromStr;

use cosmwasm_std::{coins, to_json_binary, BankMsg, Decimal, DepsMut, Env, MessageInfo, QueryRequest, Response, Uint128, WasmMsg, WasmQuery};
use cw721::OwnerOfResponse;

use crate::{ContractError, state::{NftListing, NFT_LISTINGS, PLATFORM_FEE_RECEIVER}, utils::{parse_decimal, query_check_royalties, query_royalty_info}};

pub fn list(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    price: String,
    nft_contract_address: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let price = Decimal::from_str(price.as_str())
        .map_err(|_e| ContractError::InvalidPrice {  })?;
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let nft_listing = NftListing {
        lister: info.sender,
        price,
        nft_contract_address,
        token_id
    };

    NFT_LISTINGS.save(
        deps.storage,
        (&nft_listing.nft_contract_address.to_string(), &nft_listing.token_id),
        &nft_listing
    )
        .map_err(|_e| ContractError::ErrorCreatingNewListing {  })?;

    // verify that escrow has the NFT
    let cw721_query_owner_msg = cw721::Cw721QueryMsg::OwnerOf {
        token_id: nft_listing.token_id.clone(),
        include_expired: Some(false),
    };

    let cw721_query = QueryRequest::Wasm(
        WasmQuery::Smart {
            contract_addr: nft_listing.nft_contract_address.to_string(),
            msg: to_json_binary(&cw721_query_owner_msg)?
        }
    );

    let cw721_query_response: OwnerOfResponse = deps.querier.query(&cw721_query)?;
    if cw721_query_response.owner != env.contract.address.to_string() {
        return Err(ContractError::NftNotInEscrow {  });
    }

    Ok(
        Response::new()
            .add_attribute("action", "list")
            .add_attribute("price", parse_decimal(nft_listing.price)?.to_string())
            .add_attribute("lister", nft_listing.lister)
            .add_attribute("nft_contract_address", nft_listing.nft_contract_address)
            .add_attribute("token_id", nft_listing.token_id)
    )
}

pub fn buy_listing(
    deps: DepsMut,
    info: MessageInfo,
    nft_contract_address: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let key = (nft_contract_address.as_str(), token_id.as_str());

    let nft_listing = NFT_LISTINGS.load(deps.storage, key)
        .map_err(|_e| ContractError::NftListingNotFound {  })?;

    NFT_LISTINGS.remove(deps.storage, key);

    // transfer nft from escrow to buyer
    let cw721_transfer_nft_msg = cw721::Cw721ExecuteMsg::TransferNft {
        recipient: info.sender.to_string(),
        token_id: nft_listing.token_id.clone()
    };
    let transfer_nft_msg = WasmMsg::Execute {
        contract_addr: nft_listing.nft_contract_address.to_string(),
        msg: to_json_binary(&cw721_transfer_nft_msg)?,
        funds: vec![]
    };

    // transfer sei to lister
    let transfer_sei_msg = BankMsg::Send {
        to_address: nft_listing.lister.to_string(),
        amount: coins(parse_decimal(nft_listing.price)?.u128(), "usei")
    };

    // pay platform fee
    let platform_fee = nft_listing.price * Decimal::percent(2);
    let pay_platform_fee_msg = BankMsg::Send {
        to_address: PLATFORM_FEE_RECEIVER.to_string(),
        amount: coins(parse_decimal(platform_fee)?.u128(), "usei")
    };

    let mut response = Response::new()
        .add_message(transfer_nft_msg)
        .add_message(transfer_sei_msg)
        .add_message(pay_platform_fee_msg)
        .add_attribute("action", "buy_listing")
        .add_attribute("price", parse_decimal(nft_listing.price)?.to_string())
        .add_attribute("lister", nft_listing.lister)
        .add_attribute("buyer", info.sender.to_string())
        .add_attribute("nft_contract_address", nft_listing.nft_contract_address.clone())
        .add_attribute("token_id", nft_listing.token_id.clone());

    // pay royalties
    let check_royalties_response_result = query_check_royalties(&deps, nft_listing.nft_contract_address.to_string());
    match check_royalties_response_result {
        Ok(check_royalties_response) => {
            if check_royalties_response.royalty_payments {
                let royalty_info_response = query_royalty_info(
                    &deps,
                    nft_listing.nft_contract_address.to_string(),
                    nft_listing.token_id.clone(),
                    parse_decimal(nft_listing.price)?,
                )?;
                if !royalty_info_response.address.is_empty() && royalty_info_response.royalty_amount > Uint128::zero() {
                    let pay_royalties_msg = BankMsg::Send {
                        to_address: royalty_info_response.address,
                        amount: coins(royalty_info_response.royalty_amount.u128(), "usei")
                    };
                    response = response.add_message(pay_royalties_msg);
                }
            }
        },
        // if there is an error that means the nft contract does not support royalties
        Err(_e) => {}
    }
    
    Ok(response)
}

pub fn cancel_listing(
    deps: DepsMut,
    info: MessageInfo,
    nft_contract_address: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let key = (nft_contract_address.as_str(), token_id.as_str());

    let nft_listing = NFT_LISTINGS.load(deps.storage, key)
        .map_err(|_e| ContractError::NftListingNotFound {  })?;

    NFT_LISTINGS.remove(deps.storage, key);

    if info.sender != nft_listing.lister {
        return Err(ContractError::Unauthorized {  });
    }

    // transfer nft from escrow back to lister
    let cw721_transfer_nft_msg = cw721::Cw721ExecuteMsg::TransferNft {
        recipient: info.sender.to_string(),
        token_id: nft_listing.token_id.clone()
    };
    let transfer_nft_msg = WasmMsg::Execute {
        contract_addr: nft_listing.nft_contract_address.to_string(),
        msg: to_json_binary(&cw721_transfer_nft_msg)?,
        funds: vec![]
    };

    let response = Response::new()
        .add_message(transfer_nft_msg)
        .add_attribute("action", "cancel_listing")
        .add_attribute("price", parse_decimal(nft_listing.price)?.to_string())
        .add_attribute("lister", nft_listing.lister)
        .add_attribute("nft_contract_address", nft_listing.nft_contract_address.clone())
        .add_attribute("token_id", nft_listing.token_id.clone());

    Ok(response)
}

pub fn delist(
    deps: DepsMut,
    info: MessageInfo,
    nft_contract_address: String,
    token_id: String,
    new_price: String,
) -> Result<Response, ContractError> {
    let new_price = Decimal::from_str(new_price.as_str())
        .map_err(|_e| ContractError::InvalidPrice {  })?;
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let key = (nft_contract_address.as_str(), token_id.as_str());

    let nft_listing = NFT_LISTINGS.update(deps.storage, key, |nft_listing_option| -> Result<NftListing, ContractError> {
        match nft_listing_option {
            Some(nft_listing) => {
                if info.sender != nft_listing.lister {
                    return Err(ContractError::Unauthorized {  });
                }
                Ok(NftListing {
                    price: new_price,
                    lister: nft_listing.lister,
                    nft_contract_address: nft_listing.nft_contract_address,
                    token_id: nft_listing.token_id
                })
            },
            None => Err(ContractError::NftListingNotFound {  })
        }
    })?;

    let response = Response::new()
        .add_attribute("action", "delist")
        .add_attribute("new_price", parse_decimal(nft_listing.price)?.to_string())
        .add_attribute("lister", nft_listing.lister)
        .add_attribute("nft_contract_address", nft_listing.nft_contract_address.clone())
        .add_attribute("token_id", nft_listing.token_id.clone());

    Ok(response)
}