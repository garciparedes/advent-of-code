use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    
    let mut grid = vec![vec![false; 1000]; 1000];
    for operation in buffer.trim().split("\n") {
        let operation = operation.trim();
        let coordinates: Vec<_> = operation
            .trim_start_matches(|c: char| !c.is_digit(10))
            .trim_end_matches(|c: char| !c.is_digit(10))
            .split(" through ")
            .map(|raw| raw.split(",").map(|v| v.parse::<usize>().unwrap()).collect::<Vec<_>>())
            .collect();

        if operation.starts_with("turn on") {
            for i in coordinates[0][0]..=coordinates[1][0] {
                for j in coordinates[0][1]..=coordinates[1][1] {
                    grid[i][j] = true;
                }
            }
        } else if operation.starts_with("turn off") {
            for i in coordinates[0][0]..=coordinates[1][0] {
                for j in coordinates[0][1]..=coordinates[1][1] {
                    grid[i][j] = false;
                }
            }
        } else {
            for i in coordinates[0][0]..=coordinates[1][0] {
                for j in coordinates[0][1]..=coordinates[1][1] {
                    grid[i][j] ^= true;
                }
            }
        }
    }

    let ans = grid.into_iter().fold(0, |cum, row| cum + row.into_iter().filter(|&cell| cell).count());
    println!("{}", ans);

    return Ok(());
}
