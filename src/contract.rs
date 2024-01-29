#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, MigrateMsg};
use crate::execute::{bidding, collection_bidding, listing};
use crate::query;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tradesei";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> Result<Response, ContractError> {
    // No state migration needed, but the function must exist
    Ok(Response::new().add_attribute("method", "migrate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::List { price, nft_contract_address, token_id } => listing::list(deps, info, env, price, nft_contract_address, token_id),
        ExecuteMsg::Delist { new_price, nft_contract_address, token_id } => listing::delist(deps, info, nft_contract_address, token_id, new_price),
        ExecuteMsg::BuyListing { nft_contract_address, token_id } => listing::buy_listing(deps, info, nft_contract_address, token_id),
        ExecuteMsg::CancelListing { nft_contract_address, token_id } => listing::cancel_listing(deps, info, nft_contract_address, token_id),
        ExecuteMsg::Bid { price, nft_contract_address, token_id } => bidding::bid(deps, info, price, nft_contract_address, token_id),
        ExecuteMsg::UpdateBid { new_price, nft_contract_address, token_id } => bidding::update_bid(deps, info, nft_contract_address, token_id, new_price),
        ExecuteMsg::CancelBid { nft_contract_address, token_id } => bidding::cancel_bid(deps, info, nft_contract_address, token_id),
        ExecuteMsg::SellToBid { nft_contract_address, token_id, bidder } => bidding::sell_to_bid(deps, info, nft_contract_address, token_id, bidder),
        ExecuteMsg::CollectionBid { prices, nft_contract_address } => collection_bidding::collection_bid(deps, info, prices, nft_contract_address),
        ExecuteMsg::CancelAllCollectionBids { nft_contract_address } => collection_bidding::cancel_all_collection_bids(deps, info, nft_contract_address),
        ExecuteMsg::SellToCollectionBid { nft_contract_address, token_id, bidder, price } => collection_bidding::sell_to_collection_bid(deps, info, nft_contract_address, token_id, bidder, price),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNftListing { nft_contract_address, token_id } => to_json_binary(&query::get_nft_listing(deps, nft_contract_address, token_id)?),
        QueryMsg::GetNftBid { nft_contract_address, token_id, bidder } => to_json_binary(&query::get_nft_bid(deps, nft_contract_address, token_id, bidder)?),
        QueryMsg::GetNftCollectionBid { nft_contract_address, bidder } => to_json_binary(&query::get_nft_collection_bid(deps, nft_contract_address, bidder)?),
    }
}
