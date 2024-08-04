mod print_json;

use clap::Parser;
use print_json::print_json;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::process::ExitCode;

const ALLOW: &str = "allow";
const DISALLOW: &str = "disallow";

#[derive(Debug, Clone)]
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

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// All the filter labels to apply, if not provided then default will be used.
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    filter: Vec<String>,
}

const CONFIG_FILES_NAME: [&str; 2] = ["livejq.toml", "Livejq.toml"];

const CONFIG_PARSE_ERROR_MSG: &str = "Failed to parse config file, check and try again!";

fn main() -> ExitCode {
    let mut filters = Args::parse().filter;
    if filters.is_empty() {
        filters.push(String::from("default"));
    }

    let mut config_data: Option<String> = None;
    for file in CONFIG_FILES_NAME {
        if let Ok(content) = std::fs::read_to_string(file) {
            config_data = Some(content);
            break;
        }
    }

    let mut envs = HashMap::<String, Environment>::new();

    if !envs.contains_key("default") {
        envs.insert(String::from("default"), Environment::default());
    }

    if let Some(config) = config_data {
        let parsed = config.parse::<toml::Table>().expect(CONFIG_PARSE_ERROR_MSG);

        for (key, value) in parsed {
            if let toml::Value::Array(arr) = &value {
                // means this is default environment
                // since it does not have group
                // i.e [default] or [something_else]

                let env = envs.get_mut("default").unwrap();

                match key.as_str() {
                    // key is consumed
                    ALLOW => {
                        for json_key_name in arr {
                            env.allow.push(sanitize_string(&json_key_name.to_string()));
                        }
                    }
                    DISALLOW => {
                        for json_key_name in arr {
                            env.disallow
                                .push(sanitize_string(&json_key_name.to_string()));
                        }
                    }
                    _ => {
                        eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                        return ExitCode::FAILURE;
                    }
                }
            } else if let toml::Value::String(s) = &value {
                let env = envs.get_mut("default").unwrap();

                match key.as_str() {
                    ALLOW => {
                        env.allow.push(sanitize_string(s));
                    }
                    DISALLOW => {
                        env.disallow.push(sanitize_string(s));
                    }
                    _ => {
                        eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                        return ExitCode::FAILURE;
                    }
                }
            } else if let toml::Value::Table(t) = &value {
                let mut env = Environment::default();

                if t.contains_key(ALLOW) {
                    if let toml::Value::Array(arr) = t.get(ALLOW).expect(CONFIG_PARSE_ERROR_MSG) {
                        for json_key_name in arr {
                            env.allow.push(sanitize_string(&json_key_name.to_string()));
                        }
                    } else if let toml::Value::String(key_name) =
                        t.get(ALLOW).expect(CONFIG_PARSE_ERROR_MSG)
                    {
                        env.allow.push(sanitize_string(key_name));
                    } else {
                        eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                        return ExitCode::FAILURE;
                    }
                }

                if t.contains_key(DISALLOW) {
                    if let toml::Value::Array(arr) = t.get(DISALLOW).expect(CONFIG_PARSE_ERROR_MSG)
                    {
                        for json_key_name in arr {
                            env.disallow
                                .push(sanitize_string(&json_key_name.to_string()));
                        }
                    } else if let toml::Value::String(key_name) =
                        t.get(DISALLOW).expect(CONFIG_PARSE_ERROR_MSG)
                    {
                        env.disallow.push(sanitize_string(key_name));
                    } else {
                        eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                        return ExitCode::FAILURE;
                    }
                }

                envs.insert(key, env);
            } else {
                eprintln!("{CONFIG_PARSE_ERROR_MSG}");
                return ExitCode::FAILURE;
            }
        }
    }

    for (k, v) in &envs {
        if !v.allow.is_empty() && !v.disallow.is_empty() {
            eprintln!("Error: For every filter label, use either allow or disallow, not both.");
            eprintln!("       Both allow and disallow is used for key: {k}");
            return ExitCode::FAILURE;
        }
    }

    let mut allow_list = Vec::<&String>::new();
    let mut disallow_list = Vec::<&String>::new();

    for filter in filters {
        if !envs.contains_key(&filter) {
            eprintln!("Error: No label with name : {filter} found in config file.");
            return ExitCode::FAILURE;
        }

        let env = envs.get(&filter).unwrap();
        for allow in &env.allow {
            allow_list.push(allow);
        }
        for disallow in &env.disallow {
            disallow_list.push(disallow);
        }
    }

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

                    let mut show = allow_list.is_empty() && disallow_list.is_empty();

                    for (k, _) in json {
                        if disallow_list.contains(&k) {
                            show = false;
                            break;
                        }
                        if allow_list.contains(&k) {
                            show = true;
                            break;
                        }
                    }

                    // Token is a valid JSON
                    // recursively print the JSON values
                    if show {
                        print_json(parsed_json, true, 0);
                    }
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

fn sanitize_string(s: &String) -> String {
    s.trim_matches(|c| c == '"' || c == '\\').to_string()
}
