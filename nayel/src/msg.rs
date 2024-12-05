use cosmwasm_schema::cw_serde;
use cosmwasm_std::CustomMsg;

#[derive(Default)]
#[cw_serde]
pub struct NayelResponse {
    pub code_creator_address: Option<String>,
    pub code_id: Option<u64>,
    pub contract_creator_address: Option<String>,
    pub contract_address: Option<String>,
    pub err_contract_info: Option<String>,
    pub err_code_info: Option<String>,
    pub err_self_query: Option<String>,
}

impl CustomMsg for NayelResponse {}
