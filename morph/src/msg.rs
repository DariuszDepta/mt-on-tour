use cosmwasm_schema::cw_serde;

/// Messages executed by the contract.
#[cw_serde]
pub enum MorphExecuteMessage {
    Appius,
    Marcus,
    Tiberia,
    Smileus,
}
