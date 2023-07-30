use serde_json::Value;

const RED: &str = "\x1b[1;31m";
const GREEN: &str = "\x1b[0;32m";
const BLUE: &str = "\x1b[1;34m";
const BOLD_WHITE: &str = "\x1b[1m";
const NC: &str = "\x1b[0m";

pub fn print_json(json_obj: Value, newline: bool, tab_cnt: usize) {

    let terminator = if newline {"\n"} else {""};

    // Creating space to add before value
    let mut spaces = String::new();

    for _ in 0..tab_cnt {
        spaces = spaces + "  ";
    }

    match json_obj {
        Value::Null => return,

        Value::Bool(v) => {
            print!("{}{}{}{}", RED, v, NC, terminator);
        }

        Value::Number(v) => {
            print!("{}{}{}{}", BOLD_WHITE, v, NC, terminator);
        }

        Value::String(v) => {
            print!("{}\"{}\"{}{}", GREEN, v, NC, terminator);
        },

        Value::Array(v) => {
            println!("[");

            // number of times need to put `,` after the value
            let mut sz = (v.len() - 1) as i32; 

            for elem in v {
                print!("{}  ", spaces);
                print_json(elem, false, tab_cnt + 1);
                println!("{}", if sz != 0 {","} else {""});
                sz -= 1;
            }

            print!("{}]{}", spaces, terminator);
        }

        Value::Object(v) =>  {

            println!("{{");

            // number of times need to put `,` after the value
            let mut sz = (v.len() - 1) as i32; 

            for (key, value) in v {
                print!("{}  {}\"{}\"{}: ", &spaces, BLUE, key, NC);
                print_json(value, false, tab_cnt + 1);
                println!("{}", if sz != 0 {","} else {""});
                sz -= 1;
            }

            print!("{}}}{}", spaces, terminator);
        }
    }
}
