use cosmwasm_std::{
    Addr, AnyMsg, Api, Binary, BlockInfo, CustomMsg, CustomQuery, Empty, GrpcQuery, MsgResponse,
    Querier, Storage, SubMsgResponse,
};
use cw_multi_test::error::{bail, AnyResult};
use cw_multi_test::{
    no_init, AppBuilder, AppResponse, Contract, ContractWrapper, CosmosRouter, Executor, IntoAddr,
    Stargate,
};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgCreateDenomResponse};
use prost::Message;
use serde::de::DeserializeOwned;

const MSG_STARGATE_EXECUTE: &str = "stargate execute called";
const MSG_STARGATE_QUERY: &str = "stargate query called";
const MSG_GRPC_QUERY: &str = "grpc query called";

struct StargateKeeper;

impl Stargate for StargateKeeper {
    fn execute_stargate<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _type_url: String,
        _value: Binary,
    ) -> AnyResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        bail!(MSG_STARGATE_EXECUTE)
    }

    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _path: String,
        _data: Binary,
    ) -> AnyResult<Binary> {
        bail!(MSG_STARGATE_QUERY)
    }

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
        let message = MsgCreateDenom::decode(msg.value.as_slice())?;
        println!("type-url = {}", msg.type_url);
        println!("  sender = {}", message.sender);
        println!("subdenom = {}", message.subdenom);
        // Do something with the received message (like the chain does) and prepare the response.
        let response = MsgCreateDenomResponse {
            new_token_denom: "new-token-denom".to_string(),
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

    fn query_grpc(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _request: GrpcQuery,
    ) -> AnyResult<Binary> {
        bail!(MSG_GRPC_QUERY)
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
    app.execute_contract(sender, contract_addr.clone(), &Empty {}, &[])
        .unwrap();
}
