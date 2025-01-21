use crate::msg::{
    ResponderExecuteMessage, ResponderInstantiateMessage, ResponderQueryMessage, ResponderResponse,
};
use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Reply, ReplyOn, Response, StdResult, SubMsg, SubMsgResponse, SubMsgResult, Uint128,
};

/// Entry-point for instantiating the responder contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ResponderInstantiateMessage,
) -> StdResult<Response> {
    Ok(Response::default())
}

/// Entry-point for executing responder contract's messages.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ResponderExecuteMessage,
) -> StdResult<Response> {
    Ok(match msg {
        ResponderExecuteMessage::Add(value1, value2) => {
            let sum = value1.saturating_add(value2);
            Response::new().set_data(to_json_binary(&sum)?)
        }
        ResponderExecuteMessage::BankSend(addr, amount, denom) => {
            Response::new().add_submessage(reply_always(
                1,
                BankMsg::Send {
                    to_address: addr.clone(),
                    amount: coins(amount, denom),
                }
                .into(),
            ))
        }
        ResponderExecuteMessage::BankBurn(amount, denom) => {
            Response::new().add_submessage(reply_always(
                2,
                BankMsg::Burn {
                    amount: coins(amount, denom),
                }
                .into(),
            ))
        }
    })
}

/// Entry-point for querying the contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: ResponderQueryMessage) -> StdResult<Binary> {
    Ok(Binary::default())
}

/// Entry-point for handling submessage replies.
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
        Ok(Response::new().set_data(to_json_binary(&ResponderResponse {
            id: Some(id),
            msg_responses,
        })?))
    } else {
        Ok(Response::new().set_data(to_json_binary(&ResponderResponse {
            id: None,
            msg_responses: vec![],
        })?))
    }
}

/// Utility function for creating coins.
fn coins(amount: u128, denom: String) -> Vec<Coin> {
    vec![Coin::new(Uint128::new(amount), denom.clone())]
}

/// Utility function for creating submessages that require reply.
fn reply_always(id: u64, msg: CosmosMsg) -> SubMsg {
    SubMsg {
        id,
        payload: Default::default(),
        msg,
        gas_limit: None,
        reply_on: ReplyOn::Always,
    }
}
