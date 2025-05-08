use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use crate::msg::MorphExecuteMessage;
use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};

#[cw_serde]
struct MorphedAppius {
    name: String,
    height: u64,
    age: u8,
}

#[cw_serde]
struct MorphedMarcus {
    name: String,
    age: u8,
}

#[cw_serde]
struct MorphedTiberia {
    name: String,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: MorphExecuteMessage,
) -> StdResult<Response> {
    let data = match msg {
        MorphExecuteMessage::Appius => to_json_binary(&MorphedAppius {
            name: "Appius".to_string(),
            height: 192,
            age: 38,
        })?,
        MorphExecuteMessage::Marcus => to_json_binary(&MorphedMarcus {
            name: "Marcus".to_string(),
            age: 24,
        })?,
        MorphExecuteMessage::Tiberia => to_json_binary(&MorphedTiberia {
            name: "Tiberia".to_string(),
        })?,
        MorphExecuteMessage::Smileus => Binary::from(r#"{"smile":"😀"}"#.to_string().as_bytes()),
    };
    Ok(Response::new().set_data(data))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    to_json_binary(&Empty {})
}
