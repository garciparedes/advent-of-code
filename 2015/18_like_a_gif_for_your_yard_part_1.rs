use std::io;
use std::io::prelude::*;

const STEPS: usize = 100;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let lines: Vec<_> = buffer.trim().split('\n').map(|line| line.trim()).collect();

    let mut grid = vec![vec![false; 2 + lines[0].len()]; 2 + lines.len()];
    for (i, line) in lines.into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                grid[i + 1][j + 1] = true;
            }
        }
    }

    for _ in 0..STEPS {
        grid = simulate(&grid);
    }

    let ans = grid
        .into_iter()
        .fold(0, |cum, curr| cum + curr.into_iter().fold(0, |i_cum, i_curr| if i_curr {i_cum + 1} else { i_cum }));

    println!("{}", ans);
    
    return Ok(());
}

fn simulate(previous: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut new = vec![vec![false; previous[0].len()]; previous.len()];

    for i in 1..previous.len() - 1 {
        for j in 1..previous[0].len() - 1 {
            let neighbors_on = [
                    previous[i - 1][j - 1], 
                    previous[i - 1][j], 
                    previous[i - 1][j + 1], 
                    previous[i][j - 1], 
                    previous[i][j + 1], 
                    previous[i + 1][j - 1], 
                    previous[i + 1][j], 
                    previous[i + 1][j + 1]
                ]
                .iter()
                .filter(|&v| *v)
                .count();

            if previous[i][j] {
                new[i][j] = neighbors_on == 2 || neighbors_on == 3;
            } else {
                new[i][j] = neighbors_on == 3;
            }
        }
    }
    return new;
}
