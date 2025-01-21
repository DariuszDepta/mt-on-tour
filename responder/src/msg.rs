use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::MsgResponse;

/// Instantiating messages of the responder contract.
#[cw_serde]
pub enum ResponderInstantiateMessage {
    None,
}

/// Messages executed on the responder contract.
#[cw_serde]
pub enum ResponderExecuteMessage {
    /// Adds two unsigned integers and returns the sum.
    Add(u64, u64),
    /// Returns submessage BankMsg::Send.
    BankSend(String, u128, String),
    /// Returns submessage BankMsg::Burn.
    BankBurn(u128, String),
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum ResponderQueryMessage {
    #[returns(ResponderResponse)]
    Value,
}

/// Utility structure for convenient data transfer
/// from reply entry-point back to caller.
#[cw_serde]
pub struct ResponderResponse {
    pub id: Option<u64>,
    pub msg_responses: Vec<MsgResponse>,
}
