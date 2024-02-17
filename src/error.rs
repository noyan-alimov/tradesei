use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Error parsing decimal to u128")]
    ParseDecimal {},

    #[error("Invalid price")]
    InvalidPrice {},

    #[error("Invalid NFT contract address")]
    InvalidNftContractAddress {},

    #[error("Error creating new listing")]
    ErrorCreatingNewListing {},

    #[error("NFT listing not found")]
    NftListingNotFound {},

    #[error("Error creating new bid")]
    ErrorCreatingNewBid {},

    #[error("Invalid bidder")]
    InvalidBidder {},

    #[error("NFT bid not found")]
    NftBidNotFound {},

    #[error("NFT not in escrow")]
    NftNotInEscrow {},

    #[error("Insufficient funds sent")]
    InsufficientFundsSent {},

    #[error("New price can't be the same as old price")]
    NewPriceCantBeSameAsOldPrice {},

    #[error("Error creating new collection bid")]
    ErrorCreatingNewCollectionBid {},

    #[error("Collection bids exceed 100")]
    CollectionBidsExceed100 {},

    #[error("Invalid collection bidder")]
    InvalidCollectionBidder {},

    #[error("NFT collection bid not found")]
    NftCollectionBidNotFound {},

    #[error("NFT collection bid price not found")]
    NftCollectionBidPriceNotFound {},

    #[error("Error updating collection bid")]
    ErrorUpdatingCollectionBid {},

    #[error("Listing already exists")]
    ListingAlreadyExists {},

    #[error("Bid already exists")]
    BidAlreadyExists {},
}
