use reqwest;
use serde_json::Value;
use std::error::Error;

pub async fn get_price_current_coin(chain: &str, address: &str) -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://coins.llama.fi/prices/current/{}:{}",
        chain, address
    );
    let response: Value = reqwest::get(&url).await?.json().await?;
    Ok(response)
}

pub async fn get_price_historical_timestamp_coin(
    timestamp: &str,
    coin: &str,
) -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://coins.llama.fi/prices/historical/{}:{}",
        timestamp, coin
    );

    let response: Value = reqwest::get(&url).await?.json().await?;
    Ok(response)
}

pub async fn batch_historical() -> Result<Value, Box<dyn Error>> {
    let url = "https://coins.llama.fi/batchHistorical";
    let response = reqwest::get(url).await?.json().await?;
    Ok(response)
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct PriceData {
    timestamp: i64,
    price: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoinData {
    decimals: Option<i8>,
    confidence: f64,
    prices: Vec<PriceData>,
    symbol: String,
}

type CoinMap = HashMap<String, CoinData>;

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    coins: CoinMap,
}
#[allow(unused)]
async fn get_chart_coin(
    coins: &str,
    start: i64,
    span: i64,
    period: &str,
    search_width: i64,
) -> Result<ApiResponse, reqwest::Error> {
    let base_url = "https://coins.llama.fi/chart";

    let url = format!("{}/{}", base_url, coins);
    let raw_response = reqwest::Client::new()
        .get(&url)
        .query(&[
            ("start", start.to_string()),
            ("span", span.to_string()),
            ("period", period.to_string()),
            ("searchWidth", search_width.to_string()),
        ])
        .send()
        .await?
        .text()
        .await?;

    println!("Raw API Response: {}", raw_response);

    let response: ApiResponse = serde_json::from_str(&raw_response).unwrap();

    Ok(response)
}

type CoinPercentageMap = HashMap<String, f64>;

#[derive(Debug, Serialize, Deserialize)]
struct PercentageResponse {
    coins: CoinPercentageMap,
}
#[allow(unused)]

async fn get_percentage_change(
    coins: &str,
    timestamp: i64,
    look_forward: bool,
    period: &str,
) -> Result<PercentageResponse, reqwest::Error> {
    let base_url = "https://coins.llama.fi/percentage";

    let url = format!("{}/{}", base_url, coins);
    let response = reqwest::Client::new()
        .get(&url)
        .query(&[
            ("timestamp", timestamp.to_string()),
            ("lookForward", look_forward.to_string()),
            ("period", period.to_string()),
        ])
        .send()
        .await?
        .json::<PercentageResponse>()
        .await?;

    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct CoinFirstPrice {
    price: f64,
    symbol: String,
    timestamp: i64,
}

type CoinFirstPriceMap = HashMap<String, CoinFirstPrice>;

#[derive(Debug, Serialize, Deserialize)]
struct FirstPriceResponse {
    coins: CoinFirstPriceMap,
}
#[allow(unused)]
async fn get_first_price_coin(coins: &str) -> Result<FirstPriceResponse, reqwest::Error> {
    let base_url = "https://coins.llama.fi/prices/first";

    let url = format!("{}/{}", base_url, coins);
    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json::<FirstPriceResponse>()
        .await?;

    Ok(response)
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockResponse {
    height: u64,
    timestamp: u64,
}
#[allow(unused)]
async fn get_block_chain_timestamp(
    chain: &str,
    timestamp: i64,
) -> Result<BlockResponse, reqwest::Error> {
    let base_url = "https://coins.llama.fi/block";

    let url = format!("{}/{}/{}", base_url, chain, timestamp);
    let response = reqwest::Client::new()
        .get(&url)
        .send()
        .await?
        .json::<BlockResponse>()
        .await?;

    Ok(response)
}
#[tokio::test]
async fn test_protocol() {
    let res= get_price_current_coin("ethereum:0xdF574c24545E5FfEcb9a659c229253D4111d87e1,coingecko:ethereum,bsc:0x762539b45a1dcce3d36d080f74d1aed37844b878,ethereum:0xdB25f211AB05b1c97D595516F45794528a807ad8", "1648680149").await;
    match &res {
        Ok(json_val) => {
            println!("Received JSON:");
            println!("{:#?}", json_val);
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
// #[tokio::test]
// async fn test_get_protocol_historical() {
//     let response =
//         get_price_current_coin("ethereum", "0xdF574c24545E5FfEcb9a659c229253D4111d87e1").await;

//     match &response {
//         Ok(json_val) => {
//             println!("Received JSON:");
//             println!("{:#?}", json_val);
//         }
//         Err(e) => println!("Error: {:?}", e),
//     }

//     assert!(response.is_ok());
// }
