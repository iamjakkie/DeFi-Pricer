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

    pub async fn call_contract( // I need to return a dynamic json structure here, based on the function signature, e.g. function totalSupply() external view returns (uint256) - should return { "totalSupply": uint256 }
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
        println!("Result: {:?}", result);
        Ok(result.to_string())
    }
}

#[post("/call_contract", format = "application/json", data = "<request>")]
async fn call_contract(
    state: &State<Mutex<AppState>>,
    request: Json<ContractCallRequest>,
) -> Result<String, String> {
    println!("Request: {:?}", request);
    let state = state.lock().await;

    match state
        .call_contract(
            &request.contract_address,
            &request.function_signature,
            request.block_number,
        )
        .await{
            Ok(result) => Ok(serde_json::to_string(&result).unwrap()),
            Err(err) => Err(format!("Error: {}", err)),
        }
}

#[rocket::main]
async fn main() {
    let state = AppState::new().await;

    rocket::build()
        .manage(Mutex::new(state))
        .mount("/", routes![call_contract])
        .launch()
        .await
        .expect("Failed to launch the server");
}