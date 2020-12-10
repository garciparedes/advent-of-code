use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut jolts: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|jolt| jolt.parse::<usize>().unwrap())
        .collect();

    jolts.sort_unstable();

    let mut counter = vec![0, 0, 1];
    counter[jolts[0] - 1] += 1;
    for i in 1..jolts.len() {
        counter[jolts[i] - (jolts[i - 1] + 1)] += 1;
    }

    let ans = counter[0] * counter[2];
    println!("{:?}", ans);


    return Ok(());
}
