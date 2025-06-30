use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::utils::crypto;
use crate::models::request::{SignMessageRequest, VerifyMessageRequest};
use crate::models::response::{SignMessageResponse, VerifyMessageResponse};

pub async fn sign_message(req: web::Json<SignMessageRequest>) -> impl Responder {
    let message = &req.message;
    let secret = &req.secret;

    match crypto::sign_message(message, secret) {
        Ok((signature, public_key)) => {
            let response = SignMessageResponse {
                signature,
                public_key,
                message: message.clone(),
            };
            HttpResponse::Ok().json(json!({
                "success": true,
                "data": response
            }))
        }
        Err(err) => HttpResponse::Ok().json(json!({
            "success": false,
            "error": err.to_string()
        })),
    }
}

pub async fn verify_message(req: web::Json<VerifyMessageRequest>) -> impl Responder {
    let message = &req.message;
    let signature = &req.signature;
    let pubkey = &req.pubkey;

    match crypto::verify_message(message, signature, pubkey) {
        Ok(valid) => {
            let response = VerifyMessageResponse {
                valid,
                message: message.clone(),
                pubkey: pubkey.clone(),
            };
            HttpResponse::Ok().json(json!({
                "success": true,
                "data": response
            }))
        }
        Err(err) => HttpResponse::Ok().json(json!({
            "success": false,
            "error": err.to_string()
        })),
    }
}