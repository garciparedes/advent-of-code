use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let (raw_dots, raw_folds) = buffer
        .trim()
        .split_once("\n\n")
        .unwrap();

    let mut dots: HashSet<_> = raw_dots
        .trim()
        .split('\n')
        .map(|line| {
            let (raw_x, raw_y) = line.split_once(',').unwrap();
            let (x, y) = (raw_x.parse::<isize>().unwrap(), raw_y.parse::<isize>().unwrap());
            return (x, y);
        })
        .collect();

    let folds: Vec<_> = raw_folds
        .trim()
        .split('\n')
        .map(|line| {
            let mut words = line.trim().split_whitespace();
            words.next();
            words.next();

            let (raw_axis, raw_index) = words.next().unwrap().split_once('=').unwrap();
            let axis = if raw_axis == "x" { 0 } else { 1 };
            let index = raw_index.parse::<isize>().unwrap();

            return (axis, index);
        })
        .collect();

    dots = fold(dots, folds[0]);

    let ans = dots.len();
    println!("{}", ans);

    return Ok(());
}

fn fold(previous: HashSet<(isize, isize)>, fold: (isize, isize)) -> HashSet<(isize, isize)> {
    let mut new = HashSet::new();
    for (x, y) in previous {
        if fold.0 == 0 {
            if x < fold.1 {
                new.insert((x, y));
            } else if x > fold.1 {
                new.insert((2 * fold.1 - x, y));
            }
            
        } else {
            if y < fold.1 {
                new.insert((x, y));
            } else if y > fold.1 {
                new.insert((x, 2 * fold.1 - y));
            } 

        }
    }

    return new;
}
