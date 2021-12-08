use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans: usize = buffer
        .trim()
        .split('\n')
        .map(|line| {

            let (_, raw_digits) = line.split_once(" | ").unwrap();
            let digits: Vec<_> = raw_digits.trim().split_whitespace().collect();

            return digits
                .iter()
                .filter(|digit| digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7)
                .count();
        })
        .sum();

    println!("{}", ans);

    return Ok(());
}
