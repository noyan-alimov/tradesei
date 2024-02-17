use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{NftListing, NftBid, NftCollectionBid};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    // has to be executed with transferring nft to escrow in single transaction before this execution
    // lister pays royalties and platform fee
    List {
        price: String,
        nft_contract_address: String,
        token_id: String,
    },

    Delist {
        new_price: String,
        nft_contract_address: String,
        token_id: String,
    },

    // send more sei than displayed in nft_listing.price to this execution to cover royalties and platform fee
    BuyListing {
        nft_contract_address: String,
        token_id: String,
    },

    CancelListing {
        nft_contract_address: String,
        token_id: String,
    },


    // send sei in this execution via funds
    // seller pays royalties and platform fee
    Bid {
        price: String,
        nft_contract_address: String,
        token_id: String,
    },

    UpdateBid {
        new_price: String,
        nft_contract_address: String,
        token_id: String,
    },

    CancelBid {
        nft_contract_address: String,
        token_id: String,
    },

    // has to be executed with transferring nft to escrow in single transaction before this execution
    SellToBid {
        nft_contract_address: String,
        token_id: String,
        bidder: String,
    },


    // send funds, total_amount = sum of prices
    CollectionBid {
        prices: Vec<String>,
        nft_contract_address: String,
    },

    CancelAllCollectionBids {
        nft_contract_address: String,
    },

    CancelCollectionBid {
        nft_contract_address: String,
        price: String,
    },

    // has to be executed with transferring nft to escrow in single transaction before this execution
    SellToCollectionBid {
        nft_contract_address: String,
        token_id: String,
        bidder: String,
        price: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(GetNftListingResponse)]
    GetNftListing {
        nft_contract_address: String,
        token_id: String,
    },

    #[returns(GetNftBidResponse)]
    GetNftBid {
        nft_contract_address: String,
        token_id: String,
        bidder: String,
    },

    #[returns(GetNftCollectionBidResponse)]
    GetNftCollectionBid {
        nft_contract_address: String,
        bidder: String,
    }
}

#[cw_serde]
pub struct GetNftListingResponse {
    pub nft_listing: NftListing,
}

#[cw_serde]
pub struct GetNftBidResponse {
    pub nft_bid: NftBid,
}

#[cw_serde]
pub struct GetNftCollectionBidResponse {
    pub nft_collection_bid: NftCollectionBid,
}