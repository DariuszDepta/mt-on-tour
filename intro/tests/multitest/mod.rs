mod test_execute;
mod test_instantiate;
mod test_query;
mod test_metadata;

use cosmwasm_std::{Checksum, Empty};
use cw_multi_test::{Contract, ContractWrapper};

/// Checksum calculated from `intro.wasm` binary.
///
/// ```shell
/// sha256sum ./target/wasm32-unknown-unknown/release/intro.wasm | awk '{print toupper($1)}'
/// ```
const CHECKSUM: &str = "E3523348FF69519F913FBB12DC5660161ECB2978E603E909925DD87ABCC6A229";

pub fn intro() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new_with_empty(
        intro::contract::execute,
        intro::contract::instantiate,
        intro::contract::query,
    ).with_checksum(Checksum::from_hex(CHECKSUM).unwrap()))
}
