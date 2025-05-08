use crate::multitest::intro;
use cosmwasm_std::Empty;
use cw_multi_test::{App, Executor};
use morph::msg::MorphExecuteMessage;

#[test]
fn execute_should_work() {
    // Prepare the chain simulator.
    let mut app = App::default();

    // Prepare the address of the contract creator.
    let creator_addr = app.api().addr_make("creator");

    // Store the contract's code on the chain.
    let code_id = app.store_code_with_creator(creator_addr, intro());

    // Prepare the address of the contract owner.
    let owner_addr = app.api().addr_make("owner");

    // Create an instance of the contract and save its address.
    let contract_addr = app
        .instantiate_contract(
            code_id,
            owner_addr,
            &Empty {},
            &[],
            "introductory-contract",
            None,
        )
        .unwrap();

    // Prepare address of the execute message sender.
    let sender_addr = app.api().addr_make("sender");

    // Invoke the `execute` entry-point of the contract with `MorphExecuteMessage::Appius`.
    let response = app
        .execute_contract(
            sender_addr.clone(),
            contract_addr.clone(),
            &MorphExecuteMessage::Appius {},
            &[],
        )
        .unwrap();

    // Data returned from the `execute` entry-point must not be empty.
    assert_eq!(
        response.data,
        Some(br#"{"name":"Appius","height":192,"age":38}"#.into())
    );

    // Invoke the `execute` entry-point of the contract with `MorphExecuteMessage::Marcus`.
    let response = app
        .execute_contract(
            sender_addr.clone(),
            contract_addr.clone(),
            &MorphExecuteMessage::Marcus {},
            &[],
        )
        .unwrap();

    // Data returned from the `execute` entry-point must not be empty.
    assert_eq!(response.data, Some(br#"{"name":"Marcus","age":24}"#.into()));

    // Invoke the `execute` entry-point of the contract with `MorphExecuteMessage::Tiberia`.
    let response = app
        .execute_contract(
            sender_addr.clone(),
            contract_addr.clone(),
            &MorphExecuteMessage::Tiberia {},
            &[],
        )
        .unwrap();

    // Data returned from the `execute` entry-point must not be empty.
    assert_eq!(response.data, Some(br#"{"name":"Tiberia"}"#.into()));

    // Invoke the `execute` entry-point of the contract with `MorphExecuteMessage::Smileus`.
    let response = app
        .execute_contract(
            sender_addr,
            contract_addr,
            &MorphExecuteMessage::Smileus {},
            &[],
        )
        .unwrap();

    // Data returned from the `execute` entry-point must not be empty.
    assert_eq!(response.data, Some("😀".to_string().as_bytes().into()));
}
