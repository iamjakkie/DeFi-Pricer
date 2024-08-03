#[macro_use] 
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{http, State};
use reqwest::Client;
use std::env;
use std::fs;
use tokio::sync::Mutex;
use web3::types::{Address, BlockId, BlockNumber};
use web3::transports::Http;
use web3::Web3;

#[derive(Debug, Deserialize, Serialize)]
struct ContractCallRequest {
    contract_address: String,
    block_number: u64,
    function_signature: String,
    contract_args: Vec<String>,
}

struct AppState {
    web3: Web3<Http>,
}

impl AppState {
    pub async fn new() -> Self {
        let node_url = env::var("NODE_URL").expect("NODE_URL must be set");
        let http = Http::new(&node_url).expect("Failed to create HTTP transport");
        let web3 = Web3::new(http);

        AppState { web3 }
    }

    pub async fn call_contract(
        &self,
        contract_address: &str,
        block_number: u64,
        function_signature: &str,
        params: Vec<String>,
    ) -> Result<String, web3::Error> {
        let address: Address = contract_address.parse().expect("Invalid contract address");

        let function = ethabi::Function {
            name: String::new(),
            inputs: ethabi::decode(&function_signature).expect("Invalid function signature"),
            outputs: vec![],
            state_mutability: ethabi::StateMutability::Pure,
        };

        let params_tokens: Vec<ethabi::Token> = params
            .into_iter()
            .map(|param| ethabi::Token::String(param))
            .collect();

        let data = function.encode_input(&params_tokens).expect("Failed to encode parameters");

        let call_result = self
            .web3
            .eth()
            .call(
                web3::types::CallRequest {
                    from: None,
                    to: Some(address),
                    gas: None,
                    gas_price: None,
                    value: None,
                    data: Some(data.into()),
                    transaction_type: None,
                    access_list: None,
                    max_priority_fee_per_gas: None,
                    max_fee_per_gas: None,
                },
                Some(BlockId::Number(BlockNumber::Number(block_height.into()))),
            )
            .await?;

        Ok(format!("0x{}", hex::encode(call_result)))
    }
}