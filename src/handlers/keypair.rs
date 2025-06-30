use actix_web::{HttpResponse, Responder};
use solana_sdk::signature::{Keypair, Signer};
use serde_json::json;
use bs58;

pub async fn generate_keypair() -> impl Responder {
    let keypair = Keypair::new();
    let pubkey = keypair.pubkey().to_string();
    let secret = bs58::encode(keypair.to_bytes()).into_string();

    HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "pubkey": pubkey,
            "secret": secret
        }
    }))
}