#[test]
fn with_block_should_work() {
    use cosmwasm_std::{BlockInfo, Timestamp};
    use cw_multi_test::{no_init, AppBuilder};

    // create the chain builder
    let builder = AppBuilder::default();

    // prepare the custom block
    let block = BlockInfo {
        height: 1,
        time: Timestamp::from_seconds(1723627489),
        chain_id: "starship-testnet".to_string(),
    };

    // build the chain initialized with the custom block
    let app = builder.with_block(block).build(no_init);

    // get the current block properties
    let block = app.block_info();

    // now the block height is 21
    assert_eq!(1, block.height);

    // now the block timestamp is Wed Aug 14 2024 09:24:49 GMT+0000
    assert_eq!(1723627489, block.time.seconds());

    // now the chain identifier is "starship-testnet"
    assert_eq!("starship-testnet", block.chain_id);
}
