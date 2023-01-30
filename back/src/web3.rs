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

pub fn recover(signed_message: &[u8], signature: &[u8]) -> Result<String, secp256k1::Error> {
    let recovery_id = RecoveryId::from_i32(signature[64] as i32 - 27)?;
    let sig = RecoverableSignature::from_compact(&signature[0..64], recovery_id)?;
    let message = Message::from_slice(signed_message)?;
    let public_key = sig.recover(&message)?;
    let address = get_address_from_public_key(&public_key);
    let address_hex = "0x".to_string() + &hex::encode(&address[12..]);

    Ok(address_hex)
}
