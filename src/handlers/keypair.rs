use axum::{response::IntoResponse};
use crate::models::response::KeypairOutput;
use base64::{engine::general_purpose, Engine as _};
use bs58;
use solana_sdk::signature::Keypair;

pub async fn handle_keypair_generation() -> impl IntoResponse {
    let new_keypair = Keypair::new();
    let encoded_secret = bs58::encode(&new_keypair.to_bytes()).into_string();
    let encoded_pubkey = new_keypair.pubkey().to_string();
    let keypair_result = KeypairOutput {
        pubkey: encoded_pubkey,
        secret: encoded_secret,
    };
    (
        axum::http::StatusCode::OK,
        axum::Json(serde_json::json!({
            "success": true,
            "data": keypair_result
        }))
    )
}