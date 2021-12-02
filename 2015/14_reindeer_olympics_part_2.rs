use std::io;
use std::io::prelude::*;

const THRESHOLD: usize = 2503;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let reindeers: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let words: Vec<_> = line.trim().split_whitespace().collect();
            let speed = words[3].parse::<i32>().unwrap();
            let duration = words[6].parse::<i32>().unwrap();
            let sleep = words[13].parse::<i32>().unwrap();

            return (speed, duration, sleep);
        })
        .collect();

    let mut scores = vec![0; reindeers.len()];
    let mut positions = vec![0; reindeers.len()];
    let mut stamina: Vec<_> = reindeers.iter().map(|reindeer| reindeer.1).collect();

    for _ in 0..THRESHOLD {
        let mut best = 0;
        for j in 0..reindeers.len() {
            if stamina[j] == - reindeers[j].2 {
                stamina[j] = reindeers[j].1;
            }
            if stamina[j] > 0 {
                positions[j] += reindeers[j].0;
            } 
            if best < positions[j] {
                best = positions[j];
            }
            stamina[j] -= 1;
        }

        for j in 0..reindeers.len() {
            if positions[j] == best {
                scores[j] += 1;
            }
        }
    }

    let ans = scores.into_iter().max().unwrap();
    println!("{}", ans);

    return Ok(());
}

