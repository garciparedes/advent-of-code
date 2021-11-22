use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans: usize = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let literal = line.trim();

            let mut computed_len = 2;
            let mut chars = literal.chars();
            while let Some(c1) = chars.next() {
                match c1 {
                    '\\' | '\"' => computed_len += 2,
                    _ => computed_len += 1,
                };
            }
            return computed_len - literal.len();
        })
        .sum();

    println!("{}", ans);

    return Ok(());
}
