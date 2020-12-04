use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

struct Entry<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Entry<'a> {

    fn new(raw: &'a str) -> Self {

        let fields: HashMap<_, _> = raw
            .split_whitespace()
            .map(|pair| {
                let kv: Vec<_> = pair.split(':').collect();
                return (kv[0], kv[1]);
            })
            .collect();

        return Self { fields: fields };
    }

    fn is_valid(&self) -> bool {
        self.fields.contains_key(&"byr")
        && self.fields.contains_key(&"byr")
        && self.fields.contains_key(&"iyr")
        && self.fields.contains_key(&"eyr")
        && self.fields.contains_key(&"hgt")
        && self.fields.contains_key(&"hcl")
        && self.fields.contains_key(&"ecl")
        && self.fields.contains_key(&"pid")
    }
}


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim().split("\n\n")
        .map(|raw| Entry::new(raw))
        .filter(|entry| entry.is_valid())
        .count();
    
    println!("{:?}", ans);

    return Ok(());
}

