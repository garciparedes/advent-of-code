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
                let key = parts[0][4..parts[0].len()-1].parse::<usize>().unwrap();
                let value = parts[1].parse::<u64>().unwrap();
                return Self::UpdateMem(key, value);
            }
        }; 
    }

}

fn transform(mask: &str, key: usize) -> Vec<usize> {
    let mut binaries = Vec::new(); 
    binaries.push(String::new());
    for (i, b) in mask.chars().enumerate() {
        match b {
            '1' => {
                for s in binaries.iter_mut() {
                    s.push('1');
                }
            },
            '0' => {
                let c = if key & (1 << (35 - i)) > 0 { '1' } else { '0' };
                for s in binaries.iter_mut() {
                    s.push(c)
                }
            },
            _ => {
                let n = binaries.len();
                for i in 0..n {
                    binaries.push(binaries[i].clone());
                }
                for i in 0..n {
                    binaries[i].push('0');
                }
                for i in n..(2 * n) {
                    binaries[i].push('1');
                }
            },
        }
    }

    let ans: Vec<_> = binaries
        .into_iter()
        .map(|bin| usize::from_str_radix(&bin, 2).unwrap())
        .collect();

    return ans;
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
            Operation::UpdateMem(key, value) => { 
                for transformed_key in transform(mask, key) {
                    memory.insert(transformed_key, value); 
                }
            },
        }
    }

    let ans: u64 = memory
        .values()
        .sum();

    println!("{:?}", ans);

    return Ok(());
}
