use axum::{extract::Json, response::IntoResponse};
use base64::{engine::general_purpose, Engine as _};
use bs58;
use solana_sdk::signature::{Keypair, Signature};
use crate::models::request::{MessageSigningInput, MessageVerificationInput};
use crate::models::response::{SignatureOutput, VerificationOutput};
use crate::utils::validation::*;
use serde_json::json;

pub async fn handle_message_signing(Json(input): Json<MessageSigningInput>) -> impl IntoResponse {
    let message_content = match input.message {
        Some(ref msg) if !msg.trim().is_empty() => msg.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let secret_key = match input.secret {
        Some(ref key) if !key.trim().is_empty() => key.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    if let Err(error_msg) = validate_message_constraints(message_content) {
        return build_error_response(&error_msg);
    }
    let secret_bytes = match bs58::decode(secret_key).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return build_error_response("Invalid secret key format"),
    };
    if secret_bytes.len() != 64 {
        return build_error_response("Invalid secret key length");
    }
    let signing_keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(keypair) => keypair,
        Err(_) => return build_error_response("Invalid secret key"),
    };
    let message_signature = signing_keypair.sign_message(message_content.as_bytes());
    let encoded_signature = general_purpose::STANDARD.encode(message_signature.as_ref());
    let signing_result = SignatureOutput {
        signature: encoded_signature,
        public_key: signing_keypair.pubkey().to_string(),
        message: message_content.to_string(),
    };
    build_success_response(signing_result)
}

pub async fn handle_message_verification(Json(input): Json<MessageVerificationInput>) -> impl IntoResponse {
    let message_content = match input.message {
        Some(ref msg) if !msg.trim().is_empty() => msg.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let signature_data = match input.signature {
        Some(ref sig) if !sig.trim().is_empty() => sig.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let public_key_str = match input.pubkey {
        Some(ref pk) if !pk.trim().is_empty() => pk.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    if let Err(error_msg) = validate_message_constraints(message_content) {
        return build_error_response(&error_msg);
    }
    let verification_pubkey = match crate::utils::validation::validate_pubkey_format(public_key_str, "public key") {
        Ok(pk) => pk,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    let signature_bytes = match general_purpose::STANDARD.decode(signature_data) {
        Ok(bytes) => bytes,
        Err(_) => return build_error_response("Invalid signature format"),
    };
    if signature_bytes.len() != 64 {
        return build_error_response("Invalid signature length");
    }
    let decoded_signature = match Signature::try_from(signature_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => return build_error_response("Invalid signature"),
    };
    let verification_result = decoded_signature.verify(&verification_pubkey.to_bytes(), message_content.as_bytes());
    let verification_output = VerificationOutput {
        valid: verification_result,
        message: message_content.to_string(),
        pubkey: public_key_str.to_string(),
    };
    build_success_response(verification_output)
}

fn build_success_response<T: serde::Serialize>(data: T) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    (axum::http::StatusCode::OK, axum::Json(json!({ "success": true, "data": data })))
}

fn build_error_response(error_message: &str) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    (axum::http::StatusCode::BAD_REQUEST, axum::Json(json!({ "success": false, "error": error_message })))
}