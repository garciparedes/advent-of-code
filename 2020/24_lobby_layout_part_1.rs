use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let iterable = buffer
        .trim()
        .split('\n')
        .map(|raw| Direction::new_chain(raw))
        .map(|chain| Direction::reduce(&chain));

    let mut counter = HashMap::new();
    for coordinate in iterable {
        *counter.entry(coordinate).or_insert(0) += 1;
    }

    let ans: usize = counter
        .into_iter()
        .filter(|(_, v)| v % 2 == 1)
        .count();

    println!("{}", ans);

    return Ok(());
}


#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {

    fn new_chain(raw: &str) -> Vec<Self>{
        let mut ans = Vec::new();

        let mut chars = raw.trim().chars();

        while let Some(c1) = chars.next() {
            let direction = match c1 {
                'e' => Self::East,
                'w' => Self::West,
                _ => {
                    if let Some(c2) = chars.next() {
                        match (c1, c2) {
                            ('s', 'e') => Self::SouthEast, 
                            ('s', 'w') => Self::SouthWest, 
                            ('n', 'e') => Self::NorthEast, 
                            ('n', 'w') => Self::NorthWest, 
                            _ => panic!(),
                        }
                    } else {
                        panic!()
                    }
                } 
            };
            ans.push(direction);
        }


        return ans;
    }
    fn reduce(chain: &[Self]) -> Vec<isize> {
        let mut current = vec![0, 0, 0];
        
        for direction in chain {
            match direction {
                Self::East => {
                    current[0] -= 1;
                    current[1] += 1;
                },
                Self::West=> {
                    current[0] += 1;
                    current[1] -= 1;
                },
                Self::NorthEast => {
                    current[1] += 1;
                    current[2] -= 1;
                },
                Self::NorthWest => {
                    current[0] += 1;
                    current[2] -= 1;
                },
                Self::SouthEast => {
                    current[0] -= 1;
                    current[2] += 1;
                },
                Self::SouthWest => {
                    current[1] -= 1;
                    current[2] += 1;
                },
            }
        }

        return current;
    }
}
