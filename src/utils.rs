use cosmwasm_schema::cw_serde;
use cosmwasm_std::{coins, to_json_binary, BankMsg, Decimal, DepsMut, QueryRequest, Response, StdResult, Uint128, WasmQuery};

use crate::ContractError;

#[cw_serde]
struct Extension<T> {
    extension: T,
}

#[cw_serde]
struct CheckRoyaltiesExtensionQueryMsg {
    msg: CheckRoyaltiesMsg,
}

#[cw_serde]
struct CheckRoyaltiesMsg {
    check_royalties: CheckRoyaltiesPayload,
}

#[cw_serde]
struct CheckRoyaltiesPayload {}

#[cw_serde]
pub struct CheckRoyaltiesResponse {
    pub royalty_payments: bool,
}

pub fn query_check_royalties(deps: &DepsMut, nft_contract_address: String) -> StdResult<CheckRoyaltiesResponse> {
    let query_msg = Extension {
        extension: CheckRoyaltiesExtensionQueryMsg {
            msg: CheckRoyaltiesMsg {
                check_royalties: CheckRoyaltiesPayload {},
            },
        }
    };

    let wasm_query = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_contract_address,
        msg: to_json_binary(&query_msg)?,
    });

    deps.querier.query(&wasm_query)
}


#[cw_serde]
struct RoyaltyInfoExtensionQueryMsg {
    msg: RoyaltyInfoMsg,
}

#[cw_serde]
struct RoyaltyInfoMsg {
    royalty_info: RoyaltyInfoPayload,
}

#[cw_serde]
struct RoyaltyInfoPayload {
    token_id: String,
    sale_price: Uint128,
}

#[cw_serde]
pub struct RoyaltyInfoResponse {
    pub address: String,
    pub royalty_amount: Uint128,
}

pub fn query_royalty_info(deps: &DepsMut, nft_contract_address: String, token_id: String, sale_price: Uint128) -> StdResult<RoyaltyInfoResponse> {
    let query_msg = Extension {
        extension: RoyaltyInfoExtensionQueryMsg {
            msg: RoyaltyInfoMsg {
                royalty_info: RoyaltyInfoPayload {
                    token_id,
                    sale_price,
                },
            },
        }
    };

    let wasm_query = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: nft_contract_address,
        msg: to_json_binary(&query_msg)?,
    });

    deps.querier.query(&wasm_query)
}


pub fn add_transfer_sei_to_seller_msg_with_price_after_platform_fee(
    seller: String,
    price_after_platform_fee: u128,
    response: Response,
) -> Response {
    let transfer_sei_msg = BankMsg::Send {
        to_address: seller,
        amount: coins(price_after_platform_fee, "usei")
    };
    response.clone().add_message(transfer_sei_msg)
}

// need to remove extra zeros because
// 1 sei = 1_000_000 usei
// 1 sei = 1_000_000_000_000_000_000 atomics in Decimal
// so removing extra 1_000_000_000_000
pub fn parse_decimal(decimal: Decimal) -> Result<Uint128, ContractError> {
    decimal
        .atomics()
        .checked_div(Uint128::new(1_000_000_000_000))
        .map_err(|_e| ContractError::ParseDecimal {  })
}
