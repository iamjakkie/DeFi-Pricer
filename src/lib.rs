use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractCallRequest {
    pub contract_address: String,
    pub function_signature: String,
    pub block_number: Option<u64>,
}
