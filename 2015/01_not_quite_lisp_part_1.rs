use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans: i32 = buffer.trim().chars().fold(0, |sum, c| if c == '(' { sum + 1 } else { sum - 1 });
    println!("{}", ans);

    return Ok(());
}

