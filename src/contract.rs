#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::execute::listing;
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
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::List { price, nft_contract_address, token_id } => listing::list(deps, info, price, nft_contract_address, token_id),
        ExecuteMsg::Delist { price, nft_contract_address, token_id } => listing::delist(deps, info, price, nft_contract_address, token_id),
        ExecuteMsg::BuyListing { nft_contract_address, token_id } => listing::buy_listing(deps, info, nft_contract_address, token_id),
        ExecuteMsg::CancelListing { nft_contract_address, token_id } => listing::cancel_listing(deps, info, nft_contract_address, token_id),
        _ => unimplemented!()
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetNftListing { nft_contract_address, token_id } => to_json_binary(&query::get_nft_listing(deps, nft_contract_address, token_id)?),
        _ => unimplemented!()
    }
}
