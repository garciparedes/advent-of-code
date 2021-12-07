use std::io;
use std::io::prelude::*;

const LITERS: i32 = 150; 

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut containers: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    containers.sort();

    let ans = optimize(&containers, LITERS);
    println!("{}", ans);

    return Ok(());
}

fn optimize(containers: &[i32], current: i32) -> i32 {
    if current == 0 {
        return 1;
    }
    if current < 0 || containers.len() == 0 {
        return 0;
    }

    let mut best = 0;

    best += optimize(&containers[1..], current);
    best += optimize(&containers[1..], current - containers[0]);

    return best;
}
