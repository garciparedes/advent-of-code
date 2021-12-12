use std::io;
use std::io::prelude::*;
use std::collections::{ HashMap, HashSet };

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut adjacency = HashMap::new();
    for line in buffer.trim().split('\n') {
        let (a, b) = line.trim().split_once('-').unwrap();
        adjacency.entry(a).or_insert_with(HashSet::new).insert(b);
        adjacency.entry(b).or_insert_with(HashSet::new).insert(a);
    }

    let mut visited = HashSet::new();

    
    let ans = dfs(&adjacency, &mut visited, "start");
    println!("{}", ans);

    return Ok(());
}


fn dfs<'a>(adjacency: &HashMap<&'a str, HashSet<&'a str>>, visited: &mut HashSet<&'a str>, current: &'a str) -> usize {
    if visited.contains(current) {
        return 0;
    }
    if current == "end" {
        return 1;
    }

    if current.chars().next().unwrap().is_lowercase() {
        visited.insert(current);
    }

    let mut count = 0;
    if let Some(destinations) = adjacency.get(current) {
        for destination in destinations {
            count += dfs(adjacency, visited, destination);
        } 
    }

    visited.remove(current);
    return count;
}
