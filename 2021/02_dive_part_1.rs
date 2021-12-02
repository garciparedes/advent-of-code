use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let (mut x, mut y) = (0, 0);
    for line in buffer.trim().split('\n') {
        let words: Vec<_> = line.trim().split_whitespace().collect();
        let offset = words[1].parse::<i32>().unwrap();
        match words[0] {
            "forward" => x += offset,
            "up" => y -= offset,
            _ => y += offset,
        }
    }

    let ans = x * y;
    println!("{}", ans);

    return Ok(());
}
