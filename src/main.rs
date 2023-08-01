mod print_json;

use print_json::print_json;
use serde_json::Value;
use std::io::{self, BufRead};

fn main() {

    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                if let Ok(parsed_json) = serde_json::from_str::<Value>(&input) {
                    // Token is a valid JSON
                    // recursively print the JSON values 
                    print_json(parsed_json, true, 0);
                } else {
                    // Token is not a valid JSON 
                    // So just print it
                    println!("{}", input);
                }
            }
            Err(_) => {
                break;
            }
        }
    }
}
