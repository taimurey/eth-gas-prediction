use log::info;
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::time::sleep;

async fn fetch_gas_prices(
    url: &str,
    auth_header: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let res = client
        .get(url)
        .header(AUTHORIZATION, auth_header)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(res)
}

fn process_gas_prices(res: &Value) {
    if let Value::Object(main_obj) = res {
        if let Some(Value::Array(block_prices)) = main_obj.get("blockPrices") {
            if let Some(Value::Object(block_price_obj)) = block_prices.first() {
                if let Some(Value::Array(estimated_prices)) = block_price_obj.get("estimatedPrices")
                {
                    if let Some(Value::Object(price_obj)) = estimated_prices.first() {
                        if let Some(Value::Number(confidence)) = price_obj.get("confidence") {
                            if confidence.as_f64() == Some(99.0) {
                                if let Some(Value::Number(max_fee_per_gas)) =
                                    price_obj.get("maxFeePerGas")
                                {
                                    if let Some(Value::Number(max_priority_fee_per_gas)) =
                                        price_obj.get("maxPriorityFeePerGas")
                                    {
                                        let sum = max_fee_per_gas.as_f64().unwrap_or(0.0)
                                            + max_priority_fee_per_gas.as_f64().unwrap_or(0.0);
                                        let rounded_sum = (sum * 100.0).round() / 100.0; // Round to 2 decimal places
                                        info!("Confidence: {}", confidence);
                                        info!("Max Fee Per Gas: {}", max_fee_per_gas);
                                        info!(
                                            "Max Priority Fee Per Gas: {}",
                                            max_priority_fee_per_gas
                                        );
                                        println!("Sum: {:.2}", rounded_sum);
                                        // Display rounded sum with 2 decimal places
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.blocknative.com/gasprices/blockprices";
    let auth_header = "348231ff-8218-45bd-a947-f3fe1b3951d1";
    env_logger::init();

    let mut buffer: VecDeque<Value> = VecDeque::with_capacity(5000);

    loop {
        let res = fetch_gas_prices(url, auth_header).await?;

        if buffer.len() == 5000 {
            buffer.pop_front();
        }
        buffer.push_back(res.clone());

        process_gas_prices(&res);

        sleep(Duration::from_secs_f64(0.5)).await;
    }
}
