use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

static TURNS: usize = 30000000;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let initial: Vec<_> = buffer
        .trim()
        .split(',')
        .map(|raw| raw.parse::<usize>().unwrap())
        .collect();

    let mut previous: HashMap<_, _> = initial[..initial.len() - 1]
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, v)| (v, i))
        .collect();

    let mut last = initial[initial.len() - 1];
    for i in (initial.len() - 1)..(TURNS - 1) {
        if let Some(prevs) = previous.get_mut(&last) {
            last = i - *prevs;
            *prevs = i;
        } else {
            last = 0;
            previous.insert(last, i);
        }
    }

    let ans = last;
    println!("{:?}", ans);

    return Ok(());
}
