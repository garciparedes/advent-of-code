use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut current = (0, 0);
    let mut visited: HashSet<(isize, isize)> = vec![current].into_iter().collect();

    for direction in buffer.trim().chars() {
        match direction {
            '>' => {
                current = (current.0 + 1, current.1);
            },
            '<' => {
                current = (current.0 - 1, current.1);
            },
            '^' => {
                current = (current.0, current.1 + 1);
            },
            _ => {
                current = (current.0, current.1 - 1);
            },
        }
        visited.insert(current);
    }

    println!("{}", visited.len());

    return Ok(());
}
