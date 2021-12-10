use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans: usize = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let mut stack = Vec::new();
            let mut score = 0;
            for c in line.trim().chars() {
                match c {
                    ')' => {
                        if let Some('(') = stack.last() {
                            stack.pop();
                        } else {
                            score = 3;
                            break;
                        }
                    },
                    ']' => {
                        if let Some('[') = stack.last() {
                            stack.pop();
                        } else {
                            score = 57;
                            break;
                        }
                    },
                    '}' => {
                        if let Some('{') = stack.last() {
                            stack.pop();
                        } else {
                            score = 1197;
                            break;
                        }
                    },
                    '>' => {
                        if let Some('<') = stack.last() {
                            stack.pop();
                        } else {
                            score = 25137;
                            break;
                        }
                    },
                    _ => stack.push(c),
                }

            }
            return score;
        })
        .sum();

    println!("{}", ans);

    return Ok(());
}
