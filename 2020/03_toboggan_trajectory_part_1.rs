use std::io;
use std::io::prelude::*;

const OFFSET: (usize, usize) = (1, 3);

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let board: Vec<_> = buffer
        .trim()
        .split("\n")
        .map(|row| row.trim().chars().map(|cell| cell == '.').collect::<Vec<_>>())
        .collect();

    let width  = board[0].len();
    let (mut x, mut y) = (0, 0);
    let mut ans = 0;

    loop {
        x = x + OFFSET.0;
        y = (y + OFFSET.1) % width;

        if x >= board.len() {
            break;
        }
        if !board[x][y] {
            ans += 1;
        }
    }
    
    println!("{:?}", ans);

    return Ok(());
}

