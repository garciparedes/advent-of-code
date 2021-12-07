use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let positions: Vec<_> = buffer
        .trim()
        .split(',')
        .map(|r| r.parse::<i32>().unwrap())
        .collect();

    let (min, max) = positions
        .iter()
        .fold((i32::MAX, i32::MIN), |mut cum, curr| {
            if *curr < cum.0 {
                cum = (*curr, cum.1);
            }
            if *curr > cum.1 {
                cum = (cum.0, *curr);
            }
            return cum;
        });

    let mut costs = vec![0; (max - min + 1) as usize];
    for p in positions {
        for i in 0..costs.len() {
            let distance = i32::abs(p - (min + i as i32));
            costs[i] += distance * (distance + 1) / 2;
        }
    }

    let ans = costs
        .into_iter()
        .min()
        .unwrap();

    println!("{}", ans);

    return Ok(());
}
