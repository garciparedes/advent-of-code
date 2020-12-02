use std::io;
use std::io::prelude::*;


#[derive(Debug)]
struct Case {
    minimum: usize,
    maximum: usize,
    target: char,
    text: String,
}


impl Case {
    fn new(raw: &String) -> Self {
        let raw: Vec<char> = raw.chars().collect();

        let mut j: usize = 0;
        while raw[j] != '-' {
            j += 1;
        }
        let minimum = raw[..j].iter().collect::<String>().parse::<usize>().unwrap();
        
        j += 1;
        let i = j;
        while raw[j] != ' ' {
            j += 1;
        }
        let maximum = raw[i..j].iter().collect::<String>().parse::<usize>().unwrap();

        j += 1;
        let target = raw[j];
        

        j += 3;
        let text = raw[j..].iter().collect();

        Self {minimum: minimum, maximum: maximum, target: target, text: text}
    }

    fn is_valid(&self) -> bool {
        match (self.text.chars().nth(self.minimum - 1), self.text.chars().nth(self.maximum - 1)) {
            (Some(a), Some(b)) => (a == self.target) != (b == self.target),
            _ => false,
        }
    }
}


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let count = buffer
        .trim()
        .split("\n")
        .map(|raw| Case::new(&raw.to_string()))
        .filter(|case| case.is_valid())
        .count();

    println!("{}", count);
    

    return Ok(());
}
