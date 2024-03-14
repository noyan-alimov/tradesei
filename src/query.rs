use cosmwasm_std::{Deps, Order, StdResult};
use cw_storage_plus::Bound;

use crate::{msg::{GetNftBidResponse, GetNftCollectionBidResponse, GetNftListingResponse, GetPaginatedBidsResponse, GetPaginatedCollectionBidsResponse, GetPaginatedListingsResponse}, state::{NFT_BIDS, NFT_COLLECTION_BIDS, NFT_LISTINGS}};


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


pub fn query_paginated_listings(
    deps: Deps,
    nft_contract_address: String,
    start_after: Option<&str>,
    limit: Option<u32>,
) -> StdResult<GetPaginatedListingsResponse> {
    let start_bound = start_after.map(|id| Bound::exclusive(id));
    let listings: StdResult<Vec<_>> = NFT_LISTINGS
        .prefix(nft_contract_address.as_str())
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit.unwrap_or(10) as usize)
        .collect();

    Ok(GetPaginatedListingsResponse {
        listings: listings?,
    })
}

pub fn query_paginated_bids(
    deps: Deps,
    nft_contract_address: String,
    token_id: String,
    start_after: Option<&str>,
    limit: Option<u32>,
) -> StdResult<GetPaginatedBidsResponse> {
    let start_bound = start_after.map(|id| Bound::exclusive(id));
    let bids: StdResult<Vec<_>> = NFT_BIDS
        .prefix((nft_contract_address.as_str(), token_id.as_str()))
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit.unwrap_or(10) as usize)
        .collect();

    Ok(GetPaginatedBidsResponse {
        bids: bids?,
    })
}

pub fn query_paginated_collection_bids(
    deps: Deps,
    nft_contract_address: String,
    start_after: Option<&str>,
    limit: Option<u32>,
) -> StdResult<GetPaginatedCollectionBidsResponse> {
    let start_bound = start_after.map(|id| Bound::exclusive(id));
    let collection_bids: StdResult<Vec<_>> = NFT_COLLECTION_BIDS
        .prefix(nft_contract_address.as_str())
        .range(deps.storage, start_bound, None, Order::Ascending)
        .take(limit.unwrap_or(10) as usize)
        .collect();

    Ok(GetPaginatedCollectionBidsResponse {
        collection_bids: collection_bids?,
    })
}