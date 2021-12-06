use std::io;
use std::io::prelude::*;

const DAYS: i32 = 80;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let current: Vec<i32> = buffer
        .trim()
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();

    let mut ans = 0;
    for c in current {
        ans += simulate(DAYS, c);
    }

    println!("{}", ans);

    return Ok(());
}

fn simulate(days: i32, state: i32) -> i32 {
    if days <= state {
        return 1;
    }

    return simulate(days - (state + 1), 6) + simulate(days - (state + 1), 8);
}
