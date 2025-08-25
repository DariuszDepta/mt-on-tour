use cosmwasm_std::{
    Addr, AnyMsg, Api, Binary, BlockInfo, CosmosMsg, CustomMsg, CustomQuery, Empty, GrpcQuery,
    Querier, QueryRequest, StdError, StdResult, Storage,
};
use cw_multi_test::{no_init, AppBuilder, AppResponse, CosmosRouter, Executor, Stargate};
use serde::de::DeserializeOwned;

const MSG_STARGATE_EXECUTE: &str = "stargate execute called";
const MSG_STARGATE_QUERY: &str = "stargate query called";
const MSG_ANY_EXECUTE: &str = "any execute called";
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
    ) -> StdResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        Err(StdError::msg(MSG_STARGATE_EXECUTE))
    }

    fn query_stargate(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _path: String,
        _data: Binary,
    ) -> StdResult<Binary> {
        Err(StdError::msg(MSG_STARGATE_QUERY))
    }

    fn execute_any<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _msg: AnyMsg,
    ) -> StdResult<AppResponse>
    where
        ExecC: CustomMsg + DeserializeOwned + 'static,
        QueryC: CustomQuery + DeserializeOwned + 'static,
    {
        Err(StdError::msg(MSG_ANY_EXECUTE))
    }

    fn query_grpc(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        _request: GrpcQuery,
    ) -> StdResult<Binary> {
        Err(StdError::msg(MSG_GRPC_QUERY))
    }
}

#[test]
fn building_app_with_custom_any_grpc_should_work() {
    // Build the chain simulator with custom stargate keeper.
    let mut app = AppBuilder::default()
        .with_stargate(StargateKeeper)
        .build(no_init);

    // Prepare user addresses.
    let sender_addr = app.api().addr_make("sender");

    // Executing `any` message should return an error defined in custom stargate keeper.
    let msg = CosmosMsg::Any(AnyMsg {
        type_url: "test".to_string(),
        value: Default::default(),
    });
    assert_eq!(
        format!("kind: Other, error: {MSG_ANY_EXECUTE}"),
        app.execute(sender_addr, msg).unwrap_err().to_string(),
    );

    // Executing `grpc` query should return an error defined in custom stargate keeper.
    let request: QueryRequest<Empty> = QueryRequest::Grpc(GrpcQuery {
        path: "test".to_string(),
        data: Default::default(),
    });
    assert!(app
        .wrap()
        .query::<Empty>(&request)
        .unwrap_err()
        .to_string()
        .ends_with(MSG_GRPC_QUERY));
}
