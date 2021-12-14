use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

const STEPS: usize = 10;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let (raw_polymer, raw_replacements) = buffer
        .trim()
        .split_once("\n\n")
        .unwrap();

    let mut polymer:Vec<_> = raw_polymer
        .trim()
        .chars()
        .collect();

    let replacements: HashMap<_, _> = raw_replacements
        .trim()
        .split('\n')
        .map(|line| {
            let (k, v) = line.trim().split_once(" -> ").unwrap();
            let mut ks = k.trim().chars();
            let mut vs = v.trim().chars();

            let (k1, k2) = (ks.next().unwrap(), ks.next().unwrap());

            return ((k1, k2), vs.next().unwrap());
        })
        .collect();

    for _ in 0..STEPS {
        polymer = process(&replacements, polymer);
    }

    let mut counter = HashMap::new();
    for v in polymer {
        *counter.entry(v).or_insert(0) += 1;
    }

    let (less, most) = counter
        .values()
        .cloned()
        .fold((usize::MAX, usize::MIN), |mut cum, curr| {
            if curr < cum.0 {
                cum = (curr, cum.1);
            }
            if curr > cum.1 {
                cum = (cum.0, curr);
            }
            return cum;
        });

    let ans = most - less;
    println!("{}", ans);

    return Ok(());
}

fn process(replacements: &HashMap<(char, char), char>, previous: Vec<char>) -> Vec<char> {
    let mut new = vec![previous[0]];
    for &v1 in previous[1..].into_iter() {
        if let Some(&v2) = replacements.get(&(new[new.len() - 1], v1)) {
            new.push(v2);
        }
        new.push(v1);
    }
    return new;
}
