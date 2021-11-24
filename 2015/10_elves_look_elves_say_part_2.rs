use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut current = buffer.trim().to_string();
    for _ in 0..50 {
        let mut new = String::new();

        let mut chars = current.chars().peekable();
        while let Some(c1) = chars.next() {
            let mut count = 1;
            while let Some(c2) = chars.peek() {
                if c1 != *c2 {
                    break;
                }
                count += 1;
                chars.next();
            }
            new.push_str(&format!("{}{}", count, c1));
        }

        current = new;
    }

    println!("{}", current.len());

    return Ok(());
}
