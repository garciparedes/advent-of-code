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

            let mut computed_len = 0;
            let mut chars = (&literal[1..literal.len() - 1]).chars();
            while let Some(c1) = chars.next() {
                match c1 {
                    '\\' => {
                        let c2 = chars.next().unwrap();
                        match c2 {
                            '\\' | '\"' => computed_len += 1,
                            _ => {
                                chars.next().unwrap();
                                chars.next().unwrap();
                                computed_len += 1;
                            }
                        };
                    },
                    _ => computed_len += 1,
                };
            }
            return literal.len() - computed_len;
        })
        .sum();

    println!("{}", ans);

    return Ok(());
}
