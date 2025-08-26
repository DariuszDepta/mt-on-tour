#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    AnyMsg, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Reply, Response, StdResult,
    SubMsg,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
use prost::Message;

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
pub fn execute(_deps: DepsMut, _env: Env, info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    let create_denom_msg = MsgCreateDenom {
        sender: info.sender.into(),
        subdenom: "my-sub-denom".into(),
    };
    let msg = CosmosMsg::Any(AnyMsg {
        type_url: MsgCreateDenom::TYPE_URL.to_string(),
        value: Binary::from(create_denom_msg.encode_to_vec()),
    });
    Ok(Response::new().add_submessage(SubMsg::reply_always(msg, 1)))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    Ok(Binary::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    println!("REPLY_ID = {}", msg.id);
    println!("PAYLOAD = {:?}", msg.payload);
    println!("RESULT = {:?}", msg.result);
    Ok(Response::default())
}
