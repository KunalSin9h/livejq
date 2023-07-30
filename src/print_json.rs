use serde_json::Value;

/*
 * enum Value {
        Null,
        Bool(bool),
        Number(Number),
        String(String),
        Array(Vec<Value>),
        Object(Map<String, Value>),
    }
*/

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
            print!("{}{}", v, terminator);
        }

        Value::Number(v) => {
            print!("{}{}", v, terminator);
        }

        Value::String(v) => {
            print!("\"{}\"{}", v, terminator);
        },

        Value::Array(v) => {
            println!("[");

            for elem in v {
                print!("{}  ", spaces);
                print_json(elem, false, tab_cnt + 1);
                println!(",");
            }

            print!("{}]{}", spaces, terminator);
        }

        Value::Object(v) =>  {

            println!("{{");

            for (key, value) in v {
                print!("{}  \"{}\": ", &spaces, key);
                print_json(value, false, tab_cnt + 1);
                println!();
            }

            print!("{}}}{}", spaces, terminator);
        }
    }
}
