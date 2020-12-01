use std::io;
use std::io::prelude::*;
use std::cmp;
use std::cmp::Ordering;

const TARGET: i32 = 2020;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut numbers: Vec<_> = buffer.trim().split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    numbers.sort_unstable_by_key(|&k| cmp::Reverse(k));

    for i in 0..numbers.len() {
        for j in  (i + 1..numbers.len()).rev() {
            match (numbers[i] + numbers[j]).cmp(&TARGET) {
                Ordering::Equal => {
                    println!("{:?}", numbers[i] * numbers[j]);
                    return Ok(());
                },
                Ordering::Greater => break,
                _ => (),
            }
        }
    }
    println!("NOT FOUND!");

    return Ok(());
}
