#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::NayelResponse;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response<NayelResponse>> {
    let mut response = NayelResponse::default();
    let contract_addr = env.contract.address;
    response.contract_address = Some(contract_addr.to_string());
    match deps.querier.query_wasm_contract_info(contract_addr.clone()) {
        Ok(cir) => {
            response.code_id = Some(cir.code_id);
            response.contract_creator_address = Some(cir.creator.to_string());
        }
        Err(reason) => response.err_contract_info = Some(reason.to_string()),
    }
    match deps.querier.query_wasm_code_info(1) {
        Ok(cir) => {
            response.code_creator_address = Some(cir.creator.to_string());
        }
        Err(reason) => response.err_code_info = Some(reason.to_string()),
    }
    match deps
        .querier
        .query_wasm_smart::<NayelResponse>(contract_addr, &Empty {})
    {
        Ok(_) => {}
        Err(reason) => response.err_self_query = Some(reason.to_string()),
    }
    Ok(Response::new().set_data(to_json_binary(&response)?))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    to_json_binary(&NayelResponse::default())
}
