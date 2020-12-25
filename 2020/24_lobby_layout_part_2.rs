use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

const STEPS: usize = 100;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let iterable = buffer
        .trim()
        .split('\n')
        .map(|raw| Direction::new_chain(raw))
        .map(|chain| Direction::reduce(&chain));

    let mut board = HashMap::new();
    for coordinate in iterable {
        *board.entry(coordinate).or_insert(false) ^= true;
    }


    for _ in 0..STEPS {
        let keys: Vec<_> = board.keys().cloned().collect();
        for tile in keys {
            if !board.contains_key(&vec![tile[0], tile[1] + 1, tile[2] - 1]) {
                board.insert(vec![tile[0], tile[1] + 1, tile[2] - 1], false);
            }
            if !board.contains_key(&vec![tile[0], tile[1] - 1, tile[2] + 1]) {
                board.insert(vec![tile[0], tile[1] - 1, tile[2] + 1], false);
            }
            if !board.contains_key(&vec![tile[0] - 1, tile[1], tile[2] + 1]) {
                board.insert(vec![tile[0] - 1, tile[1], tile[2] + 1], false);
            }
            if !board.contains_key(&vec![tile[0] + 1, tile[1], tile[2] - 1]) {
                board.insert(vec![tile[0] + 1, tile[1], tile[2] - 1], false);
            }
            if !board.contains_key(&vec![tile[0] - 1, tile[1] + 1, tile[2]]) {
                board.insert(vec![tile[0] - 1, tile[1] + 1, tile[2]], false);
            }
            if !board.contains_key(&vec![tile[0] + 1, tile[1] - 1, tile[2]]) {
                board.insert(vec![tile[0] + 1, tile[1] - 1, tile[2]], false);
            }
        }

        let mut new = board.clone();
        for (tile, state) in board.iter_mut() {
            let count = count_black(&mut new, tile);
            if *state && (count == 0 || count > 2) {
                *state = false;
            } else if !*state && count == 2 {
                *state = true;
            }
        }

    }

    let ans: usize = board
        .into_iter()
        .filter(|(_, v)| *v)
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

fn count_black(board: &mut HashMap<Vec<isize>, bool>, current: &[isize]) -> usize {
    let mut ans = 0;

    if *board.get(&vec![current[0], current[1] + 1, current[2] - 1]).unwrap_or(&false) {
        ans += 1;
    }

    if *board.get(&vec![current[0], current[1] - 1, current[2] + 1]).unwrap_or(&false) {
        ans += 1;
    }

    if *board.get(&vec![current[0] + 1, current[1], current[2] - 1]).unwrap_or(&false) {
        ans += 1;
    }

    if *board.get(&vec![current[0] - 1, current[1], current[2] + 1]).unwrap_or(&false) {
        ans += 1;
    }

    if *board.get(&vec![current[0] + 1, current[1] - 1, current[2]]).unwrap_or(&false) {
        ans += 1;
    }

    if *board.get(&vec![current[0] - 1, current[1] + 1, current[2]]).unwrap_or(&false) {
        ans += 1;
    }

    

    return ans;
}
