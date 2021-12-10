use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut scores: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let mut stack = Vec::new();
            let mut corrupted = false;
            for c in line.trim().chars() {
                match c {
                    ')' => {
                        if let Some('(') = stack.last() {
                            stack.pop();
                        } else {
                            corrupted = true;
                            break;
                        }
                    },
                    ']' => {
                        if let Some('[') = stack.last() {
                            stack.pop();
                        } else {
                            corrupted = true;
                            break;
                        }
                    },
                    '}' => {
                        if let Some('{') = stack.last() {
                            stack.pop();
                        } else {
                            corrupted = true;
                            break;
                        }
                    },
                    '>' => {
                        if let Some('<') = stack.last() {
                            stack.pop();
                        } else {
                            corrupted = true;
                            break;
                        }
                    },
                    _ => stack.push(c),
                }

            if corrupted {
                return 0;
            }
            
            let mut score: u128 = 0;
            for s in stack.into_iter().rev() {
                score *= 5;
                match s {
                    '(' => score += 1,
                    '[' => score += 2,
                    '{' => score += 3,
                    '<' => score += 4,
                    _ => panic!(),
                }
            }
            return score;
        })
        .collect();

    scores.sort_unstable();
    let ans = scores[scores.len() / 2];

    println!("{}", ans);

    return Ok(());
}
