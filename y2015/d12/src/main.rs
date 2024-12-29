use std::{fs::File, io::BufReader};

use serde_json::Value;

fn main() {
    let input = parse_json("input");
    let total = sum(&input);

    println!("{}", total);
}

fn parse_json(path: &str) -> Value {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

fn sum(value: &Value) -> i64 {
    match value {
        Value::Number(number) => number.as_i64().unwrap(),
        Value::Array(vec) => vec.iter().map(sum).sum(),
        Value::Object(map) => {
            if map.values().any(|v| v == &Value::String("red".to_string())) {
                return 0;
            }
            map.values().map(sum).sum()
        }
        _ => 0,
    }
}
