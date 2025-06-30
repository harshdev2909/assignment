use serde::Deserialize;

#[derive(Deserialize)]
pub struct GenerateKeypairRequest {
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Deserialize)]
pub struct CreateTokenRequest {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: String,
    pub mint: String,
    pub decimals: u8,
}

#[derive(Deserialize)]
pub struct MintTokenRequest {
    pub mint: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

#[derive(Deserialize)]
pub struct SignMessageRequest {
    pub message: String,
    pub secret: String,
}

#[derive(Deserialize)]
pub struct VerifyMessageRequest {
    pub message: String,
    pub signature: String,
    pub pubkey: String,
}

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

#[derive(Deserialize)]
pub struct TokenCreationInput {
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<String>,
    pub mint: Option<String>,
    pub decimals: Option<u8>,
}

#[derive(Deserialize)]
pub struct TokenMintingInput {
    pub mint: Option<String>,
    pub destination: Option<String>,
    pub authority: Option<String>,
    #[serde(deserialize_with = "crate::utils::parse_amount_field")]
    pub amount: Option<u64>,
}

#[derive(Deserialize)]
pub struct MessageSigningInput {
    pub message: Option<String>,
    pub secret: Option<String>,
}

#[derive(Deserialize)]
pub struct MessageVerificationInput {
    pub message: Option<String>,
    pub signature: Option<String>,
    pub pubkey: Option<String>,
}

#[derive(Deserialize)]
pub struct SolTransferInput {
    pub from: Option<String>,
    pub to: Option<String>,
    #[serde(deserialize_with = "crate::utils::parse_amount_field")]
    pub lamports: Option<u64>,
}

#[derive(Deserialize)]
pub struct TokenTransferInput {
    pub destination: Option<String>,
    pub mint: Option<String>,
    pub owner: Option<String>,
    #[serde(deserialize_with = "crate::utils::parse_amount_field")]
    pub amount: Option<u64>,
}