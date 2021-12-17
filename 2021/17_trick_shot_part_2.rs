use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut limits = buffer
        .trim()
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|pair| {
            let (raw_low, raw_high) = pair.split_once("=").unwrap().1.split_once("..").unwrap();
            return (raw_low.parse::<i32>().unwrap(), raw_high.parse::<i32>().unwrap());
        });

    let (min_x, max_x) = limits.next().unwrap();
    let (min_y, max_y) = limits.next().unwrap();

    let mut ans = 0;
    for vx in 0..=1000 {
        for vy in -1000..=1000 {
            if simulate(min_x, max_x, min_y, max_y, vx, vy).is_some() {
                ans += 1;
            }

        }
    }
    println!("{:?}", ans);
  
    return Ok(());
}

fn simulate(
    min_x: i32, 
    max_x: i32, 
    min_y: i32, 
    max_y: i32, 
    mut velocity_x: i32, 
    mut velocity_y: i32, 
) -> Option<i32> {
    let (mut x, mut y): (i32, i32) = (0, 0);

    let mut best_y = y;
    while x <= max_x && y >= min_y {
        if best_y < y {
            best_y = y;
        }
        if x >= min_x && y <= max_y {
            return Some(best_y);
        } 

        x += velocity_x;
        y += velocity_y;


        if velocity_x < 0 {
            velocity_x += 1;
        } else if velocity_x > 0 {
            velocity_x -= 1;
        }

        velocity_y -= 1;
    }

    return None;
}
