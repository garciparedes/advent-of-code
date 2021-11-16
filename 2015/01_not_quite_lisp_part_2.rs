use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut ans: usize = 0;
    let mut sum: i32 = 0;
    for (i, c) in buffer.trim().chars().enumerate() {
        if c == '(' {
            sum += 1;
        } else {
            sum -= 1;
        }
        if sum < 0 {
            ans = i + 1;
            break;
        }
    }
    println!("{}", ans);

    return Ok(());
}

