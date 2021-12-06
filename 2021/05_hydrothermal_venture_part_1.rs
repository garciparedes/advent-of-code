use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cmp::{ min, max };

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut board = HashMap::new();

    for line in buffer.trim().split('\n') {
        let (raw_1, raw_2) = line.trim().split_once(" -> ").unwrap();
        let (raw_x1, raw_y1) = raw_1.trim().split_once(',').unwrap();
        let (raw_x2, raw_y2) = raw_2.trim().split_once(',').unwrap();
        let (x1, y1) = (raw_x1.parse::<isize>().unwrap(), raw_y1.parse::<isize>().unwrap());
        let (x2, y2) = (raw_x2.parse::<isize>().unwrap(), raw_y2.parse::<isize>().unwrap());

        if x1 == x2 {
            for i in min(y1, y2)..=max(y1, y2) {
                *board.entry((x1, i)).or_insert(0) += 1;
            }
        } else if y1 == y2 {
            for i in min(x1, x2)..=max(x1, x2) {
                *board.entry((i, y1)).or_insert(0) += 1;
            }
        }
    }

    let ans = board
        .values()
        .filter(|&v| *v >= 2)
        .count();

    println!("{}", ans);

    return Ok(());
}
