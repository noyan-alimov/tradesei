use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Map;

pub const PLATFORM_FEE_RECEIVER: &str = "sei1904lsj3d3rtm903dqt4ljxx0dpugtmngf4tgal";

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
    pub nft_contract_address: Addr,
    pub bids_prices: Vec<Decimal>,
}

// key: (nft contract address, bidder)
pub const NFT_COLLECTION_BIDS: Map<(&str, &str), NftCollectionBid> = Map::new("nft_collection_bids");
