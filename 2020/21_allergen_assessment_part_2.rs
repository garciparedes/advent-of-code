use std::io::prelude::*;
use std::io;
use std::cmp;
use std::collections::{
    HashSet,
    HashMap,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;


    let entries: Vec<(HashSet<&str>, HashSet<&str>)> = buffer
        .trim()
        .split('\n')
        .map(|line| {
            let mut parts = line[..line.len() - 1].split(" (contains ");
            let ingredients: HashSet<_> = parts
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .collect();

            let allergens: HashSet<_> = parts
                .next()
                .unwrap()
                .trim()
                .split(", ")
                .collect();

            return (ingredients, allergens);
        })
        .collect();

    let mut data = HashMap::new();
    for (ings, alls) in entries.clone() {
        for all in alls.iter() {
            data.entry(all.clone()).or_insert_with(Vec::new).push(ings.clone());
        }
    }
    let mut data: Vec<(_, _)> = data.into_iter().collect();
    data.sort_unstable_by_key(|(_, v)| cmp::Reverse(v.iter().map(|x| x.len()).sum::<usize>()));

    let mut new_data = Vec::new();
    for i in 0..data.len() {
        let mut iterable = data[i].1.iter();
        let mut intersect: HashSet<_> = iterable.next().unwrap().clone();
        while let Some(another) = iterable.next() {
            intersect = intersect.intersection(&another).cloned().collect();
        }
        new_data.push((data[i].0, intersect));
    }

    new_data.sort_unstable_by_key(|(_, v)| v.len());
    let mut mapper = HashMap::new();
    for _ in 0..new_data.len() {
        let mut i = 0;
        while new_data[i].1.len() == 0 {
            i += 1;
        }
        let key = new_data[i].0;
        let values = new_data[i].1.clone();
        if !values.len() == 1 {
            panic!();
        }
        let value = values.into_iter().next().unwrap();

        for j in 0..new_data.len() {
            new_data[j].1.remove(value.clone());
        }
        new_data.sort_unstable_by_key(|(_, v)| v.len());
        mapper.insert(key, value);
    }

    let mut mapper: Vec<_> = mapper.into_iter().collect();
    mapper.sort();

    let ans = mapper
        .into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
        .join(",");

    println!("{}", ans);

    return Ok(());
}
