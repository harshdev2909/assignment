use axum::{extract::Json, response::IntoResponse};
use base64::{engine::general_purpose, Engine as _};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use spl_token::{instruction as token_instructions, ID as SPL_TOKEN_PROGRAM};
use spl_associated_token_account;
use crate::models::request::{TokenCreationInput, TokenMintingInput, TokenTransferInput};
use crate::models::response::{InstructionOutput, AccountMetadata, TokenAccountMetadata, TokenTransferOutput};
use crate::utils::validation::*;
use serde_json::json;

pub async fn handle_token_creation(Json(input): Json<TokenCreationInput>) -> impl IntoResponse {
    let authority_address = match input.mint_authority {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let mint_address = match input.mint {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let token_decimals = match input.decimals {
        Some(d) => d,
        None => return build_error_response("Missing required fields"),
    };
    if let Err(error_msg) = validate_token_decimals(token_decimals) {
        return build_error_response(&error_msg);
    }
    let parsed_authority = match validate_pubkey_format(authority_address, "mint authority") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    let parsed_mint = match validate_pubkey_format(mint_address, "mint") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    if parsed_authority == parsed_mint {
        return build_error_response("Mint and mint authority cannot be the same");
    }
    let token_instruction = match token_instructions::initialize_mint(
        &SPL_TOKEN_PROGRAM,
        &parsed_mint,
        &parsed_authority,
        None,
        token_decimals,
    ) {
        Ok(instruction) => instruction,
        Err(error) => return build_error_response(&format!("Failed to create token instruction: {}", error)),
    };
    let account_list: Vec<AccountMetadata> = token_instruction.accounts
        .iter()
        .map(|account| AccountMetadata {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();
    let encoded_instruction_data = general_purpose::STANDARD.encode(&token_instruction.data);
    let creation_result = InstructionOutput {
        program_id: token_instruction.program_id.to_string(),
        accounts: account_list,
        instruction_data: encoded_instruction_data,
    };
    build_success_response(creation_result)
}

pub async fn handle_token_minting(Json(input): Json<TokenMintingInput>) -> impl IntoResponse {
    let mint_address = match input.mint {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let destination_address = match input.destination {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let authority_address = match input.authority {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let mint_amount = match input.amount {
        Some(amount) => {
            if let Err(error_msg) = validate_amount_bounds(amount, "amount") {
                return build_error_response(&error_msg);
            }
            amount
        },
        None => return build_error_response("Missing required fields"),
    };
    let parsed_mint = match validate_pubkey_format(mint_address, "mint") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    let parsed_destination = match validate_pubkey_format(destination_address, "destination") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    let parsed_authority = match validate_pubkey_format(authority_address, "authority") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    if parsed_destination == parsed_mint {
        return build_error_response("Destination cannot be the same as mint address");
    }
    let mint_instruction = match token_instructions::mint_to(
        &SPL_TOKEN_PROGRAM,
        &parsed_mint,
        &parsed_destination,
        &parsed_authority,
        &[],
        mint_amount,
    ) {
        Ok(instruction) => instruction,
        Err(error) => return build_error_response(&format!("Failed to create mint instruction: {}", error)),
    };
    let account_list: Vec<AccountMetadata> = mint_instruction.accounts
        .iter()
        .map(|account| AccountMetadata {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
            is_writable: account.is_writable,
        })
        .collect();
    let encoded_instruction_data = general_purpose::STANDARD.encode(&mint_instruction.data);
    let minting_result = InstructionOutput {
        program_id: mint_instruction.program_id.to_string(),
        accounts: account_list,
        instruction_data: encoded_instruction_data,
    };
    build_success_response(minting_result)
}

pub async fn handle_token_transfer(Json(input): Json<TokenTransferInput>) -> impl IntoResponse {
    let destination_address = match input.destination {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let mint_address = match input.mint {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let owner_address = match input.owner {
        Some(ref addr) if !addr.trim().is_empty() => addr.trim(),
        _ => return build_error_response("Missing required fields"),
    };
    let transfer_amount = match input.amount {
        Some(amount) => {
            if let Err(error_msg) = validate_amount_bounds(amount, "amount") {
                return build_error_response(&error_msg);
            }
            amount
        },
        None => return build_error_response("Missing required fields"),
    };
    let parsed_destination = match validate_pubkey_format(destination_address, "destination") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    let parsed_mint = match validate_pubkey_format(mint_address, "mint") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    let parsed_owner = match validate_pubkey_format(owner_address, "owner") {
        Ok(addr) => addr,
        Err(error_msg) => return build_error_response(&error_msg),
    };
    let source_token_account = spl_associated_token_account::get_associated_token_address(&parsed_owner, &parsed_mint);
    if source_token_account == parsed_destination {
        return build_error_response("Cannot transfer to the same token account");
    }
    let transfer_instruction = match token_instructions::transfer(
        &SPL_TOKEN_PROGRAM,
        &source_token_account,
        &parsed_destination,
        &parsed_owner,
        &[],
        transfer_amount,
    ) {
        Ok(instruction) => instruction,
        Err(error) => return build_error_response(&format!("Failed to create transfer instruction: {}", error)),
    };
    let account_list: Vec<TokenAccountMetadata> = transfer_instruction.accounts
        .iter()
        .map(|account| TokenAccountMetadata {
            pubkey: account.pubkey.to_string(),
            is_signer: account.is_signer,
        })
        .collect();
    let encoded_instruction_data = general_purpose::STANDARD.encode(&transfer_instruction.data);
    let transfer_result = TokenTransferOutput {
        program_id: transfer_instruction.program_id.to_string(),
        accounts: account_list,
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