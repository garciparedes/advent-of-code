use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.trim().split('\n');
    let base = lines.next().unwrap().parse::<u32>().unwrap();

    let (wait, id) = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .filter(|x| *x != "x")
        .map(|x| x.parse::<u32>().unwrap())
        .map(|x| ((x * (base as f32 / x as f32).ceil() as u32) - base, x))
        .min()
        .unwrap();

    let ans = wait * id;
    println!("{:?}", ans);


    return Ok(());
}
