use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans: i32 = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let (raw_pattern, raw_digits) = line.split_once(" | ").unwrap();

            let mut patterns: Vec<_> = raw_pattern
                .trim()
                .split_whitespace()
                .map(|d| d.chars().collect::<HashSet<_>>())
                .collect();
            patterns.sort_unstable_by_key(|p| p.len());

            let digits: Vec<_> = raw_digits
                .trim()
                .split_whitespace()
                .map(|d| d.chars().collect::<HashSet<_>>())
                .collect();

            let value = digits
                .iter()
                .map(|digit| decode(&patterns, digit))
                .fold(0, |cum, d| cum * 10 + d);

            return value;
        })
        .sum();

    println!("{}", ans);

    return Ok(());
}

fn decode(patterns: &[HashSet<char>], digit: &HashSet<char>) -> i32 {
    if digit.len() == patterns[0].len() {
        return 1;
    }
    if digit.len() == patterns[1].len() {
        return 7;
    }
    if digit.len() == patterns[2].len() {
        return 4;
    }
    if digit.len() == patterns[9].len() {
        return 8;
    }

    if digit.len() == 5 {
        if patterns[0].difference(digit).count() == 0 {
            return 3;
        } else if patterns[2].difference(digit).count() == 1{
            return 5;
        } else {
            return 2;
        }
    } else {
        if patterns[0].difference(digit).count() > 0 {
            return 6;
        } else if patterns[2].difference(digit).count() > 0 {
            return 0;
        } else {
            return 9;
        }
    }
}
