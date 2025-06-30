use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::models::request::{CreateTokenRequest, MintTokenRequest};
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction as token_instruction;
use base64::Engine;
use std::str::FromStr;

pub async fn create_token(req: web::Json<CreateTokenRequest>) -> impl Responder {
    // Validate required fields
    if req.mint_authority.is_empty() || req.mint.is_empty() {
        return HttpResponse::Ok().json(json!({
            "success": false,
            "error": "Missing required fields"
        }));
    }

    let mint_pubkey = match Pubkey::from_str(&req.mint) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::Ok().json(json!({"success": false, "error": "Invalid mint pubkey"})),
    };
    let mint_authority = match Pubkey::from_str(&req.mint_authority) {
        Ok(pk) => pk,
        Err(_) => return HttpResponse::Ok().json(json!({"success": false, "error": "Invalid mint authority pubkey"})),
    };
    let decimals = req.decimals;

    let ix = match token_instruction::initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &mint_authority,
        None,
        decimals,
    ) {
        Ok(ix) => ix,
        Err(_) => return HttpResponse::Ok().json(json!({"success": false, "error": "Failed to create instruction"})),
    };

    let accounts: Vec<_> = ix.accounts.iter().map(|acc| json!({
        "pubkey": acc.pubkey.to_string(),
        "isSigner": acc.is_signer,
        "isWritable": acc.is_writable
    })).collect();
    let instruction_data = base64::engine::general_purpose::STANDARD.encode(ix.data);
    let program_id = ix.program_id.to_string();

    HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "program_id": program_id,
            "accounts": accounts,
            "instruction_data": instruction_data
        }
    }))
}

pub async fn mint_token(req: web::Json<MintTokenRequest>) -> impl Responder {
    // Validate required fields
    if req.mint.is_empty() || req.destination.is_empty() || req.authority.is_empty() {
        return HttpResponse::Ok().json(json!({
            "success": false,
            "error": "Missing required fields"
        }));
    }

    // Placeholder for actual minting logic
    let program_id = "TokenProgramId"; // Replace with actual program ID
    let accounts = vec![
        json!({
            "pubkey": req.mint,
            "isSigner": false,
            "isWritable": true
        }),
        json!({
            "pubkey": req.destination,
            "isSigner": false,
            "isWritable": true
        }),
        json!({
            "pubkey": req.authority,
            "isSigner": true,
            "isWritable": false
        })
    ];
    let instruction_data = "base64-encoded-data"; // Replace with actual instruction data

    HttpResponse::Ok().json(json!({
        "success": true,
        "data": {
            "program_id": program_id,
            "accounts": accounts,
            "instruction_data": instruction_data
        }
    }))
}