use std::io::prelude::*;
use std::io;
use std::collections::{
    HashSet,
    HashMap,
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Tile {
    id: usize,
    content: Vec<Vec<bool>>,
    borders: Vec<Vec<bool>>,
}

impl Tile {
    fn new(raw: &str) -> Self {
        let mut iterable = raw
            .trim()
            .split('\n');


        let id = if let Some(line) = iterable.next() {
            let mut parts = line.trim().split_whitespace();
            parts.next();
            if let Some(raw_number) = parts.next() {
                raw_number[..raw_number.len() - 1].parse::<usize>().unwrap()
            } else {
                panic!()
            }
        } else {
            panic!()
        };

        let content: Vec<_> = iterable
            .map(|line| line.trim().chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect();
        
        let top = content[0].clone();
        let right: Vec<_> = content.iter().map(|row| row[row.len() - 1]).collect();
        let bottom = content[content.len() - 1].clone();
        let left: Vec<_> = content.iter().map(|row| row[0]).collect();
        
        return Tile { id: id, content: content, borders: vec![top, right, bottom, left]};
    }

    fn matches_with(&self, another: &Tile) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if self.borders[i] == another.borders[j]
                    || self.borders[i] == another.borders[j].iter().cloned().rev().collect::<Vec<_>>()
                {
                    return true;
                } 
            }
        }
        return false;
    }

}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let tiles: Vec<_> = buffer
        .trim()
        .split("\n\n")
        .map(|raw| Tile::new(raw))
        .collect();
    
    
    let mut matches = HashMap::new(); 
    for i in 0..tiles.len() {
        let tile_i = &tiles[i];
        for j in (i + 1)..tiles.len() {
            let tile_j = &tiles[j];
            if tile_i.matches_with(tile_j) {
                matches.entry(tile_i).or_insert_with(HashSet::new).insert(tile_j);
                matches.entry(tile_j).or_insert_with(HashSet::new).insert(tile_i);
            }
        }
    }
    
    let ans: usize = matches
        .iter()
        .filter(|(_, v)| v.len() == 2)
        .map(|(k, _)| k.id)
        .product();

    println!("{:?}", ans);

    return Ok(());
}
