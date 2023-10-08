use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn fetch_protocols() -> Result<serde_json::Value, Box<dyn Error>> {
    let url = "https://api.llama.fi/protocols";
    let response: serde_json::Value = reqwest::get(url).await?.json().await?;
    Ok(response)
}

pub async fn get_protocol_historical(protocol: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://api.llama.fi/protocol/{}", protocol);
    let response: Value = reqwest::get(&url).await?.json().await?;
    Ok(response)
}

pub async fn fetch_historical_chain_tvl() -> Result<serde_json::Value, Box<dyn Error>> {
    let url = "https://api.llama.fi//v2/historicalChainTvl";
    let response: serde_json::Value = reqwest::get(url).await?.json().await?;
    Ok(response)
}
pub async fn historical_chain_tvl_chain(chain: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://api.llama.fi/v2/historicalChainTvl/{}", chain);
    let response: Value = reqwest::get(&url).await?.json().await?;
    Ok(response)
}

pub async fn tv1_protocol(protocol: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!("https://api.llama.fi/tv1/{}", protocol);
    let response: Value = reqwest::get(&url).await?.json().await?;
    Ok(response)
}

pub async fn fetch_v2_chain() -> Result<serde_json::Value, Box<dyn Error>> {
    let url = "https://api.llama.fi/v2/chains";
    let response: serde_json::Value = reqwest::get(url).await?.json().await?;
    Ok(response)
}
// #[tokio::test]
// async fn test_fetch_protocols() {
//     let response = fetch_protocols().await;

//     match &response {
//         Ok(json_val) => {
//             println!("Received JSON:");
//             println!("{:#?}", json_val);
//         }
//         Err(e) => println!("Error: {:?}", e),
//     }

//     assert!(response.is_ok());
// }
// #[tokio::test]
// async fn test_get_protocol_historical() {
//     let protocol = "aave"; // Or any other protocol slug
//     let response = get_protocol_historical(protocol).await;

//     match &response {
//         Ok(json_val) => {
//             println!("Received JSON:");
//             println!("{:#?}", json_val);
//         }
//         Err(e) => println!("Error: {:?}", e),
//     }

//     assert!(response.is_ok());
// }
