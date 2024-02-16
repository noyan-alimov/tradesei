use std::str::FromStr;

use cosmwasm_std::{coins, to_json_binary, BankMsg, Decimal, DepsMut, MessageInfo, Response, Uint128, WasmMsg};

use crate::{state::{NftBid, NFT_BIDS, PLATFORM_FEE_RECEIVER}, utils::{add_transfer_sei_to_seller_msg_with_price_after_platform_fee, parse_decimal, query_check_royalties, query_royalty_info}, ContractError};


pub fn bid(
    deps: DepsMut,
    info: MessageInfo,
    price: String,
    nft_contract_address: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let price = Decimal::from_str(price.as_str())
        .map_err(|_e| ContractError::InvalidPrice {  })?;
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let nft_bid = NftBid {
        bidder: info.sender,
        price,
        nft_contract_address,
        token_id
    };

    let key = (nft_bid.nft_contract_address.as_str(), nft_bid.token_id.as_str(), nft_bid.bidder.as_str());

    let is_bid_exists = NFT_BIDS.has(deps.storage, key);
    if is_bid_exists {
        return Err(ContractError::BidAlreadyExists {  });
    }

    NFT_BIDS.save(
        deps.storage,
        key,
        &nft_bid
    )
        .map_err(|_e| ContractError::ErrorCreatingNewBid {  })?;

    let sent_amount = info
        .funds
        .iter()
        .find(|coin| coin.denom == "usei".to_string())
        .map_or(Uint128::zero(), |coin| coin.amount);

    // Check if the sent amount is sufficient
    if sent_amount < parse_decimal(price)? {
        return Err(ContractError::InsufficientFundsSent {  });
    }

    Ok(
        Response::new()
            .add_attribute("action", "bid")
            .add_attribute("price", parse_decimal(nft_bid.price)?.to_string())
            .add_attribute("bidder", nft_bid.bidder)
            .add_attribute("nft_contract_address", nft_bid.nft_contract_address)
            .add_attribute("token_id", nft_bid.token_id)
    )
}

pub fn sell_to_bid(
    deps: DepsMut,
    info: MessageInfo,
    nft_contract_address: String,
    token_id: String,
    bidder: String,
) -> Result<Response, ContractError> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let bidder = deps.api.addr_validate(bidder.as_str())
        .map_err(|_e| ContractError::InvalidBidder {  })?;

    let key = (nft_contract_address.as_str(), token_id.as_str(), bidder.as_str());

    let nft_bid = NFT_BIDS.load(deps.storage, key)
        .map_err(|_e| ContractError::NftBidNotFound {  })?;

    NFT_BIDS.remove(deps.storage, key);

    // transfer nft from escrow to bidder
    let cw721_transfer_nft_msg = cw721::Cw721ExecuteMsg::TransferNft {
        recipient: nft_bid.bidder.to_string(),
        token_id: nft_bid.token_id.clone()
    };
    let transfer_nft_msg = WasmMsg::Execute {
        contract_addr: nft_bid.nft_contract_address.to_string(),
        msg: to_json_binary(&cw721_transfer_nft_msg)?,
        funds: vec![]
    };

    // pay platform fee
    let platform_fee = nft_bid.price * Decimal::percent(2);
    let pay_platform_fee_msg = BankMsg::Send {
        to_address: PLATFORM_FEE_RECEIVER.to_string(),
        amount: coins(parse_decimal(platform_fee)?.u128(), "usei")
    };

    let price_after_platform_fee = nft_bid.price.checked_sub(platform_fee)
        .map_err(|e| ContractError::Std(cosmwasm_std::StdError::Overflow { source: e }))?;

    let mut response = Response::new()
        .add_message(transfer_nft_msg)
        .add_message(pay_platform_fee_msg)
        .add_attribute("action", "sell_to_bid")
        .add_attribute("price", parse_decimal(nft_bid.price)?.to_string())
        .add_attribute("bidder", nft_bid.bidder)
        .add_attribute("seller", info.sender.to_string())
        .add_attribute("nft_contract_address", nft_bid.nft_contract_address.clone())
        .add_attribute("token_id", nft_bid.token_id.clone());

    // pay royalties
    let check_royalties_response_result = query_check_royalties(&deps, nft_bid.nft_contract_address.to_string());
    match check_royalties_response_result {
        Ok(check_royalties_response) => {
            if check_royalties_response.royalty_payments {
                let royalty_info_response = query_royalty_info(
                    &deps,
                    nft_bid.nft_contract_address.to_string(),
                    nft_bid.token_id.clone(),
                    parse_decimal(nft_bid.price)?,
                )?;
                if !royalty_info_response.address.is_empty() && royalty_info_response.royalty_amount > Uint128::zero() {
                    let pay_royalties_msg = BankMsg::Send {
                        to_address: royalty_info_response.address,
                        amount: coins(royalty_info_response.royalty_amount.u128(), "usei")
                    };
                    let price_after_platform_fee_and_royalties = parse_decimal(price_after_platform_fee)?.checked_sub(royalty_info_response.royalty_amount)
                        .map_err(|e| ContractError::Std(cosmwasm_std::StdError::Overflow { source: e }))?;
                    let transfer_sei_msg = BankMsg::Send {
                        to_address: info.sender.to_string(),
                        amount: coins(price_after_platform_fee_and_royalties.u128(), "usei")
                    };
                    response = response.add_message(pay_royalties_msg).add_message(transfer_sei_msg)
                } else {
                    response = add_transfer_sei_to_seller_msg_with_price_after_platform_fee(info.sender.to_string(), parse_decimal(price_after_platform_fee)?.u128(), response);
                }
            } else {
                response = add_transfer_sei_to_seller_msg_with_price_after_platform_fee(info.sender.to_string(), parse_decimal(price_after_platform_fee)?.u128(), response);
            }
        },
        // if there is an error that means the nft contract does not support royalties
        Err(_e) => {
            response = add_transfer_sei_to_seller_msg_with_price_after_platform_fee(info.sender.to_string(), parse_decimal(price_after_platform_fee)?.u128(), response);
        }
    }
    
    Ok(response)
}

pub fn cancel_bid(
    deps: DepsMut,
    info: MessageInfo,
    nft_contract_address: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let key = (nft_contract_address.as_str(), token_id.as_str(), info.sender.as_str());

    let nft_bid = NFT_BIDS.load(deps.storage, key)
        .map_err(|_e| ContractError::NftBidNotFound {  })?;

    NFT_BIDS.remove(deps.storage, key);

    // transfer sei from escrow back to bidder
    let transfer_sei_msg = BankMsg::Send {
        to_address: nft_bid.bidder.to_string(),
        amount: coins(parse_decimal(nft_bid.price)?.u128(), "usei")
    };

    let response = Response::new()
        .add_message(transfer_sei_msg)
        .add_attribute("action", "cancel_bid")
        .add_attribute("price", parse_decimal(nft_bid.price)?.to_string())
        .add_attribute("bidder", nft_bid.bidder)
        .add_attribute("nft_contract_address", nft_bid.nft_contract_address.clone())
        .add_attribute("token_id", nft_bid.token_id.clone());

    Ok(response)
}

pub fn update_bid(
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

    let mut transfer_sei_msg = None;
    let key = (nft_contract_address.as_str(), token_id.as_str(), info.sender.as_str());

    let nft_bid = NFT_BIDS.update(deps.storage, key, |nft_bid_option| -> Result<NftBid, ContractError> {
        match nft_bid_option {
            Some(nft_bid) => {
                if new_price > nft_bid.price {
                    let diff = new_price.checked_sub(nft_bid.price)
                        .map_err(|e| ContractError::Std(cosmwasm_std::StdError::Overflow { source: e }))?;
                    let sent_amount = info
                        .funds
                        .iter()
                        .find(|coin| coin.denom == "usei".to_string())
                        .map_or(Uint128::zero(), |coin| coin.amount);
                    if sent_amount < parse_decimal(diff)? {
                        return Err(ContractError::InsufficientFundsSent {  });
                    }
                } else if new_price < nft_bid.price {
                    let diff = nft_bid.price.checked_sub(new_price)
                        .map_err(|e| ContractError::Std(cosmwasm_std::StdError::Overflow { source: e }))?;
                    transfer_sei_msg = Some(BankMsg::Send {
                        to_address: info.sender.to_string(),
                        amount: coins(parse_decimal(diff)?.u128(), "usei")
                    });
                } else {
                    return Err(ContractError::NewPriceCantBeSameAsOldPrice {  });
                }
                Ok(NftBid {
                    price: new_price,
                    bidder: nft_bid.bidder,
                    nft_contract_address: nft_bid.nft_contract_address,
                    token_id: nft_bid.token_id
                })
            },
            None => Err(ContractError::NftBidNotFound {  })
        }
    })?;

    let mut response = Response::new()
        .add_attribute("action", "update_bid")
        .add_attribute("new_price", parse_decimal(nft_bid.price)?.to_string())
        .add_attribute("bidder", nft_bid.bidder)
        .add_attribute("nft_contract_address", nft_bid.nft_contract_address.clone())
        .add_attribute("token_id", nft_bid.token_id.clone());

    if let Some(transfer_sei_msg) = transfer_sei_msg {
        response = response.add_message(transfer_sei_msg);
    }

    Ok(response)
}