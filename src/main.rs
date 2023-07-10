use entropy::shannon_entropy;
use num_bigint::BigUint;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use textplots::{Chart, ColorPlot, Shape};

#[derive(serde::Deserialize, Debug)]
pub struct Resp {
    content: String,
}

const DIGITS_PER_REQUEST: usize = 1000;

// URL: https://api.pi.delivery/v1/pi?start=100000000000000&numberOfDigits=1000&radix=10

const PRINT_LEN: usize = 100;
const WHITE: rgb::RGB8 = rgb::RGB8::new(0xFF, 0xFF, 0xFF);
const GREEN: rgb::RGB8 = rgb::RGB8::new(0x00, 0xFF, 0x00);
const USER_AGENT: &str = "SaganSearch (https://github.com/phayes/SaganSearch, email patrick.d.hayes@gmail.com for questions)";

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

    // Get arguments and check where we should start the search
    let mut digit_start: usize = {
        let args: Vec<String> = std::env::args().collect();

        if args.len() >= 2 {
            let start_digit: BigUint = args[1].parse().unwrap();
            start_digit.try_into().unwrap()
        } else {
            100000000000000
        }
    };

    // run until we get ctrl+C or a potential message is found
    while should_run.as_ref().load(Ordering::Acquire) {
        let url = format!(
            "https://api.pi.delivery/v1/pi?start={}&numberOfDigits={}&radix=10",
            digit_start, DIGITS_PER_REQUEST
        );

        let http_resp = minreq::get(url).with_header("User-Agent", USER_AGENT).send().unwrap();

        if http_resp.status_code == 429 {
            use std::{thread, time};
            thread::sleep(time::Duration::from_secs(30));
            continue;
        }

        let body = http_resp.into_bytes();

        let resp: Resp = serde_json::from_slice(&body).unwrap_or_else(|e| {
            eprintln!("Got a serde_json::Error: {}", e);
            if let Ok(body_string) = std::str::from_utf8(&body) {
                eprintln!("body: {}", body_string);
            }

            std::process::exit(1);
        });
        let digits: BigUint = resp.content.parse().unwrap();
        let mut digit_bytes = digits.to_bytes_be();

        // Prepend to fill to 416 bytes since we are using 1000 digits and the digits might start with zero
        while digit_bytes.len() < 416 {
            digit_bytes.insert(0, 0);
        }

        let entropy = shannon_entropy(&digit_bytes) / 8.0;
        if entropy < 0.9 && entropy != 0.0 {
            let digit_end = digit_start + DIGITS_PER_REQUEST;
            eprintln!("Great success!");
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

        // Go to the next set of digits
        digit_start = digit_start - DIGITS_PER_REQUEST;
    }
    println!("");
    println!("Ended search at digit: {}", digit_start);

    // re-reveal the cursor
    let term = console::Term::stdout();
    term.show_cursor().unwrap();
}
