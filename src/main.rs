use reqwest::header::AUTHORIZATION;
use serde_json::Value;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.blocknative.com/gasprices/blockprices";
    let auth_header = "put your API key here";

    let client = reqwest::Client::new();

    let mut buffer: VecDeque<Value> = VecDeque::with_capacity(5000);

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

        // Output the entire JSON response to the console
        println!("{:#?}", res);

        sleep(Duration::from_micros(1)).await;
    }
}
