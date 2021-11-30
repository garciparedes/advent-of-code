use std::io;
use std::io::prelude::*;
use serde_json::{Value, Map};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans: i64 = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let data: Value = serde_json::from_str(&line).unwrap();
            let mut s = 0;
            sum_numbers(&mut s, &data);

            return s;
        })
        .sum();

    println!("{}", ans);

    return Ok(());
}

fn sum_numbers(ans: &mut i64, data: &Value) {
    match data {
        Value::Number(n) => {*ans += n.as_i64().unwrap()},
        Value::Array(arr) => {
            for value in arr {
                sum_numbers(ans, value);
            }
        }, 
        Value::Object(mapping) => {
            if !contains_red(&mapping) {
                for value in mapping.values() {
                    sum_numbers(ans, value);
                }
            }
        }
        _ => (),
    } 
}

fn contains_red(mapping: &Map<String, Value>) -> bool{
    for value in mapping.values() {
        match value {
            Value::String(s) => {
                if s == "red" {
                    return true;
                }
            },
            _ => (),
        } 
    }
    return false;
}
