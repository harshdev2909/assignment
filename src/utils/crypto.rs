use solana_sdk::signature::{Keypair, Signer};
use bs58::{decode as bs58_decode, encode as bs58_encode};
use base64::Engine;

pub fn generate_keypair() -> (String, String) {
    let keypair = Keypair::new();
    let pubkey = bs58_encode(keypair.pubkey().to_bytes()).into_string();
    let secret = bs58_encode(keypair.to_bytes()).into_string();
    (pubkey, secret)
}

pub fn sign_message(message: &str, secret: &str) -> Result<(String, String), &'static str> {
    let secret_bytes = match bs58_decode(secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid secret key format"),
    };
    let keypair = Keypair::from_bytes(&secret_bytes).map_err(|_| "Invalid secret key")?;
    let signature = keypair.sign_message(message.as_bytes());
    Ok((
        base64::engine::general_purpose::STANDARD.encode(signature.as_ref()),
        bs58_encode(keypair.pubkey().to_bytes()).into_string(),
    ))
}

pub fn verify_message(message: &str, signature: &str, pubkey: &str) -> Result<bool, &'static str> {
    let signature_bytes = match base64::engine::general_purpose::STANDARD.decode(signature) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid signature format"),
    };
    let pubkey_bytes = match bs58_decode(pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid public key format"),
    };
    let pubkey = solana_sdk::pubkey::Pubkey::new(&pubkey_bytes);
    let signature = solana_sdk::signature::Signature::new(&signature_bytes);
    Ok(signature.verify(pubkey.as_ref(), message.as_bytes()))
}