use futures::{stream, StreamExt}; // 0.3.27
use reqwest::Client; // 0.11.14
use tokio; // 1.26.0, features = ["macros"]
use std::sync::atomic::AtomicU64;
use num_bigint::BigUint;
use entropy::shannon_entropy;

#[derive(serde::Deserialize, Debug)]
pub struct Resp {
    content: String
}

static DIGIT_START: std::sync::atomic::AtomicU64 = AtomicU64::new(100000000000000);
const DIGITS_PER_REQUEST: usize = 1000;

// URL: https://api.pi.delivery/v1/pi?start=100000000000000&numberOfDigits=1000&radix=10

#[tokio::main]
async fn main() {
    let client = reqwest::Client::builder()
    .user_agent("sagansearch (email patrick.d.hayes@gmail.com for questions)")
    .build().unwrap();
    
    loop {
        let digit_start = DIGIT_START.load(std::sync::atomic::Ordering::Relaxed) as usize;

        if digit_start % 10000 == 0 {
            eprintln!("Analysing π at digit {}", digit_start);
        }

        let url = format!(
            "https://api.pi.delivery/v1/pi?start={}&numberOfDigits={}&radix=10",
            digit_start, DIGITS_PER_REQUEST
        );

        let http_resp = client.get(url).send().await.unwrap();

        let body = http_resp.bytes().await.unwrap();

        let resp: Resp = serde_json::from_slice(&body).unwrap_or_else(|e| {
            eprintln!("Got a serde_json::Error: {}", e);
            if let Ok(body_string) = std::str::from_utf8(&body) {
                eprintln!("body: {}", body_string);
            }

            std::process::exit(1);
        });
        let digits: BigUint = resp.content.parse().unwrap();
        let digit_bytes = digits.to_bytes_be();
        let entropy = shannon_entropy(&digit_bytes) / 8.0;

        if entropy < 0.9 && entropy != 0.0 {
            let digit_end = digit_start + DIGITS_PER_REQUEST;
            eprintln!("Found anomalous entropy between digits {} and {}: {}", digit_start, digit_end, entropy);
            eprintln!("Anomalous digits: {}", digits);
            std::process::exit(0);
        }

        let next_digit_start = digit_start - DIGITS_PER_REQUEST;
        DIGIT_START.store(next_digit_start as u64, std::sync::atomic::Ordering::Relaxed);
    }
}
