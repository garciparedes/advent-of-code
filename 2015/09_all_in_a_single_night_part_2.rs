use std::io;
use std::io::prelude::*;
use std::collections::{ HashMap, HashSet };

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut adjacency = HashMap::new();
    for line in buffer.trim().split('\n') {
        let (raw_pair, raw_dist) = line.split_once(" = ").unwrap();
        let pair = raw_pair.split_once(" to ").unwrap();
        let dist = raw_dist.parse::<u32>().unwrap();

        adjacency.entry(pair.0).or_insert_with(HashSet::new).insert((pair.1, dist));
        adjacency.entry(pair.1).or_insert_with(HashSet::new).insert((pair.0, dist));
    }

    let ans = adjacency
        .keys()
        .map(|initial| { 
            let mut visited = HashSet::from([*initial]);
            return dp(&adjacency, &mut visited, initial);
        })
        .max()
        .unwrap();

    println!("{}", ans);

    return Ok(());
}


fn dp<'a>(
    adjacency: &HashMap<&'a str, HashSet<(&'a str, u32)>>, visited: &mut HashSet<&'a str>, current: &'a str
) -> u32 {
    if visited.len() == adjacency.len() {
        return 0;
    }
    let mut best = u32::MIN;

    if let Some(options) = adjacency.get(current) {
        for (next, dist) in options.iter() {
            if visited.contains(next) {
                continue;
            }
            visited.insert(next);
            let d = dist + dp(adjacency, visited, next);
            visited.remove(next);
            if d > best {
                best = d;
            }
        } 
    }
    return best;
}
