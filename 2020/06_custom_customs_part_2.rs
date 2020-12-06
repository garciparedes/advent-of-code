
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z',
];

struct Group<'a> {
    raw: &'a str,
}

impl<'a> Group<'a> {

    fn new(raw: &'a str) -> Self {
        return Self { raw: raw };
    }

    fn intersection_count(&self) -> usize {
        let mut distinct: HashSet<_> = ASCII_LOWER.iter().cloned().collect();
        
        for line in self.raw.trim().split("\n") {
            let mut current = HashSet::new();
            for c in line.chars() {
                if !c.is_ascii_alphabetic() {
                    continue;
                }
                current.insert(c);
            }
            distinct = distinct.intersection(&current).cloned().collect();

        }
        return distinct.len();
    }
}


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans: usize = buffer
        .trim().split("\n\n")
        .map(|raw| Group::new(raw))
        .map(|group| group.intersection_count())
        .sum();
    
    println!("{:?}", ans);

    return Ok(());
}

