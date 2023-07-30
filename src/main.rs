mod print_json;

use std::io::stdin;
use print_json::print_json;
use serde_json::Value;

fn main() {
    loop {
        let mut token = String::new();
        stdin().read_line(&mut token).expect("livejq: failed to read token from input");

        if let Ok(parsed_json) = serde_json::from_str::<Value>(&token) {
            // Token is a valid JSON
            
            // recursively print the JSON values 
            print_json(parsed_json, true, 0);
        } else {
            println!("@");
            // Token is not a valid JSON 
            
            // So just print it
            println!("{}", token);
        }
    }
}
