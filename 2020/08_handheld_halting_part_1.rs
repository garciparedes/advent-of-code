use std::io::prelude::*;
use std::io;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;


    let mut acc = 0;
    let mut visited = HashSet::new();
    let mut index = 0;
    visited.insert(index);

    let lines: Vec<_> = buffer.trim().split('\n').collect();


    loop {
        let mut parts = lines[index].trim().split_whitespace();
        match (parts.next(), parts.next()) {
            (Some("acc"), Some(raw)) => {
                index += 1;
                if visited.contains(&index) {
                    break;
                }
                let offset = raw.parse::<isize>().unwrap();
                acc += offset;
            } 
            (Some("jmp"), Some(raw)) => {
                let offset = raw.parse::<isize>().unwrap();
                index = (index as isize + offset) as usize;
            },
            (Some("nop"), _) => index += 1,
            _ => panic!(),
        }
        if !visited.insert(index) {
            break;
        }
    }

    println!("{:?}", acc);

    return Ok(());
}
