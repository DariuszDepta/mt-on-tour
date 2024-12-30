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

#[test]
fn instantiate2_algorithm_specification_should_work() {
    let checksum = &[
        214, 99, 99, 107, 134, 192, 157, 214, 178, 212, 49, 132, 110, 96, 13, 98, 161, 227, 237,
        164, 100, 25, 24, 200, 200, 226, 97, 251, 38, 0, 215, 76,
    ];
    let creator = &[
        55, 93, 230, 45, 239, 230, 164, 101, 174, 136, 148, 6, 41, 28, 203, 147, 100, 226, 183, 155,
    ];
    let salt = &[1, 2, 3];
    let msg = &[];
    assert_eq!(
        "cosmwasm1l6ug8zcdvejlas0st7qyvz5lv5vm5ppqdzx0mjsk0cgyc6ktsa6sfzyeww",
        MockApi::default()
            .addr_humanize(&instantiate2(checksum, creator, salt, msg))
            .unwrap()
            .as_str()
    );
}

/// Equivalent of the `wasmd` implementation of the algorithm
/// for address generation during contract instantiation.
fn instantiate(code_id: u64, instance_id: u64) -> CanonicalAddr {
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

    Sha256::new()
        .chain(Sha256::digest(b"module"))
        .chain(b"wasm\0")
        .chain(code_id.to_be_bytes())
        .chain(instance_id.to_be_bytes())
        .finalize()
        .to_vec()
        .into()
}

/// Equivalent of the `wasmd` implementation of the algorithm for address generation
/// during contract instantiation with predictable addresses.
fn instantiate2(checksum: &[u8], creator: &[u8], salt: &[u8], msg: &[u8]) -> CanonicalAddr {
    use sha2::digest::Update;
    use sha2::{Digest, Sha256};

    Sha256::new()
        .chain(Sha256::digest(b"module"))
        .chain(b"wasm\0")
        .chain((checksum.len() as u64).to_be_bytes())
        .chain(checksum)
        .chain((creator.len() as u64).to_be_bytes())
        .chain(creator)
        .chain((salt.len() as u64).to_be_bytes())
        .chain(salt)
        .chain((msg.len() as u64).to_be_bytes())
        .chain(msg)
        .finalize()
        .to_vec()
        .into()
}
