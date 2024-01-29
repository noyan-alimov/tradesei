use std::str::FromStr;

use cosmwasm_std::{coins, to_json_binary, BankMsg, Decimal, DepsMut, MessageInfo, Response, Uint128, WasmMsg};

use crate::{state::{NftCollectionBid, NFT_COLLECTION_BIDS, PLATFORM_FEE_RECEIVER}, utils::{add_transfer_sei_to_seller_msg_with_price_after_platform_fee, parse_decimal, query_check_royalties, query_royalty_info}, ContractError};


pub fn collection_bid(
    deps: DepsMut,
    info: MessageInfo,
    prices: Vec<String>,
    nft_contract_address: String,
) -> Result<Response, ContractError> {
    if prices.len() >= 100 {
        return Err(ContractError::CollectionBidsExceed100 {  })
    }

    let prices = prices.iter().map(|price| Decimal::from_str(price.as_str())
        .map_err(|_e| ContractError::InvalidPrice {  })).collect::<Result<Vec<Decimal>, ContractError>>()?;
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let nft_contract_address_clone = nft_contract_address.clone();
    let sender_clone = info.sender.clone();
    let key = (nft_contract_address_clone.as_str(), sender_clone.as_str());
    let nft_collection_bid = NFT_COLLECTION_BIDS.may_load(deps.storage, key)?;
    let nft_collection_bid = match nft_collection_bid {
        Some(mut nft_collection_bid) => {
            for price in prices.iter() {
                nft_collection_bid.bids_prices.push(price.clone());
            }
            if nft_collection_bid.bids_prices.len() >= 100 {
                return Err(ContractError::CollectionBidsExceed100 {  });
            }
            NFT_COLLECTION_BIDS.save(
                deps.storage,
                key,
                &nft_collection_bid
            )
                .map_err(|_e| ContractError::ErrorCreatingNewCollectionBid {  })?;

            nft_collection_bid
        },
        None => {
            let nft_collection_bid = NftCollectionBid {
                bidder: info.sender.clone(),
                nft_contract_address: nft_contract_address.clone(),
                bids_prices: prices,
            };

            NFT_COLLECTION_BIDS.save(
                deps.storage,
                key,
                &nft_collection_bid
            )
                .map_err(|_e| ContractError::ErrorCreatingNewCollectionBid {  })?;

            nft_collection_bid
        }
    };
    
    let sent_amount = info
        .funds
        .iter()
        .find(|coin| coin.denom == "usei".to_string())
        .map_or(Uint128::zero(), |coin| coin.amount);

    let total_amount = nft_collection_bid.bids_prices.iter().fold(Decimal::zero(), |acc, x| acc + x);
    let total_amount = parse_decimal(total_amount)?;

    // Check if the sent amount is sufficient
    if sent_amount < total_amount {
        return Err(ContractError::InsufficientFundsSent {  });
    }

    Ok(
        Response::new()
            .add_attribute("action", "collection_bid")
            .add_attribute("total_amount", total_amount.to_string())
            .add_attribute("bidder", info.sender)
            .add_attribute("nft_contract_address", nft_contract_address)
    )
}

pub fn sell_to_collection_bid(
    deps: DepsMut,
    info: MessageInfo,
    nft_contract_address: String,
    token_id: String,
    bidder: String,
    price: String,
) -> Result<Response, ContractError> {
    let price = Decimal::from_str(price.as_str())
        .map_err(|_e| ContractError::InvalidPrice {  })?;

    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let bidder = deps.api.addr_validate(bidder.as_str())
        .map_err(|_e| ContractError::InvalidCollectionBidder {  })?;

    let key = (nft_contract_address.as_str(), bidder.as_str());

    let mut nft_collection_bid = NFT_COLLECTION_BIDS.load(deps.storage, key)
        .map_err(|_e| ContractError::NftCollectionBidNotFound {  })?;

    if let Some(index) = nft_collection_bid.bids_prices.iter().position(|&x| x == price) {
        nft_collection_bid.bids_prices.remove(index);
    } else {
        return Err(ContractError::NftCollectionBidNotFound {  });
    }

    if nft_collection_bid.bids_prices.len() == 0 {
        NFT_COLLECTION_BIDS.remove(deps.storage, key);
    } else {
        NFT_COLLECTION_BIDS.save(
            deps.storage,
            key,
            &nft_collection_bid
        )
            .map_err(|_e| ContractError::ErrorUpdatingNewCollectionBid {  })?;
    }
    
    // transfer nft from escrow to bidder
    let cw721_transfer_nft_msg = cw721::Cw721ExecuteMsg::TransferNft {
        recipient: nft_collection_bid.bidder.to_string(),
        token_id: token_id.clone()
    };
    let transfer_nft_msg = WasmMsg::Execute {
        contract_addr: nft_collection_bid.nft_contract_address.to_string(),
        msg: to_json_binary(&cw721_transfer_nft_msg)?,
        funds: vec![]
    };

    // pay platform fee
    let platform_fee = price * Decimal::percent(2);
    let pay_platform_fee_msg = BankMsg::Send {
        to_address: PLATFORM_FEE_RECEIVER.to_string(),
        amount: coins(parse_decimal(platform_fee)?.u128(), "usei")
    };

    let price_after_platform_fee = price.checked_sub(platform_fee)
        .map_err(|e| ContractError::Std(cosmwasm_std::StdError::Overflow { source: e }))?;

    let mut response = Response::new()
        .add_message(transfer_nft_msg)
        .add_message(pay_platform_fee_msg)
        .add_attribute("action", "sell_to_collection_bid")
        .add_attribute("price", parse_decimal(price)?.to_string())
        .add_attribute("bidder", nft_collection_bid.bidder)
        .add_attribute("seller", info.sender.to_string())
        .add_attribute("nft_contract_address", nft_collection_bid.nft_contract_address.clone())
        .add_attribute("token_id", token_id.clone());

    // pay royalties
    let check_royalties_response_result = query_check_royalties(&deps, nft_collection_bid.nft_contract_address.to_string());
    match check_royalties_response_result {
        Ok(check_royalties_response) => {
            if check_royalties_response.royalty_payments {
                let royalty_info_response = query_royalty_info(
                    &deps,
                    nft_collection_bid.nft_contract_address.to_string(),
                    token_id.clone(),
                    parse_decimal(price)?,
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

pub fn cancel_all_collection_bids(
    deps: DepsMut,
    info: MessageInfo,
    nft_contract_address: String,
) -> Result<Response, ContractError> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())
        .map_err(|_e| ContractError::InvalidNftContractAddress {  })?;

    let key = (nft_contract_address.as_str(), info.sender.as_str());

    let nft_collection_bid = NFT_COLLECTION_BIDS.load(deps.storage, key)
        .map_err(|_e| ContractError::NftListingNotFound {  })?;

    NFT_COLLECTION_BIDS.remove(deps.storage, key);

    let total_amount = nft_collection_bid.bids_prices.iter().fold(Decimal::zero(), |acc, x| acc + x);
    let total_amount = parse_decimal(total_amount)?;

    // transfer sei from escrow back to bidder
    let transfer_sei_msg = BankMsg::Send {
        to_address: nft_collection_bid.bidder.to_string(),
        amount: coins(total_amount.u128(), "usei")
    };

    Ok(
        Response::new()
            .add_message(transfer_sei_msg)
            .add_attribute("action", "cancel_all_collection_bids")
            .add_attribute("bidder", nft_collection_bid.bidder)
            .add_attribute("nft_contract_address", nft_collection_bid.nft_contract_address.clone())
            .add_attribute("total_amount", total_amount.to_string())
    )
}