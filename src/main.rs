mod print_json;

use print_json::print_json;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::process::ExitCode;

const ALLOW: &str = "allow";
const DISALLOW: &str = "disallow";

#[derive(Debug)]
struct Environment {
    allow: Vec<String>,
    disallow: Vec<String>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            allow: Vec::<String>::new(),
            disallow: Vec::<String>::new(),
        }
    }
}

const CONFIG_FILES_NAME: [&str; 2] = ["livejq.toml", "Livejq.toml"];

const CONFIG_PARSE_ERROR_MSG: &str = "Failed to parse config file, check and try again!";

fn main() -> ExitCode {
    let mut config_data: Option<String> = None;
    for file in CONFIG_FILES_NAME {
        if let Ok(content) = std::fs::read_to_string(file) {
            config_data = Some(content);
            break;
        }
    }

    let mut envs = HashMap::<&str, Environment>::new();

    if let Some(config) = config_data {
        let parsed = config.parse::<toml::Table>().expect(CONFIG_PARSE_ERROR_MSG);

        for (key, value) in parsed {
            if let toml::Value::Array(arr) = &value {
                // means this is default environment
                // since it does not have group
                // i.e [default] or [something_else]

                let mut env = Environment::default();

                match key.as_str() {
                    ALLOW => {
                        for json_key_name in arr {
                            env.allow.push(json_key_name.to_string())
                        }
                    }
                    DISALLOW => {
                        for json_key_name in arr {
                            env.disallow.push(json_key_name.to_string());
                        }
                    }
                    _ => {
                        eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                        return ExitCode::FAILURE;
                    }
                }

                envs.insert("default", env);
            } else if let toml::Value::String(s) = &value {
                let mut env = Environment::default();
                match key.as_str() {
                    ALLOW => {
                        env.allow.push(s.clone());
                    }
                    DISALLOW => {
                        env.disallow.push(s.clone());
                    }
                    _ => {
                        eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                        return ExitCode::FAILURE;
                    }
                }

                envs.insert("default", env);
            } else if let toml::Value::Table(t) = &value {
                let mut env = Environment::default();

                if t.contains_key(ALLOW) {
                    if let toml::Value::Array(arr) = t.get(ALLOW).expect(CONFIG_PARSE_ERROR_MSG) {
                        for json_key_name in arr {
                            env.allow.push(json_key_name.to_string());
                        }
                    } else if let toml::Value::String(key_name) =
                        t.get(ALLOW).expect(CONFIG_PARSE_ERROR_MSG)
                    {
                        env.allow.push(key_name.clone());
                    } else {
                        eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                        return ExitCode::FAILURE;
                    }
                }

                if t.contains_key(DISALLOW) {
                    if let toml::Value::Array(arr) = t.get(DISALLOW).expect(CONFIG_PARSE_ERROR_MSG)
                    {
                        for json_key_name in arr {
                            env.disallow.push(json_key_name.to_string());
                        }
                    } else if let toml::Value::String(key_name) =
                        t.get(DISALLOW).expect(CONFIG_PARSE_ERROR_MSG)
                    {
                        env.disallow.push(key_name.clone());
                    } else {
                        eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                        return ExitCode::FAILURE;
                    }
                }

                envs.insert("hello", env);
            } else {
                eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                return ExitCode::FAILURE;
            }
        }
    }

    dbg!(envs);

    println!();
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                if let Ok(parsed_json) = serde_json::from_str::<Value>(&input) {
                    let json = parsed_json
                        .as_object()
                        .expect("Failed to parse json, try again!");

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

    return ExitCode::SUCCESS;
}
