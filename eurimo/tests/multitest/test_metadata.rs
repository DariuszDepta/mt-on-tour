use super::*;
use cosmwasm_std::Empty;
use cw_multi_test::{no_init, AppBuilder, Executor, MockApiBech32};

#[test]
fn querying_metadata_should_work() {
    // Prepare the chain.
    let mut app = AppBuilder::new()
        .with_api(MockApiBech32::new("wasm"))
        .build(no_init);

    // Prepare the address of the contract's code creator.
    let creator_addr = app.api().addr_make("creator");

    // Store the contract's code on the chain.
    let code_id = app.store_code_with_creator(creator_addr.clone(), intro());

    // Prepare the address of the contract's owner.
    let owner_addr = app.api().addr_make("owner");

    // Invoke the `instantiate` entry-point of the contract.
    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner_addr.clone(),
            &Empty {},
            &[],
            "my-intro-1",
            None,
        )
        .unwrap();

    // Contract is now instantiated, check if the contract address is valid.
    assert!(contract_addr.as_str().starts_with("wasm1"));

    // Having a contract address, retrieve the contract's metadata.
    // This is the equivalent of calling:
    //
    // $ wasmd q wasm contract wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d
    //
    let contract_info_response = app.wrap().query_wasm_contract_info(contract_addr).unwrap();
    assert_eq!(1, contract_info_response.code_id);
    assert_eq!(None, contract_info_response.admin);
    assert_eq!(None, contract_info_response.ibc_port);
    assert!(!contract_info_response.pinned);
    assert_eq!(owner_addr.as_str(), contract_info_response.creator.as_str());

    // Having the code identifier, retrieve the source code metadata.
    // This is the equivalent of calling:
    //
    // $ wasmd query wasm code-info 1
    //
    let code_info_response = app
        .wrap()
        .query_wasm_code_info(contract_info_response.code_id)
        .unwrap();
    assert_eq!(1, code_info_response.code_id);
    assert_eq!(creator_addr.as_str(), code_info_response.creator.as_str());
    assert_eq!(
        CHECKSUM,
        code_info_response.checksum.to_string().to_uppercase()
    );
}
