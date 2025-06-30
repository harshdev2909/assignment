use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub fn validate_pubkey_format(address_str: &str, field_identifier: &str) -> Result<Pubkey, String> {
    let trimmed_address = address_str.trim();
    if trimmed_address.len() < 32 || trimmed_address.len() > 44 {
        return Err(format!("Invalid {} address format", field_identifier));
    }
    Pubkey::from_str(trimmed_address)
        .map_err(|_| format!("Invalid {} address", field_identifier))
}

pub fn validate_amount_bounds(amount_value: u64, field_identifier: &str) -> Result<(), String> {
    if amount_value == 0 {
        return Err(format!("Invalid {} - amount must be greater than 0", field_identifier));
    }
    if amount_value > u64::MAX / 2 {
        return Err(format!("Invalid {} - amount too large", field_identifier));
    }
    Ok(())
}

pub fn validate_token_decimals(decimal_count: u8) -> Result<(), String> {
    if decimal_count > 9 {
        return Err("Invalid decimals - maximum allowed is 9".to_string());
    }
    Ok(())
}

pub fn validate_message_constraints(message_content: &str) -> Result<(), String> {
    if message_content.len() > 1024 {
        return Err("Message too long - maximum 1024 characters".to_string());
    }
    Ok(())
} 