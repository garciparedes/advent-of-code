use std::io;
use std::io::prelude::*;
use std::cmp;
use std::cmp::Ordering;

const TARGET: i32 = 2020;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut numbers: Vec<_> = buffer.trim().split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    if numbers.len() < 3 {
        println!("BAD INPUT");
        return Ok(());
    }
    numbers.sort_unstable_by_key(|&k| cmp::Reverse(k));

    for i in 0..numbers.len() - 2 {
        for j in  (i + 1..numbers.len() - 1).rev() {
            if numbers[i] + numbers[j] > TARGET {
                break;
            }
            for k in (j + 1..numbers.len()).rev() {
                match (numbers[i] + numbers[j] + numbers[k]).cmp(&TARGET) {
                    Ordering::Equal => {
                        println!("{:?}", numbers[i] * numbers[j] * numbers[k]);
                        return Ok(());
                    },
                    Ordering::Greater => break,
                    _ => (),
                }
            }
        }
    }
    println!("NOT FOUND!");

    return Ok(());
}
