use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::{NftListing, NftBid, NftCollectionBid};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    // has to be executed with transferring nft to escrow in single transaction
    List {
        price: String,
        nft_contract_address: String,
        token_id: String,
    },

    Delist {
        price: String,
        nft_contract_address: String,
        token_id: String,
    },

    BuyListing {
        nft_contract_address: String,
        token_id: String,
    },

    CancelListing {
        nft_contract_address: String,
        token_id: String,
    },

    Bid {
        price: String,
        nft_contract_address: String,
        token_id: String,
    },

    SellToBid {
        nft_contract_address: String,
        token_id: String,
    },

    CollectionBid {
        price: String,
        nft_contract_address: String,
    },

    SellToCollectionBid {
        nft_contract_address: String,
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

    #[returns(GetNftCollectionBidsResponse)]
    GetNftCollectionBids {
        nft_contract_address: String,
        skip: u16,
        limit: u16,
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
pub struct GetNftCollectionBidsResponse {
    pub nft_collection_bids: Vec<NftCollectionBid>,
}