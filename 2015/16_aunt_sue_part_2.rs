use std::io;
use std::io::prelude::*;
use std::convert::From;
use std::collections::HashMap;

#[derive(Debug)]
struct Aunt {
    name: Option<i32>,
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

impl Aunt {
    fn new() -> Self {
        Aunt {
            name: None,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }

    fn filter<'a>(items: &'a [Self], target: Self) -> Vec<&'a Self> {
        items
            .iter()
            .filter(|item| {
                if let (Some(obs), Some(exp)) = (item.children, target.children) {
                    if exp != obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.cats, target.cats) {
                    if exp > obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.samoyeds, target.samoyeds) {
                    if exp != obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.pomeranians, target.pomeranians) {
                    if exp < obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.akitas, target.akitas) {
                    if exp != obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.vizslas, target.vizslas) {
                    if exp != obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.goldfish, target.goldfish) {
                    if exp < obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.trees, target.trees) {
                    if exp > obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.cars, target.cars) {
                    if exp != obs {
                        return false;
                    }
                }
                if let (Some(obs), Some(exp)) = (item.perfumes, target.perfumes) {
                    if exp != obs {
                        return false;
                    }
                }
                return true;
            })
            .collect()
    }
}

impl From<&str> for Aunt {

    fn from(item: &str) -> Self {
        let (raw_name, raw_parts) = item
            .split_once(": ")
            .unwrap();

        let name = raw_name
            .split_once(' ')
            .unwrap()
            .1
            .parse::<i32>()
            .unwrap();

        let parts: HashMap<_, _> = raw_parts
            .trim()
            .split(", ")
            .map(|part| {
                let (name, value) = part.split_once(": ").unwrap();
                return (name, value.parse::<i32>().unwrap());
            })
            .collect();

        let mut instance = Self::new();
        instance.name = Some(name);
        instance.children = parts.get("children").cloned();
        instance.cats = parts.get("cats").cloned();
        instance.samoyeds = parts.get("samoyeds").cloned();
        instance.pomeranians = parts.get("pomeranians").cloned();
        instance.akitas = parts.get("akitas").cloned();
        instance.vizslas = parts.get("vizslas").cloned();
        instance.goldfish = parts.get("goldfish").cloned();
        instance.trees = parts.get("trees").cloned();
        instance.cars = parts.get("cars").cloned();
        instance.perfumes = parts.get("perfumes").cloned();
        return instance;
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut target = Aunt::new();
    target.children = Some(3);
    target.cats = Some(7);
    target.samoyeds = Some(2);
    target.pomeranians = Some(3);
    target.akitas = Some(0);
    target.vizslas = Some(0);
    target.goldfish = Some(5);
    target.trees = Some(3);
    target.cars = Some(2);
    target.perfumes = Some(1);

    let aunts: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(Aunt::from)
        .collect();

    let filtered = Aunt::filter(&aunts, target);

    let ans = filtered[0].name.unwrap();
    println!("{:?}", ans);

    return Ok(());
}
