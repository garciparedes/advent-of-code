use std::io;
use std::io::prelude::*;
use std::collections::{
    HashMap, 
    HashSet,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let adjacency = build_adjacency(&buffer);

    let mut memory = HashMap::new();
    for first in adjacency.keys() {
        dfs(&adjacency, first, &mut memory);
    }

    let ans: usize = memory.values().filter(|v| **v).count();
    
    println!("{:?}", ans);

    return Ok(());
}

fn build_adjacency<'a>(buffer: &'a str) -> HashMap<&'a str, HashSet<(&'a str, usize)>> {
    let mut adjacency = HashMap::new();

    for row in buffer.trim().split("\n") {
        let mut kv = row.split("contain");
        let origin = kv.next().unwrap().split(" bag").next().unwrap().trim();
        let destinations: HashSet<_>= kv
            .next()
            .unwrap()
            .trim()
            .split(", ")
            .filter(|item| *item != "no other bags.")
            .map(|item| {
                let raw = item.split("bag").next().unwrap().trim();

                let mut i = 0;
                let mut iterable = raw.chars();
                while let Some(c) = iterable.next() {
                    if c.is_digit(10) {
                        break;
                    }
                    i += 1;
                }

                let (raw_count, label) = raw.split_at(i + 2);
                return (label, raw_count.trim().parse::<usize>().unwrap());
                
            })
            .collect();

        adjacency.insert(origin, destinations);

    }

    return adjacency;
}

fn dfs<'a>(
    adjacency: &HashMap<&'a str, HashSet<(&'a str, usize)>>, 
    key: &'a str, 
    memory: &mut HashMap<&'a str, bool>
) -> bool {
    if key == "shiny gold" {
        return true;
    }
    if let Some(&ans) = memory.get(&key) {
        return ans;
    }
    match adjacency.get(&key) {
        Some(destinations) => {
            let mut reachable = false;
            for (destination, _) in destinations.iter() {
                if dfs(adjacency, destination, memory) {
                    reachable = true;
                    break;
                }
            }
            memory.insert(key, reachable);
            return reachable;
        },
        None => false,
    }
}
