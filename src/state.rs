use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Map;

pub const PLATFORM_FEE_RECEIVER: &str = "sei168rypuja9p6jgu8ax377gj9ch8dcesa82gt634";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct NftListing {
    pub lister: Addr,
    pub price: Decimal,
    pub nft_contract_address: Addr,
    pub token_id: String,
}

// key: (nft contract address, nft token id)
pub const NFT_LISTINGS: Map<(&str, &str), NftListing> = Map::new("nft_listings");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct NftBid {
    pub bidder: Addr,
    pub price: Decimal,
    pub nft_contract_address: Addr,
    pub token_id: String,
}

// key: (nft contract address, nft token id, bidder)
pub const NFT_BIDS: Map<(&str, &str, &str), NftBid> = Map::new("nft_bids");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct NftCollectionBid {
    pub bidder: Addr,
    pub price: Decimal,
    pub nft_contract_address: Addr,
}

// key: nft contract address
pub const NFT_COLLECTION_BIDS: Map<String, Vec<NftCollectionBid>> = Map::new("nft_collection_bids");
