use std::io::prelude::*;
use std::io;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines: Vec<_> = buffer.trim().split('\n').map(|l| l.to_string()).collect();
    
    let mut acc = -1;
    for i in 0..lines.len() {
        if lines[i].starts_with("jmp") {
            lines[i] = lines[i].replacen("jmp", "nop", 1);
            if let Some(v) = execute(&lines) {
                acc = v;
                break;
            } else {
                lines[i] = lines[i].replacen("nop", "jmp", 1);
            }

        } else if lines[i].starts_with("nop") {
            lines[i] = lines[i].replacen("nop", "jmp", 1);
            if let Some(v) = execute(&lines) {
                acc = v;
                break;
            } else {
                lines[i] = lines[i].replacen("jmp", "nop", 1);
            }
        }
    }
    
    println!("{:?}", acc);

    return Ok(());
}

fn execute(lines: &[String]) -> Option<i32> {
    let mut acc = 0;
    let mut visited = HashSet::new();
    let mut index = 0;
    visited.insert(index);
    while index < lines.len() {
        let mut parts = lines[index].trim().split_whitespace();
        match (parts.next(), parts.next()) {
            (Some("acc"), Some(raw)) => {
                index += 1;
                if visited.contains(&index) {
                    return None;
                }
                acc += raw.parse::<i32>().unwrap();
            } 
            (Some("jmp"), Some(raw)) => {
                let offset = raw.parse::<isize>().unwrap();
                index = (index as isize + offset) as usize;
            },
            (Some("nop"), _) => index += 1,
            _ => panic!(),
        }
        if !visited.insert(index) {
            return None;
        }
    }

    return Some(acc) 
}
