use cosmwasm_schema::cw_serde;
use cosmwasm_std::{QueryRequest, StdResult, WasmQuery, to_json_binary, DepsMut, Uint128};

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
