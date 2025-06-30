use actix_web::{web, HttpResponse};
use serde_json::json;
use serde::Deserialize;
use axum::{extract::Json, response::IntoResponse};
use base64::{engine::general_purpose, Engine as _};
use solana_sdk::system_instruction;
use crate::models::request::SolTransferInput;
use crate::models::response::SolTransferOutput;
use crate::utils::validation::*;

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

pub async fn handle_sol_transfer(Json(input): Json<SolTransferInput>) -> impl IntoResponse {
    let source_address = match input.from {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let destination_address = match input.to {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let transfer_lamports = match input.lamports {
        Some(amount) => {
            if let Err(error_msg) = validate_amount_bounds(amount, "lamports") {
                return build_error_response(&error_msg);
            }
            amount
        },
        None => return build_error_response("Missing required fields"),
    };
    let parsed_source = match validate_pubkey_format(source_address, "from") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    let parsed_destination = match validate_pubkey_format(destination_address, "to") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    if parsed_source == parsed_destination {
        return build_error_response("Cannot transfer to the same address");
    }
    let transfer_instruction = system_instruction::transfer(&parsed_source, &parsed_destination, transfer_lamports);
    let account_addresses: Vec<String> = transfer_instruction.accounts
        .iter()
        .map(|account| account.pubkey.to_string())
        .collect();
    let encoded_instruction_data = general_purpose::STANDARD.encode(&transfer_instruction.data);
    let transfer_result = SolTransferOutput {
        program_id: transfer_instruction.program_id.to_string(),
        accounts: account_addresses,
        instruction_data: encoded_instruction_data,
    };
    build_success_response(transfer_result)
}

fn build_success_response<T: serde::Serialize>(data: T) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    (axum::http::StatusCode::OK, axum::Json(json!({ "success": true, "data": data })))
}

fn build_error_response(error_message: &str) -> (axum::http::StatusCode, axum::Json<serde_json::Value>) {
    (axum::http::StatusCode::BAD_REQUEST, axum::Json(json!({ "success": false, "error": error_message })))
}