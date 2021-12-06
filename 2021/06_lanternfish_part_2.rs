use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

const DAYS: i128 = 256;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let current: Vec<i128> = buffer
        .trim()
        .split(',')
        .map(|v| v.parse::<i128>().unwrap())
        .collect();

    let mut memo = HashMap::new();

    let mut ans = 0;
    for c in current {
        ans += simulate(DAYS, c, &mut memo);
    }

    println!("{}", ans);

    return Ok(());
}

fn simulate(days: i128, state: i128, memo: &mut HashMap<(i128, i128), i128>) -> i128 {
    if let Some(&ans) = memo.get(&(days, state)) {
        return ans;
    }

    if days <= state {
        return 1;
    }

    let ans = simulate(days - (state + 1), 6, memo) + simulate(days - (state + 1), 8, memo);
    memo.insert((days, state), ans);
    return ans;
}
