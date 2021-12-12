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

    let wildcards = adjacency
        .keys()
        .cloned()
        .filter(|&v| v != "start" && v != "end" && v.chars().next().unwrap().is_lowercase())
        .map(Some);

    let mut paths = HashSet::new();
    for wildcard in wildcards {
        dfs(&adjacency, &mut visited, &["start"], wildcard, &mut paths);
    }
    let ans = paths.len();
    println!("{}", ans);

    return Ok(());
}


fn dfs<'a>(
    adjacency: &HashMap<&'a str, HashSet<&'a str>>, 
    visited: &mut HashSet<&'a str>, 
    current: &[&'a str], 
    mut wildcard: Option<&'a str>,
    paths: &mut HashSet<Vec<&'a str>>
) {
    let origin = current[current.len() - 1];

    if origin == "end" {
        paths.insert(current.to_vec());
        return;
    }

    if visited.contains(origin) {
        return;
    }

    if origin.chars().next().unwrap().is_lowercase() {
        if let Some(v) = wildcard {
            if v == origin {
                wildcard = None;
            } else {
                visited.insert(origin);
            }
        } else {
            visited.insert(origin);
        }
    }

    if let Some(destinations) = adjacency.get(origin) {
        for destination in destinations {
            dfs(adjacency, visited, &[current, &[destination]].concat(), wildcard, paths);
        } 
    }

    visited.remove(origin);
}
