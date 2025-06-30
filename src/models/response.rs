use serde::Serialize;

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct CreateTokenResponse {
    pub program_id: String,
    pub accounts: Vec<TokenAccount>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct TokenAccount {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct MintTokenResponse {
    pub program_id: String,
    pub accounts: Vec<TokenAccount>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SignMessageResponse {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerifyMessageResponse {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SendTokenResponse {
    pub program_id: String,
    pub accounts: Vec<TokenAccount>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct KeypairOutput {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct AccountMetadata {
    pub pubkey: String,
    pub is_signer: bool,
    pub is_writable: bool,
}

#[derive(Serialize)]
pub struct InstructionOutput {
    pub program_id: String,
    pub accounts: Vec<AccountMetadata>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct SignatureOutput {
    pub signature: String,
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct VerificationOutput {
    pub valid: bool,
    pub message: String,
    pub pubkey: String,
}

#[derive(Serialize)]
pub struct SolTransferOutput {
    pub program_id: String,
    pub accounts: Vec<String>,
    pub instruction_data: String,
}

#[derive(Serialize)]
pub struct TokenAccountMetadata {
    pub pubkey: String,
    #[serde(rename = "isSigner")]
    pub is_signer: bool,
}

#[derive(Serialize)]
pub struct TokenTransferOutput {
    pub program_id: String,
    pub accounts: Vec<TokenAccountMetadata>,
    pub instruction_data: String,
}