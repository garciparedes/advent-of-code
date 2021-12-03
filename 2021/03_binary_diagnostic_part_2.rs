use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let diagnostic: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect();

    let bit_len = diagnostic[0].len();

    let mut oxigens = diagnostic.clone();
    for i in 0..bit_len {
        if oxigens.len() == 1 {
            break;
        }
        let mut count = 0;
        for j in 0..oxigens.len() {
            if oxigens[j][i] == '1' {
                count += 1;
            }
        }
        if count * 2 >= oxigens.len() {
            oxigens = oxigens
                .into_iter()
                .filter(|row| row[i] == '1')
                .collect();
        } else {
            oxigens = oxigens
                .into_iter()
                .filter(|row| row[i] == '0')
                .collect();
        }
    }
    
    let mut co2s = diagnostic.clone();
    for i in 0..bit_len {
        if co2s.len() == 1 {
            break;
        }
        let mut count = 0;
        for j in 0..co2s.len() {
            if co2s[j][i] == '1' {
                count += 1;
            }
        }
        if count * 2 < co2s.len() {
            co2s = co2s
                .into_iter()
                .filter(|row| row[i] == '1')
                .collect();
        } else {
            co2s = co2s
                .into_iter()
                .filter(|row| row[i] == '0')
                .collect();
        }
    }

    let (mut oxigen, mut co2) = (0, 0);
    for i in 0..bit_len {
        if oxigens[0][i] == '1' {
            oxigen += i32::pow(2, (bit_len - i - 1) as u32);
        }
        if co2s[0][i] == '1' {
            co2 += i32::pow(2, (bit_len - i - 1) as u32);
        }
    }

    let ans = oxigen * co2;
    println!("{}", ans);

    return Ok(());
}
