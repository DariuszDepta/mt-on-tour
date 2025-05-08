mod test_execute;

use cosmwasm_std::{Checksum, Empty};
use cw_multi_test::{Contract, ContractWrapper};

/// Checksum calculated from `morph.wasm` binary.
///
/// ```shell
/// $ cargo build --release --lib --target wasm32-unknown-unknown
/// $ sha256sum ../target/wasm32-unknown-unknown/release/morph.wasm | awk '{print toupper($1)}'
/// ```
const CHECKSUM: &str = "757EDD8B310480C81A2B0E79D3E9555070C1A4544BEACA920E7AD0DA1136951C";

pub fn intro() -> Box<dyn Contract<Empty>> {
    Box::new(
        ContractWrapper::new_with_empty(
            morph::contract::execute,
            morph::contract::instantiate,
            morph::contract::query,
        )
        .with_checksum(Checksum::from_hex(CHECKSUM).unwrap()),
    )
}
