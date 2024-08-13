use reqwest::header::CONTENT_TYPE;
use DeFi_Pricer::ContractCallRequest;

#[tokio::main]
async fn main() {
    let contract_call = ContractCallRequest {
        contract_address: "0xA43fe16908251ee70EF74718545e4FE6C5cCEc9f".to_string(),
        function_signature: "function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast)".to_string(),
        block_number: None,
    };

    let req_str = serde_json::to_string(&contract_call).expect("Cannot serialize");


    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:8000/call_contract")
        .json(&contract_call)
        // .header(CONTENT_TYPE, "application/json")
        // .body(req_str)
        .send()
        .await
        .expect("Cannot send request");
    let res: String = res.json().await.expect("Cannot deser"); 

    println!("Res: {:?}", res);

    // let res = client.get("http://127.0.0.1:8000/hello-world")
    //     .send()
    //     .await
    //     .expect("Cannot send GET request");
}

// cargo run --example call_endpoint