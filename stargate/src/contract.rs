#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, AnyMsg, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Reply,
    Response, StdResult, SubMsg, SubMsgResponse, SubMsgResult,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgCreateDenomResponse};
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
        subdenom: "pao".into(),
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
    #[allow(deprecated)]
    if let Reply {
        id,
        result:
            SubMsgResult::Ok(SubMsgResponse {
                events: _,
                data: _,
                msg_responses,
            }),
        ..
    } = msg
    {
        if id == 1
            && msg_responses.len() == 1
            && msg_responses[0].type_url == MsgCreateDenomResponse::TYPE_URL
        {
            if let Ok(response) = MsgCreateDenomResponse::decode(msg_responses[0].value.as_slice())
            {
                return Ok(Response::new().set_data(to_json_binary(&response.new_token_denom)?));
            }
        }
    }
    Ok(Response::default())
}
