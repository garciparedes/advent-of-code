
use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

struct Group<'a> {
    raw: &'a str,
}

impl<'a> Group<'a> {

    fn new(raw: &'a str) -> Self {
        return Self { raw: raw };
    }

    fn distinct_count(&self) -> usize {
        let mut distinct = HashSet::new();
        for c in self.raw.chars() {
            if !c.is_ascii_alphabetic() {
                continue;
            }
            distinct.insert(c);
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
        .map(|group| group.distinct_count())
        .sum();
    
    println!("{:?}", ans);

    return Ok(());
}

