use entropy::shannon_entropy;
use num_bigint::BigUint;
use reqwest::blocking::Client;
use std::sync::atomic::AtomicU64;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use textplots::{Chart, ColorPlot, Shape};

#[derive(serde::Deserialize, Debug)]
pub struct Resp {
    content: String,
}

static DIGIT_START: std::sync::atomic::AtomicU64 = AtomicU64::new(100000000000000);
const DIGITS_PER_REQUEST: usize = 1000;

// URL: https://api.pi.delivery/v1/pi?start=100000000000000&numberOfDigits=1000&radix=10

const PRINT_LEN: usize = 100;
const WHITE: rgb::RGB8 = rgb::RGB8::new(0xFF, 0xFF, 0xFF);
const GREEN: rgb::RGB8 = rgb::RGB8::new(0x00, 0xFF, 0x00);

fn main() {
    let should_run = Arc::new(AtomicBool::new(true));
    let should_run_ctrlc_ref = should_run.clone();

    let mut x: [(f32, f32); PRINT_LEN] = [(0., 0.); PRINT_LEN];

    // hide the cursor so we don't see it flying all over
    let term = console::Term::stdout();
    term.hide_cursor().unwrap();
    term.clear_screen().unwrap();

    // On ctrl+C, reset terminal settings and let the thread know to stop
    ctrlc::set_handler(move || {
        should_run_ctrlc_ref
            .as_ref()
            .store(false, Ordering::Relaxed);
    })
    .unwrap();

    // Build the http client for fetching digits
    let client = Client::builder()
    .user_agent("SaganSearch (https://github.com/phayes/SaganSearch, email patrick.d.hayes@gmail.com for questions)")
    .build().unwrap();

    // run until we get ctrl+C
    let mut digit_start: usize = 100000000000000;
    while should_run.as_ref().load(Ordering::Acquire) {
        //if digit_start % 10000 == 0 {
        //    eprintln!("Analysing π at digit {}", digit_start);
        //}

        let url = format!(
            "https://api.pi.delivery/v1/pi?start={}&numberOfDigits={}&radix=10",
            digit_start, DIGITS_PER_REQUEST
        );

        let http_resp = client.get(url).send().unwrap();

        let body = http_resp.bytes().unwrap();

        let resp: Resp = serde_json::from_slice(&body).unwrap_or_else(|e| {
            eprintln!("Got a serde_json::Error: {}", e);
            if let Ok(body_string) = std::str::from_utf8(&body) {
                eprintln!("body: {}", body_string);
            }

            std::process::exit(1);
        });
        let digits: BigUint = resp.content.parse().unwrap();
        let digit_bytes = digits.to_bytes_be();

        // TODO: prepend zeros to fill bytes to
        let entropy = shannon_entropy(&digit_bytes) / 8.0;
        if entropy < 0.9 && entropy != 0.0 {
            let digit_end = digit_start + DIGITS_PER_REQUEST;
            eprintln!(
                "Found anomalous entropy between digits {} and {}: {}",
                digit_start, digit_end, entropy
            );
            eprintln!("Anomalous digits: {}", digits);
            std::process::exit(0);
        }

        // Fill target with (0, 0.9), (1, 0.9), (2, 0.9), ...
        let target = {
            let mut target = Vec::with_capacity(PRINT_LEN);
            for index in 0..PRINT_LEN {
                target.push((index as f32, 0.9));
            }
            target
        };

        // update our plotting data
        let x_val = entropy as f32;
        x.copy_within(1..PRINT_LEN, 0);
        x[PRINT_LEN - 1] = (0., x_val as f32);
        for index in 0..PRINT_LEN {
            x[index].0 += 1.;
        }

        // update our UI
        term.move_cursor_to(0, 0).unwrap();
        print!("SaganSearch: searching for artificial messages in π");
        term.move_cursor_to(0, 1).unwrap();
        Chart::new_with_y_range(200, 100, 0., PRINT_LEN as f32, 1.0, 0.9)
            .linecolorplot(&Shape::Bars(&x), WHITE)
            .linecolorplot(&Shape::Lines(&target), GREEN)
            .display();
        term.move_cursor_to(0, 100).unwrap();
        print!("Digit: {} \t\t\t Entropy: {}", digit_start, entropy);

        // Go to the next digit
        digit_start = digit_start - DIGITS_PER_REQUEST;
    }

    // re-reveal the cursor
    let term = console::Term::stdout();
    term.show_cursor().unwrap();
}
