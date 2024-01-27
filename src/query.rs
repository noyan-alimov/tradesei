use cosmwasm_std::{Deps, StdResult};

use crate::{msg::{GetNftBidResponse, GetNftCollectionBidResponse, GetNftListingResponse}, state::{NFT_BIDS, NFT_COLLECTION_BIDS, NFT_LISTINGS}};


pub fn get_nft_listing(deps: Deps, nft_contract_address: String, token_id: String) -> StdResult<GetNftListingResponse> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())?;
    let key = (nft_contract_address.as_str(), token_id.as_str());
    let nft_listing = NFT_LISTINGS.load(deps.storage, key)?;
    Ok(GetNftListingResponse{ nft_listing })
}

pub fn get_nft_bid(deps: Deps, nft_contract_address: String, token_id: String, bidder: String) -> StdResult<GetNftBidResponse> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())?;
    let bidder = deps.api.addr_validate(bidder.as_str())?;
    let key = (nft_contract_address.as_str(), token_id.as_str(), bidder.as_str());
    let nft_bid = NFT_BIDS.load(deps.storage, key)?;
    Ok(GetNftBidResponse{ nft_bid })
}

pub fn get_nft_collection_bid(deps: Deps, nft_contract_address: String, bidder: String) -> StdResult<GetNftCollectionBidResponse> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())?;
    let bidder = deps.api.addr_validate(bidder.as_str())?;
    let key = (nft_contract_address.as_str(), bidder.as_str());
    let nft_collection_bid = NFT_COLLECTION_BIDS.load(deps.storage, key)?;
    Ok(GetNftCollectionBidResponse{ nft_collection_bid })
}