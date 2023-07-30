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

pub fn print_json(json_obj: Value, newline: bool) {

    let terminator = if newline {"\n"} else {""};

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
            print!("[ ");

            for elem in v {
                print_json(elem, false);
                print!(", ");
            }

            print!("]{}", terminator);
        }

        Value::Object(v) =>  {

            println!("{{");

            for (key, value) in v {
                print!("  \"{}\": ", key);
                print_json(value, false);
                println!();
            }

            print!("}}{}", terminator);
        }
    }
}
