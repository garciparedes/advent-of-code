use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

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

    let mut counts = HashMap::new();
    optimize(&containers, LITERS, 0, &mut counts);

    let ans = counts
        .into_iter()
        .min()
        .unwrap()
        .1;

    println!("{}", ans);

    return Ok(());
}

fn optimize(containers: &[i32], current: i32, count: i32, ans: &mut HashMap<i32, i32>) {
    if current == 0 {
        *ans.entry(count).or_insert(0) += 1;
        return;
    }
    if current < 0 || containers.len() == 0 {
        return;
    }

    optimize(&containers[1..], current, count, ans);
    optimize(&containers[1..], current - containers[0], count + 1, ans);
}
