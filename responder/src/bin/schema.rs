use cosmwasm_schema::write_api;
use responder::msg::{ResponderExecuteMessage, ResponderInstantiateMessage, ResponderQueryMessage};

fn main() {
    write_api! {
        instantiate: ResponderInstantiateMessage,
        execute: ResponderExecuteMessage,
        query: ResponderQueryMessage,
    }
}
