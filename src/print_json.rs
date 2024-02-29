use serde_json::Value;

const RED: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[0;32m";
const BLUE: &str = "\x1b[1;34m";
const BOLD_WHITE: &str = "\x1b[1m";
const NC: &str = "\x1b[0m";

pub fn print_json(json_obj: Value, newline: bool, tab_cnt: usize) {
    let terminator = if newline { "\n" } else { "" };

    // Creating space to add before value
    let spaces: String = std::iter::repeat(' ').take(tab_cnt).collect();

    match json_obj {
        Value::Null => (),

        Value::Bool(v) => {
            print!("{}{}{}{}", RED, v, NC, terminator);
        }

        Value::Number(v) => {
            print!("{}{}{}{}", BOLD_WHITE, v, NC, terminator);
        }

        Value::String(v) => {
            print!("{}\"{}\"{}{}", GREEN, v, NC, terminator);
        }

        Value::Array(v) => {
            // number of times need to put `,` after the value
            let mut sz = v.len() as i32 - 1;

            // sz == -1 means the array is empty
            print!("[{}", if sz == -1 { "" } else { "\n" });

            for elem in v {
                print!("{}  ", spaces);
                print_json(elem, false, tab_cnt + 1);
                println!("{}", if sz != 0 { "," } else { "" });
                sz -= 1;
            }

            print!("{}]{}", spaces, terminator);
        }

        Value::Object(v) => {
            // number of times need to put `,` after the value
            let mut sz = v.len() as i32 - 1;

            // sz == -1 means the map is empty
            print!("{{{}", if sz == -1 { "" } else { "\n" });

            for (key, value) in v {
                print!("{}  {}\"{}\"{}: ", &spaces, BLUE, key, NC);
                print_json(value, false, tab_cnt + 1);
                println!("{}", if sz != 0 { "," } else { "" });
                sz -= 1;
            }

            print!("{}}}{}", spaces, terminator);
        }
    }
}
