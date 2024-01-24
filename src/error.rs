use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid price")]
    InvalidPrice {},

    #[error("Invalid NFT contract address")]
    InvalidNftContractAddress {},

    #[error("Error creating new listing")]
    ErrorCreatingNewListing {},

    #[error("NFT listing not found")]
    NftListingNotFound {},
}
