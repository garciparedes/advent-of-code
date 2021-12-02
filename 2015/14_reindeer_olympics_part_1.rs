use std::io;
use std::io::prelude::*;
use std::cmp;

const THRESHOLD: u128 = 2503;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let words: Vec<_> = line.trim().split_whitespace().collect();
            let speed = words[3].parse::<u128>().unwrap();
            let duration = words[6].parse::<u128>().unwrap();
            let sleep = words[13].parse::<u128>().unwrap();

            return simulate(speed, duration, sleep);
        })
        .max()
        .unwrap();

    println!("{}", ans);

    return Ok(());
}

fn simulate(speed: u128, duration: u128, sleep: u128) -> u128 {
    let (mut position, mut current) = (0, 0);

    loop {
        let next = cmp::min(cmp::max(0, THRESHOLD - current), duration);
        if next == 0 {
            break;
        }
        position += speed * next;
        current = cmp::min(THRESHOLD, current + next + sleep);
    }

    return position;
}
