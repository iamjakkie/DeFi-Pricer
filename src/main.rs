#[macro_use]
extern crate rocket;

use abi::AbiParser;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use k256::elliptic_curve::rand_core::block;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize)]
struct ContractCallRequest {
    contract_address: String,
    function_signature: String,
    block_number: Option<u64>,
}

struct AppState {
    provider: Arc<Provider<Http>>,
}

impl AppState {
    pub async fn new() -> Self {
        let rpc_url = env::var("RPC_URL").expect("RPC_URL must be set");
        let provider = Provider::<Http>::try_from(rpc_url.as_str())
            .expect("Failed to connect to the provider");
        let provider = Arc::new(provider);

        AppState { provider }
    }

    pub async fn call_contract(
        &self,
        contract_address: &str,
        function_signature: &str,
        block_number: Option<u64>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let address: Address = contract_address.parse()?;
        let block = match block_number {
            Some(block_number) => BlockId::Number(block_number.into()),
            None => BlockId::Number(BlockNumber::Latest),
        };

        let abi = format!("[{}]", function_signature);

        let abi = AbiParser::default()
            .parse_str(&abi)
            .expect("Failed to parse function signature");
        let function = abi
            .functions()
            .next()
            .expect("No functions found in the ABI");
        let contract = Contract::new(address, abi.clone(), self.provider.clone());
        let result = contract
            .method::<_, U256>(function.name.as_str(), ())?
            .block(block)
            .call()
            .await?;

        Ok(result.to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_state = AppState::new().await;

    let contract_address = "0x6982508145454Ce325dDbE47a25d4ec3d2311933";
    let function_signature = "function totalSupply() external view returns (uint256)";

    match app_state
        .call_contract(contract_address, function_signature, None)
        .await
    {
        Ok(result) => println!("Result: {}", result),
        Err(err) => eprintln!("Error: {}", err),
    }

    Ok(())
}
