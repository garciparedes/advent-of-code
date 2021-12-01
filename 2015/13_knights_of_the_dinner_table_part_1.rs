use std::io;
use std::io::prelude::*;
use std::collections::{ HashSet, HashMap };

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let edges: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let words: Vec<_> = line.trim().split_whitespace().collect();

            let origin = words[0].to_string();
            let last = words[words.len() - 1];
            let destination = last[..last.len() - 1].to_string();

            let mut happiness = words[3].parse::<i32>().unwrap();
            if words[2] == "lose" {
                happiness *= -1;
            }

            return (origin, destination, happiness);
        })
        .collect();

    let mut people = HashSet::new();
    for (origin, destination, _) in edges.iter() {
        people.insert(origin);
        people.insert(destination);
    }
    let index: HashMap<_, _> = people
        .into_iter()
        .enumerate()
        .map(|(i, name)| (name, i))
        .collect();

    let mut adjacency = vec![vec![0; index.len()]; index.len()];
    for (origin, destination, happiness) in edges.iter() {
        adjacency[*index.get(origin).unwrap()][*index.get(destination).unwrap()] = *happiness;
    }
    
    let mut visited = vec![false; index.len()];
    let mut ans = i32::MIN;
    visited[0] = true;
    dfs(&adjacency, &mut visited, 0, 0, &mut ans);
    
    println!("{}", ans);

    return Ok(());
}


fn dfs(adjacency: &Vec<Vec<i32>>, visited: &mut [bool], i: usize, current: i32, best: &mut i32) {
    let path = current + adjacency[i][0] + adjacency[0][i];
    if visited.iter().all(|v| *v) && path > *best {
        *best = path;
    }

    for j in 0..visited.len() {
        if !visited[j] {
            visited[j] = true;
            dfs(adjacency, visited, j, current + adjacency[i][j] + adjacency[j][i],best);
            visited[j] = false;
        }
    }
}
