use std::io;
use std::io::prelude::*;
use md5;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let secret = buffer.trim();
    
    let mut ans = 1;
    let mut value = format!("{}{}", secret, ans);
    let mut digest = md5::compute(&value);
    while !format!("{:x}", digest).starts_with("00000") {
        ans += 1;
        value = format!("{}{}", secret, ans);
        digest = md5::compute(&value);
    }

    println!("{}", ans);

    return Ok(());
}
