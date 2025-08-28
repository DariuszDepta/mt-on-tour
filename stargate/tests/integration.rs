use cosmwasm_std::{
    from_json, Addr, AnyMsg, Api, Binary, BlockInfo, CustomMsg, CustomQuery, Empty, MsgResponse,
    Storage, SubMsgResponse,
};
use cw_multi_test::error::AnyResult;
use cw_multi_test::{
    no_init, AppBuilder, AppResponse, Contract, ContractWrapper, CosmosRouter, Executor, IntoAddr,
    Stargate,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgCreateDenomResponse};
use prost::Message;
use serde::de::DeserializeOwned;

/// Custom implementation for Stargate module
/// used to simulate handling Any messages by chain.
struct StargateKeeper;

impl Stargate for StargateKeeper {
    /// Example of handler for all `Any` messages.
    fn execute_any<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        msg: AnyMsg,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        // Decode the received message.
        let message = MsgCreateDenom::decode(msg.value.as_slice())?;
        // Do something with the received message (exactly like your chain does).
        let response = MsgCreateDenomResponse {
            // I did here something dummy just to make this test work.
            new_token_denom: format!("{}-token-denom", message.subdenom),
        };
        let msg_response = MsgResponse {
            type_url: MsgCreateDenomResponse::TYPE_URL.to_string(),
            value: Binary::from(response.encode_to_vec()),
        };
        let sub_response = SubMsgResponse {
            events: vec![],
            #[allow(deprecated)]
            data: None,
            msg_responses: vec![msg_response],
        };
        Ok(sub_response.into())
    }
}

fn the_contract() -> Box<dyn Contract<Empty>> {
    Box::new(
        ContractWrapper::new_with_empty(
            mtot_stargate::contract::execute,
            mtot_stargate::contract::instantiate,
            mtot_stargate::contract::query,
        )
        .with_reply(mtot_stargate::contract::reply),
    )
}

#[test]
fn execute_should_work() {
    // Build the chain simulator with custom stargate keeper.
    let mut app = AppBuilder::default()
        .with_stargate(StargateKeeper)
        .build(no_init);

    let code_id = app.store_code(the_contract());

    let sender = "sender".into_addr();

    let contract_addr = app
        .instantiate_contract(
            code_id,
            sender.clone(),
            &Empty {},
            &[],
            "the-contract",
            None,
        )
        .unwrap();

    // Execute empty message to trigger processing Any message.
    // The `execute` entrypoint in contract returns MsgCreateDenom as submessage to be processed
    // by chain, so in this test it will be processed by custom Stargate module (i.e. StargateKeeper).
    // The result from processing MsgCreateDenom message is MsgCreateDenomResponse message passed
    // to the `reply` entrypoint in the contract. The content of MsgCreateDenomResponse is simply unpacked there.
    let app_response = app
        .execute_contract(sender, contract_addr.clone(), &Empty {}, &[])
        .unwrap();

    // Check the result from `reply` entrypoint, which should be the unpacked content of MsgCreateDenomResponse message.
    let response = from_json::<String>(app_response.data.unwrap()).unwrap();
    assert_eq!("pao-token-denom", response);
}
