use base64::engine::general_purpose;
use base64::Engine;
use bech32::{encode, Bech32, Hrp};
use cw_multi_test::{no_init, AppBuilder, MockApiBech32};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[test]
fn generating_wasm_user_address_should_work() {
    // Public key encoded in Base64 as displayed by the command:
    // wasmd keys list --keyring-backend=test
    let pub_key_base64 = "AqPbh8vz1AvgyvfkJkkjMA76PoIp0r5EEcvdUSYvqmOY";

    // Bech32 prefix to be used in address generation.
    let prefix = "wasm";

    // Decode the Base64 public key into bytes.
    let pub_key_bytes = general_purpose::STANDARD.decode(pub_key_base64).unwrap();

    // Calculate SHA-256 hash of the public key.
    let sha256_hash = Sha256::digest(&pub_key_bytes);
    assert_eq!(
        "89b3d93dd4854c892ba6047457726fa75ce760f900b29483f217f5fae01601ea",
        format!("{:x}", sha256_hash)
    );

    // Calculate RIPEMD-160 hash of the SHA-256 output.
    let ripemd160_hash = Ripemd160::digest(sha256_hash);
    assert_eq!(
        "18b33e31c709a693e02e2c8c434787816685d194",
        format!("{:x}", ripemd160_hash)
    );

    // Convert to Bech32 address.
    let hrp = Hrp::parse(prefix).unwrap();
    let user_address = encode::<Bech32>(hrp, ripemd160_hash.as_slice()).unwrap();

    // Make sure the generated address is equal to the expected one.
    assert_eq!("wasm1rzenuvw8pxnf8cpw9jxyx3u8s9ngt5v5870gnz", user_address);
}

fn pub_key_to_bech32(pub_key: &str, prefix: &str) -> String {
    println!("Pub key (Base64) = {}", pub_key);
    let pub_key_bytes = general_purpose::STANDARD.decode(pub_key).unwrap();
    println!(
        "   Pub key (hex) = {}",
        pub_key_bytes.iter().fold("".to_string(), |mut acc, s| {
            let _ = write!(&mut acc, "{}", s);
            acc
        })
    );
    let sha256_hash = Sha256::digest(&pub_key_bytes);
    println!("   SHA-256 (hex) = {:x}", sha256_hash);
    let ripemd160_hash = Ripemd160::digest(sha256_hash);
    println!("RIPEMD-160 (hex) = {:x}", ripemd160_hash);
    let addr = encode::<Bech32>(Hrp::parse(prefix).unwrap(), ripemd160_hash.as_slice()).unwrap();
    println!("          Bech32 = {}\n", addr);
    addr
}

#[test]
fn generating_cosmos_user_address_should_work() {
    assert_eq!(
        "cosmos10w323vd8hm8adp3kus6qp99yrs86mj4kh2ruhg",
        pub_key_to_bech32("AqAWno6jFXEqE3MC5v9BObuBkHIyrRORXDtNReuPHo/s", "cosmos")
    );
    assert_eq!(
        "cosmos1k5auppj33lptp6dlhauqnmqxnw6h8hd06j2j2m",
        pub_key_to_bech32("AoB7xZiSlrra2IxWhFR3iLR2+Ksa49MefTk8GfYesp95", "cosmos")
    );
}

#[test]
fn generating_multitest_addresses_should_work() {
    let user_alias = "alice";
    let prefix = "cosmos";
    let app = AppBuilder::default()
        .with_api(MockApiBech32::new(prefix))
        .build(no_init);
    let addr = app.api().addr_make(user_alias);
    assert_eq!(
        "cosmos190vqdjtlpcq27xslcveglfmr4ynfwg7gmw86cnun4acakxrdd6gq7g6jwn",
        addr.as_str()
    );

    let sha256_hash = Sha256::digest(user_alias);
    println!("   SHA-256 (hex) = {:x}", sha256_hash);
    let addr1 = encode::<Bech32>(Hrp::parse(prefix).unwrap(), sha256_hash.as_slice()).unwrap();
    assert_eq!(addr1.as_str(), addr.as_str());
}
