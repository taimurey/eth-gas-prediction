use linfa::prelude::*;
use ndarray::Array;
use ndarray::{array, Array1};
use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.blocknative.com/gasprices/blockprices";
    let auth_header = "348231ff-8218-45bd-a947-f3fe1b3951d1";

    let client = reqwest::Client::new();

    let mut buffer: VecDeque<Value> = VecDeque::with_capacity(5000);

    // Training data and labels
    let mut X_train = Vec::new();
    let mut y_train = Vec::new();

    loop {
        let res = client
            .get(url)
            .header(AUTHORIZATION, auth_header)
            .send()
            .await?
            .json::<Value>()
            .await?;

        if buffer.len() == 5000 {
            buffer.pop_front();
        }
        buffer.push_back(res.clone());

        // New code to collect data for training
        if let Value::Object(main_obj) = &res {
            if let Some(Value::Array(block_prices)) = main_obj.get("blockPrices") {
                for block_price in block_prices {
                    if let Value::Object(block_price_obj) = block_price {
                        if let Some(Value::Array(estimated_prices)) =
                            block_price_obj.get("estimatedPrices")
                        {
                            for estimated_price in estimated_prices {
                                if let Value::Object(price_obj) = estimated_price {
                                    if let Some(Value::Number(confidence)) =
                                        price_obj.get("confidence")
                                    {
                                        if confidence.as_f64() == Some(99.0) {
                                            if let Some(Value::Number(max_fee_per_gas)) =
                                                price_obj.get("maxFeePerGas")
                                            {
                                                if let Some(Value::Number(
                                                    max_priority_fee_per_gas,
                                                )) = price_obj.get("maxPriorityFeePerGas")
                                                {
                                                    let sum =
                                                        max_fee_per_gas.as_f64().unwrap_or(0.0)
                                                            + max_priority_fee_per_gas
                                                                .as_f64()
                                                                .unwrap_or(0.0);
                                                    X_train.push(array![
                                                        max_fee_per_gas,
                                                        max_priority_fee_per_gas
                                                    ]);
                                                    y_train.push(sum);
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
        }

        // Train the machine learning model
        let X_train = Array::from_shape_vec((X_train.len(), 2), X_train).unwrap();
        let y_train = Array1::from(y_train);
        let model = linfa::linear::MultipleLinearRegression::fit(&X_train, &y_train)?;

        // New code to make predictions
        if let Value::Object(main_obj) = &res {
            if let Some(Value::Array(block_prices)) = main_obj.get("blockPrices") {
                for block_price in block_prices {
                    if let Value::Object(block_price_obj) = block_price {
                        if let Some(Value::Array(estimated_prices)) =
                            block_price_obj.get("estimatedPrices")
                        {
                            for estimated_price in estimated_prices {
                                if let Value::Object(price_obj) = estimated_price {
                                    if let Some(Value::Number(confidence)) =
                                        price_obj.get("confidence")
                                    {
                                        if confidence.as_f64() == Some(99.0) {
                                            if let Some(Value::Number(max_fee_per_gas)) =
                                                price_obj.get("maxFeePerGas")
                                            {
                                                if let Some(Value::Number(
                                                    max_priority_fee_per_gas,
                                                )) = price_obj.get("maxPriorityFeePerGas")
                                                {
                                                    // Create input data for prediction
                                                    let input = array![
                                                        max_fee_per_gas,
                                                        max_priority_fee_per_gas
                                                    ];

                                                    // Predict using the trained model
                                                    let prediction = model.predict(&input)?;

                                                    // Print the predicted higher gas prices
                                                    println!("Confidence: {}", confidence);
                                                    println!(
                                                        "Max Fee Per Gas: {}",
                                                        max_fee_per_gas
                                                    );
                                                    println!(
                                                        "Max Priority Fee Per Gas: {}",
                                                        max_priority_fee_per_gas
                                                    );
                                                    println!("Sum: {:.2}", prediction);
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
        }

        sleep(Duration::from_secs_f64(0.5)).await;
    }
}
