use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

const STEPS: usize = 40;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let (raw_polymer, raw_replacements) = buffer
        .trim()
        .split_once("\n\n")
        .unwrap();

    let polymer:Vec<_> = raw_polymer
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

    let mut counter = HashMap::new();
    let mut memo = HashMap::new();
    for w in polymer.windows(2) {
        let (a, b) = (w[0], w[1]);
        let sub = rec(&replacements, a, b, STEPS, &mut memo);
        for (k, v) in sub {
            *counter.entry(k).or_insert(0) += v;
        }
    }
    for c in &polymer[1..polymer.len() - 1] {
        *counter.get_mut(c).unwrap() -= 1;
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

fn  rec(
    replacements: &HashMap<(char, char), char>, 
    a: char, 
    b: char, 
    depth: usize,
    memo: &mut HashMap<((char, char), usize), HashMap<char, usize>>
) -> HashMap<char, usize> {
    if depth == 0 || !replacements.contains_key(&(a, b)){
        if a == b { 
            return HashMap::from([(a, 2)]);
        } else { 
            return HashMap::from([(a, 1), (b, 1)]);
        }
    }

    if let Some(ans) = memo.get(&((a, b), depth)) {
        return ans.clone();
    }

    let &c = replacements.get(&(a,b)).unwrap();
    let mut aa = rec(replacements, a, c, depth - 1, memo);
    let bb = rec(replacements, c, b, depth - 1, memo);
    for (k, v) in bb {
        *aa.entry(k).or_insert(0) += v;
    }
    if let Some(w) = aa.get_mut(&c) {
        *w -= 1;
    }

    memo.insert(((a, b), depth), aa.clone());
    return aa;
}

