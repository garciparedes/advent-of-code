use std::io::prelude::*;
use std::io;

const SUBJECT_NUMBER: usize  = 7;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let public_keys: Vec<_>  = buffer
        .trim()
        .split('\n')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();


    let loop_sizes: Vec<_> = public_keys
        .iter()
        .map(|&public_key| find(public_key))
        .collect();


    let ans = transform(loop_sizes[0], public_keys[1]);
    println!("{:?}", ans);

    return Ok(());
}


fn find(public_key: usize) -> usize {
    let mut loop_size = 1;
    let mut value = 1;
    loop {
        value *= SUBJECT_NUMBER;
        value = value % 20201227;
        if value == public_key {
            return loop_size;
        }
        loop_size += 1;

    }
}

fn transform(loop_size: usize, subject: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_size {
        value *= subject;
        value = value % 20201227;
    }
    return value;
}
