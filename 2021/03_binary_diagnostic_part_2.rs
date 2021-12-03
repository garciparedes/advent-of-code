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

    let mut oxigens = diagnostic.clone();
    for i in 0..bit_len {
        if oxigens.len() == 1 {
            break;
        }
        let mut count = 0;
        for bits in oxigens.iter() {
            if bits[i] == '1' {
                count += 1;
            }
        }
        if count * 2 >= oxigens.len() {
            oxigens.retain(|bits| bits[i] == '1');
        } else {
            oxigens.retain(|bits| bits[i] == '0');
        }
    }
    
    let mut co2s = diagnostic.clone();
    for i in 0..bit_len {
        if co2s.len() == 1 {
            break;
        }
        let mut count = 0;
        for bits in co2s.iter() {
            if bits[i] == '1' {
                count += 1;
            }
        }
        if count * 2 < co2s.len() {
            co2s.retain(|bits| bits[i] == '1');
        } else {
            co2s.retain(|bits| bits[i] == '0');
        }
    }

    let (oxigen_bits, co2_bits) = (oxigens.iter().next().unwrap(), co2s.iter().next().unwrap());
    let (mut oxigen, mut co2) = (0, 0);
    for i in 0..bit_len {
        if oxigen_bits[i] == '1' {
            oxigen += i32::pow(2, (bit_len - i - 1) as u32);
        }
        if co2_bits[i] == '1' {
            co2 += i32::pow(2, (bit_len - i - 1) as u32);
        }
    }

    let ans = oxigen * co2;
    println!("{}", ans);

    return Ok(());
}
