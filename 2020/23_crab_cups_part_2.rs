use std::io::prelude::*;
use std::io;

const STEPS: usize = 10_000_000;
const SIZE: usize = 1_000_000;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut cups: Vec<_> = buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    
    for i in cups.len() + 1..=SIZE {
        cups.push(i as usize);
    }

    let (one, two) = simulate(&cups, STEPS);
    let ans = one * two;
    println!("{}", ans);

    return Ok(());
}

fn simulate(cups: &[usize], steps: usize) -> (usize, usize) {
    let mut next = vec![0; cups.len() + 1];
    for i in 0..cups.len() - 1 {
        next[cups[i]] = cups[i + 1];
    }
    next[cups[cups.len() - 1]] = cups[0];

    let mut current = cups[0];
    for _ in 0..steps {
        let mut pickup = Vec::new();
        let mut tmp = current;
        for _ in 0..3 {
            tmp = next[tmp];
            pickup.push(tmp);
        }

        let mut new;
        let mut i = 1;
        loop {
            new = if current > i { current - i } else { cups.len() + current - i };
            if !pickup.contains(&new) {
                break;
            }
            i += 1;
        }

        next.swap(new, current);
        next.swap(current, pickup[pickup.len() - 1]);

        current = next[current];
    }
    let one = next[1];
    let two = next[one];
    return (one, two);
}
