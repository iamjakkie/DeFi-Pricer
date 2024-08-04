use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::types::Address;
use std::sync::Arc;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract_address: Address = "0x6982508145454Ce325dDbE47a25d4ec3d2311933".parse().unwrap();
    let rpc_url = env::var("RPC_URL").unwrap();
    let provider = Provider::<Http>::try_from(rpc_url.as_str()).unwrap();
    let provider = Arc::new(provider);
    abigen!(IERC20, r#"[function totalSupply() external view returns (uint256)]"#);
    let contract = IERC20::new(contract_address, provider);
    
    if let Ok(total_supply) = contract.total_supply().call().await {
        println!("Total supply: {}", total_supply);
    }

    Ok(())
}