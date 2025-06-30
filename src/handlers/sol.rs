use actix_web::{web, HttpResponse};
use serde_json::json;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SendSolRequest {
    pub from: String,
    pub to: String,
    pub lamports: u64,
}

#[derive(Deserialize)]
pub struct SendTokenRequest {
    pub destination: String,
    pub mint: String,
    pub owner: String,
    pub amount: u64,
}

pub async fn send_sol(req: web::Json<SendSolRequest>) -> HttpResponse {
    // Validate required fields
    if req.from.is_empty() || req.to.is_empty() || req.lamports == 0 {
        return HttpResponse::Ok().json(json!({
            "success": false,
            "error": "Missing or invalid required fields"
        }));
    }
    // Implement SOL transfer logic here
    HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "program_id": "11111111111111111111111111111111", // System Program ID
            "accounts": [
                req.from.clone(),
                req.to.clone()
            ],
            "instruction_data": "instruction_data"
        }
    }))
}

pub async fn send_token(req: web::Json<SendTokenRequest>) -> HttpResponse {
    // Validate required fields
    if req.destination.is_empty() || req.mint.is_empty() || req.owner.is_empty() || req.amount == 0 {
        return HttpResponse::Ok().json(json!({
            "success": false,
            "error": "Missing or invalid required fields"
        }));
    }
    // Implement SPL token transfer logic here
    HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "program_id": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", // SPL Token Program ID
            "accounts": [
                {
                    "pubkey": req.owner.clone(),
                    "isSigner": true
                },
                {
                    "pubkey": req.destination.clone(),
                    "isSigner": false
                }
            ],
            "instruction_data": "instruction_data"
        }
    }))
}