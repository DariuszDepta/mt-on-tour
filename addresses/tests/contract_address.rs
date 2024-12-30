use cosmwasm_std::testing::MockApi;
use cosmwasm_std::{Api, CanonicalAddr};

#[test]
fn instantiate_algorithm_specification_should_work() {
    assert_eq!(
        "cosmwasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s8jef58",
        MockApi::default()
            .addr_humanize(&instantiate(1, 1))
            .unwrap()
            .as_str()
    );
}

fn instantiate(code_id: i32, instance_id: i32) -> CanonicalAddr {
    use cosmwasm_std::CanonicalAddr;
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

    let canonical_address: CanonicalAddr = Sha256::new()
        .chain(Sha256::digest(b"module"))
        .chain(b"wasm\0")
        .chain(&(code_id as u64).to_be_bytes())
        .chain(&(instance_id as u64).to_be_bytes())
        .finalize()
        .to_vec()
        .into();

    canonical_address
}
