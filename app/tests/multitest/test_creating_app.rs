#[test]
fn app_default_should_work() {
    use cw_multi_test::App;

    let app = App::default();

    // just make sure that the default settings are in use
    let block = app.block_info();
    assert_eq!(12345, block.height);
    assert_eq!(1571797419, block.time.seconds());
    assert_eq!("cosmos-testnet-14002", block.chain_id);
}

#[test]
fn app_new_no_init_should_work() {
    use cw_multi_test::{no_init, App};

    let app = App::new(no_init);

    // just make sure that the default settings are in use
    let block = app.block_info();
    assert_eq!(12345, block.height);
    assert_eq!(1571797419, block.time.seconds());
    assert_eq!("cosmos-testnet-14002", block.chain_id);
}
