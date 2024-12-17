use base64::engine::general_purpose;
use base64::Engine;
use bech32::{encode, Bech32, Hrp};
use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

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

    // Calculate RIPEMD-160 hash of the SHA-256 output.
    let ripemd160_hash = Ripemd160::digest(&sha256_hash);

    // Convert to Bech32 address.
    let hrp = Hrp::parse(prefix).unwrap();
    let user_address = encode::<Bech32>(hrp, ripemd160_hash.as_slice()).unwrap();

    // Make sure the generated address is equal to the expected one.
    assert_eq!("wasm1rzenuvw8pxnf8cpw9jxyx3u8s9ngt5v5870gnz", user_address);
}
