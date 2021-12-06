use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let parts: Vec<_> = buffer
        .trim()
        .split("\n\n")
        .collect();

    let numbers: Vec<_> = parts[0]
        .trim()
        .split(',')
        .map(|v| v.parse::<i32>().unwrap())
        .collect();
    
    let mut boards: Vec<_> = parts[1..]
        .iter()
        .map(|raw| {
            let numbers = raw
                .trim()
                .split('\n')
                .map(|line| {
                    line
                        .trim()
                        .split_whitespace()
                        .map(|v| v.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let mut hashes = vec![HashSet::new(); 10];
            for i in 0..5 {
                for j in 0..5 {
                    hashes[i].insert(numbers[i][j]);
                    hashes[5 + j].insert(numbers[i][j]);
                }
            }
            return hashes;
        })
        .collect();

    let mut ans = None;
    for number in &numbers {
        for board in boards.iter_mut() {
            let mut win = false;
            for h in board.iter_mut() {
                h.remove(number);
                if h.len() == 0 {
                    win = true;
                }
            }
            if win {
                let unconsumed = board
                    .iter()
                    .fold(HashSet::new(), |data, another| {
                        data.union(another).cloned().collect()
                    })
                    .into_iter()
                    .sum::<i32>();

                ans = Some(number * unconsumed);
            }
        }
        if ans.is_some() {
            break;
        }
    }

    println!("{}", ans.unwrap());

    return Ok(());
}

