use std::io::prelude::*;
use std::io;
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut sections = buffer
        .trim()
        .split("\n\n")
        .map(|part| {
            part
                .trim()
                .split('\n')
                .skip(1)
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<VecDeque<_>>()
        });

    let (mut alice, mut bob) = (sections.next().unwrap(), sections.next().unwrap());

    while !alice.is_empty() && !bob.is_empty() {
        let (alice_front, bob_front) = (alice.pop_front().unwrap(), bob.pop_front().unwrap());
        if alice_front < bob_front {
            bob.push_back(bob_front);
            bob.push_back(alice_front);
        } else {
            alice.push_back(alice_front);
            alice.push_back(bob_front);
        }
    }

    let winner = if !alice.is_empty() { alice } else { bob };

    let ans: i32 = winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) as i32 * v)
        .sum();

    println!("{}", ans);

    return Ok(());
}
