use std::io;
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let mut numbers: Vec<_> = line
                .trim()
                .split('x')
                .map(|v| v.parse::<u32>().unwrap())
                .collect();
            numbers.sort_unstable();

            return 2 * (numbers[0] + numbers[1]) 
                + numbers.iter().fold(1, |cum, v| cum * v);
        } )
        .fold(0, |cum, v| cum + v);

    println!("{}", ans);

    return Ok(());
}
