#[macro_use]
extern crate rocket;

use abi::AbiParser;
use anyhow::{anyhow, Result};
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use k256::elliptic_curve::rand_core::block;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use rocket::response::status;
use std::env;
use std::sync::Arc;
use diesel::RunQueryDsl;
use rocket::response::status::Created;
use tokio::sync::Mutex;

use DeFi_Pricer::ContractCallRequest;
use db::models::{NewToken, Token};
use db::connection::establish_connection_pg;



struct ContractCallResponse {
    
}

struct AppState {
    provider: Arc<Provider<Http>>,
}

// Type -> Map<bytes4, ABI / Contract>
// function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)

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
    ) -> Result<String> {
        let address: Address = contract_address.parse()?;
        let block = match block_number {
            Some(block_number) => BlockId::Number(block_number.into()),
            None => BlockId::Number(BlockNumber::Latest),
        };

        let abi = format!("[{}]", function_signature);

        let abi = AbiParser::default()
            .parse_str(&abi)
            .map_err(|e| anyhow!("Failed to parse function signature: {:?}", e))?;
        let function = abi
            .functions()
            .next()
            .ok_or(anyhow!("No functions found in the ABI"))?;
        let contract = Contract::new(address, abi.clone(), self.provider.clone());
        let result = contract
            .method::<_, (U256, U256)>(function.name.as_str(), ())?
            .block(block)
            .call()
            .await?;
        println!("Result: {:?}", result);

        let result = format!("{:?}", result);
        Ok(result)
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
            Ok(result) => {
                println!("Got result! {:?}", result);

                Ok(serde_json::to_string(&result).unwrap())
            },
            Err(err) => {
                println!("Got err: {:?}", err);

                Err(format!("Error: {}", err))
            },
        }
}

#[post("/add_token", format = "json", data = "<post>")]
pub fn add_token(post: Json<Token>) -> Result<Created<Json<Token>>, status::Custom<Json<String>>> {
    println!("Request: {:?}", post);
    let connection = &mut establish_connection_pg();

    let new_token = Token {
        id: 0,
        name: post.name.clone(),
        symbol: post.symbol.clone(),
        decimals: post.decimals,
        total_supply: post.total_supply,
        block: post.block,
    };

    diesel::insert_into(db::schema::tokens::table)
        .values(&new_token)
        .execute(connection)
        .expect("Error saving new post");
    Ok(Created::new("/").body(post))
}

#[post("/add_pair")]
async fn add_pair() -> Result<String, String> {
    // this should add a pair to a database with basic information
    Ok("boo!".to_string())
}

#[post("/add_trade")]
async fn add_trade() -> Result<String, String> {
    // this should add a trade to a database
    Ok("boo!".to_string())
}

#[post("/sync_pair")]
async fn sync_pair() -> Result<String, String> {
    // this function will get all historical price changes for a pair and store them in the database
    Ok("boo!".to_string())
}

// headers = {
//     "Content-Type": "application/json",
// }

// body = {
//     "contract_address": "0xA43fe16908251ee70EF74718545e4FE6C5cCEc9f",
//     "function_signature": "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)"
// }


// 'Error: Expected `U256`, got Tuple([Uint(2226262424639007986121597746328), Uint(7052167907527757790769), Uint(1722970811)])'

#[rocket::main]
async fn main() {
    let state = AppState::new().await;

    rocket::build()
        .manage(Mutex::new(state))
        .mount("/", routes![add_token])
        .launch()
        .await
        .expect("Failed to launch the server");
}