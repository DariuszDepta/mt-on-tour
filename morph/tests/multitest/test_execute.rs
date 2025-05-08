use crate::multitest::intro;
use cosmwasm_std::{Binary, Empty, HexBinary};
use cw_multi_test::{App, Executor};
use morph::msg::MorphExecuteMessage;

fn eq_binary(expected: &str, actual: Binary) {
    assert_eq!(
        Binary::new(HexBinary::from_hex(expected).unwrap().into()),
        actual
    );
}

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
    assert!(response.data.is_some());
    // Check the binary content.
    eq_binary(
        "7b226e616d65223a22417070697573222c22686569676874223a3139322c22616765223a33387d",
        response.data.unwrap(),
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
    assert!(response.data.is_some());
    // Check the binary content.
    eq_binary(
        "7b226e616d65223a224d6172637573222c22616765223a32347d",
        response.data.unwrap(),
    );

    // Invoke the `execute` entry-point of the contract with `MorphExecuteMessage::Tiberia`.
    let response = app
        .execute_contract(
            sender_addr,
            contract_addr,
            &MorphExecuteMessage::Tiberia {},
            &[],
        )
        .unwrap();

    // Data returned from the `execute` entry-point must not be empty.
    assert!(response.data.is_some());
    // Check the binary content.
    eq_binary(
        "7b226e616d65223a2254696265726961227d",
        response.data.unwrap(),
    );
}
