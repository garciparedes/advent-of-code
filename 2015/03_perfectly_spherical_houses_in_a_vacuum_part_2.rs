use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let (mut a, mut b) = ((0, 0), (0, 0));
    let mut visited: HashSet<(isize, isize)> = vec![a, b].into_iter().collect();

    for (i, direction) in buffer.trim().chars().enumerate() {
        if i % 2 == 0 { 
            match direction {
                '>' => {
                    a = (a.0 + 1, a.1);
                },
                '<' => {
                    a = (a.0 - 1, a.1);
                },
                '^' => {
                    a = (a.0, a.1 + 1);
                },
                _ => {
                    a = (a.0, a.1 - 1);
                },
            }
            visited.insert(a);
        } else {
            match direction {
                '>' => {
                    b = (b.0 + 1, b.1);
                },
                '<' => {
                    b = (b.0 - 1, b.1);
                },
                '^' => {
                    b = (b.0, b.1 + 1);
                }
                _ => {
                    b = (b.0, b.1 - 1);
                },
            }
            visited.insert(b);
        }
    }

    println!("{}", visited.len());

    return Ok(());
}
