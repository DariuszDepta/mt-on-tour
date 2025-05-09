mod test_execute;
mod test_instantiate;
mod test_metadata;
mod test_query;

use cosmwasm_std::{Checksum, Empty};
use cw_multi_test::{Contract, ContractWrapper};

/// Checksum calculated from WASM binary.
///
/// ```shell
/// $ task wasm-opt
/// $ cat ./artifacts/checksums.txt
/// ```
const CHECKSUM: &str = "b675b9d5b99520cdf1424b7f80344076467ad71cf4cf59f37190562cd1ea1e01";

pub fn intro() -> Box<dyn Contract<Empty>> {
    Box::new(
        ContractWrapper::new_with_empty(
            eurimo::contract::execute,
            eurimo::contract::instantiate,
            eurimo::contract::query,
        )
        .with_checksum(Checksum::from_hex(CHECKSUM).unwrap()),
    )
}
