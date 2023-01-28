use secp256k1::{ecdsa::{RecoveryId, RecoverableSignature}, Message, PublicKey};
use sha3::{Digest, Keccak256};

pub fn get_address_from_public_key(pubkey: &PublicKey) -> Vec<u8> {
    let pubkey_bytes = &pubkey.serialize_uncompressed()[1..];
    let mut hasher = Keccak256::new();
    hasher.update(pubkey_bytes);
    hasher.finalize().to_vec()
}

pub fn get_sign_message(nonce: String) -> Vec<u8> {
    let prehash = format!(
        "\u{0019}Ethereum Signed Message:\n{}{}",
        nonce.len(),
        nonce
    );
    let mut hasher = Keccak256::new();
    hasher.update(prehash.as_bytes());
    hasher.finalize().to_vec()
}

pub fn recover(signed_message: &[u8], signature: &[u8]) -> String {
    let recovery_id = RecoveryId::from_i32(signature[64] as i32 - 27).unwrap();
    let sig = RecoverableSignature::from_compact(&signature[0..64], recovery_id).unwrap();
    let message = Message::from_slice(signed_message).unwrap();
    let public_key = sig.recover(&message).unwrap();
    let address = get_address_from_public_key(&public_key);
    let address_hex = "0x".to_string() + &hex::encode(&address[12..]);

    address_hex
}

#[test]
fn test_recover() {
    let account = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8".to_string();
    let message = "0x63f9a92d8d61b48a9fff8d58080425a3012d05c8igwyk4r1o7o".to_string();
    let sign_message = get_sign_message(message);
    let signature = hex::decode("382a3e04daf88f322730f6a2972475fc5646ea8c4a7f3b5e83a90b10ba08a7364cd2f55348f2b6d210fbed7fc485abf19ecb2f3967e410d6349dd7dd1d4487751b").unwrap();
    println!("Account: {}", &account);

    let pubkey = recover(&sign_message, &signature);
    println!("PubKey {}", &pubkey);

    assert_eq!(account, pubkey)
}

#[test]
fn test_recover2() {
    let account = "0x332d40f9a6a242aa67be38dc8b86afb7cf959e17".to_string();
    let message = "app_abcd12345".to_string();
    let sign_message = get_sign_message(message);
    let signature = hex::decode("21fca3a238f4f1916bb6770a9f005b5edb2e1d0d4032516374ec614abb01fc8c29920ce26f7196c224e712433c843d8c7fa28d3f21d3f55182cb5b7910b7a80e1b").unwrap();
    println!("Account: {}", &account);

    let pubkey = recover(&sign_message, &signature);
    println!("PubKey {}", &pubkey);

    assert_eq!(account, pubkey)
}