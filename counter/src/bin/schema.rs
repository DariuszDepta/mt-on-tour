use cosmwasm_schema::write_api;

use mtot_counter::msg::{CounterExecMsg, CounterInitMsg, CounterQueryMsg};

fn main() {
    write_api! {
        instantiate: CounterInitMsg,
        execute: CounterExecMsg,
        query: CounterQueryMsg,
    }
}
