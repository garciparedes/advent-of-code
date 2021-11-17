use std::io;
use std::io::prelude::*;


fn main() -> io::Result<()> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let numbers: Vec<_> = line
                .trim()
                .split('x')
                .map(|v| v.parse::<u32>().unwrap())
                .collect();

            let numbers = vec![
                numbers[0] * numbers[1], 
                numbers[1] * numbers[2],
                numbers[0] * numbers[2],
            ];

            return 2 * numbers.iter().sum::<u32>() 
                + numbers.iter().min().unwrap();
        })
        .fold(0, |cum, v| cum + v);

    println!("{}", ans);

    return Ok(());
}
