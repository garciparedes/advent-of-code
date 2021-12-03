use std::io;
use std::io::prelude::*;

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {

    fn from_line(line: &str) -> Ingredient {
        let (name, properties) = line.trim().split_once(": ").unwrap();
        let name = name.to_string();
        let properties: Vec<_> = properties
            .split(", ")
            .map(|v| v.split_whitespace().collect::<Vec<_>>()[1].parse::<i32>().unwrap())
            .collect();

        return Ingredient {
            name: name,
            capacity: properties[0],
            durability: properties[1],
            flavor: properties[2],
            texture: properties[3],
            calories: properties[4],
        }
    }
}


struct Cookie<'a> {
    index: &'a [Ingredient],
    quantities: Vec<i32>,
}


impl<'a> Cookie<'a> {
    fn from_ingredients(ingredients: &'a [Ingredient]) -> Cookie<'a> {
        return Cookie { index: ingredients, quantities: vec![0; ingredients.len()] };
    }

    fn len(&self) -> usize {
        return self.index.len();
    }

    fn optimize(&mut self, spoons: usize) -> i32 {
        return self.optimize_(spoons, 0)
    }
    fn optimize_(&mut self, spoons: usize, i: usize) -> i32 {
        if spoons == 0 {
            return self.score();
        }
        if i == self.len() {
            return 0;
        }

        let mut best = 0;
        for s in 0..=spoons {
            self.quantities[i] = s as i32;
            let a = self.optimize_(spoons - s, i + 1);
            if best < a {
                best = a;
            }
        }
        return best;
    }

    fn score(&self) -> i32 {
        let (mut capacity, mut durability, mut flavor, mut texture) = (0, 0, 0, 0);
        for i in 0..self.len() {
            capacity += self.quantities[i] * self.index[i].capacity;
            durability += self.quantities[i] * self.index[i].durability;
            flavor += self.quantities[i] * self.index[i].flavor;
            texture += self.quantities[i] * self.index[i].texture;
        }

        let mut total = 1;
        if capacity > 0 {
            total *= capacity;
        }
        if durability > 0 {
            total *= durability;
        }
        if flavor > 0 {
            total *= flavor;
        }
        if texture > 0 {
            total *= texture;
        }
        return total;
    }

}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ingredients: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(Ingredient::from_line)
        .collect();


    let mut cookie = Cookie::from_ingredients(&ingredients);
    let ans = cookie.optimize(100);

    println!("{:?}", ans);

    return Ok(());
}
