use cosmwasm_std::{Deps, StdResult};

use crate::{msg::GetNftListingResponse, state::NFT_LISTINGS};


pub fn get_nft_listing(deps: Deps, nft_contract_address: String, token_id: String) -> StdResult<GetNftListingResponse> {
    let nft_contract_address = deps.api.addr_validate(nft_contract_address.as_str())?;
    let key = (nft_contract_address.as_str(), token_id.as_str());
    let nft_listing = NFT_LISTINGS.load(deps.storage, key)?;
    Ok(GetNftListingResponse{ nft_listing })
}