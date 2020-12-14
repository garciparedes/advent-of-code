use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

const BASE_MASK: &'static str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

#[derive(Debug)]
enum Operation<'a> {
    ChangeMask(&'a str),
    UpdateMem(usize, u64),
}

impl<'a> Operation<'a> {

    fn new(raw: &'a str) -> Self {
        let parts: Vec<_> = raw.split(" = ").collect();

        return match parts[0] {
            "mask" => Self::ChangeMask(parts[1]),
            _ => {
                let index = parts[0][4..parts[0].len()-1].parse::<usize>().unwrap();
                let value = parts[1].parse::<u64>().unwrap();
                return Self::UpdateMem(index, value);
            }
        }; 
    }

}

fn transform(mask: &str, mut value: u64) -> u64 {
    for (i, b) in mask.chars().rev().enumerate() {
        match b {
            '1' => value |= 1 << i,
            '0' => value &= !0 ^ (1 << i),
            _ => (),
        }
    }
    return value;
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    
    let iterable = buffer
        .trim()
        .split('\n')
        .map(|raw| Operation::new(raw));

    let mut mask = BASE_MASK;
    let mut memory = HashMap::new();
    for operation in iterable {
        match operation {
            Operation::ChangeMask(new) => { mask = new; },
            Operation::UpdateMem(index, value) => { memory.insert(index, transform(mask, value)); },
        }
    }

    let ans: u64 = memory
        .values()
        .sum();

    println!("{:?}", ans);

    return Ok(());
}
