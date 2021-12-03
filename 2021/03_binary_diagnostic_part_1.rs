use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let diagnostic: HashSet<_> = buffer
        .trim()
        .split('\n')
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();

    let bit_len = diagnostic
        .iter()
        .next()
        .unwrap()
        .len();

    let mut counts = vec![0; bit_len];
    for i in 0..bit_len {
        let mut count = 0;
        for bits in diagnostic.iter() {
            if bits[i] == '1' {
                count += 1;
            }
        }
        counts[i] = count;
    }

    let (mut gamma, mut epsilon) = (0, 0);
    for i in 0..bit_len {
        if counts[bit_len - i - 1] * 2 > diagnostic.len() {
            gamma += i32::pow(2, i as u32);
        } else {
            epsilon += i32::pow(2, i as u32);
        }
    }

    let ans = gamma * epsilon;
    println!("{}", ans);

    return Ok(());
}
