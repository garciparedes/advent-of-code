use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let depths: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .windows(3)
        .map(|triplet| triplet.into_iter().sum::<i32>())
        .collect();

    let mut ans = 0;
    for tuple in depths.windows(2) {
        if tuple[0] < tuple[1] {
            ans += 1;
        }
    }

    println!("{}", ans);

    return Ok(());
}
