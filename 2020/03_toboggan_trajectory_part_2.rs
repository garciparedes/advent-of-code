use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let board: Vec<_> = buffer
        .trim()
        .split("\n")
        .map(|row| row.trim().chars().map(|cell| cell == '.').collect::<Vec<_>>())
        .collect();

    let ans = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .into_iter()
        .map(|offset| cost(&board, offset))
        .fold(1, |prod, c| prod * c);

    println!("{:?}", ans);

    return Ok(());
}

fn cost(board: &Vec<Vec<bool>>, offset: (usize, usize)) -> u128 {
    let width  = board[0].len();
    let (mut x, mut y) = (0, 0);
    let mut ans = 0;

    loop {
        x = x + offset.0;
        y = (y + offset.1) % width;

        if x >= board.len() {
            break;
        }
        if !board[x][y] {
            ans += 1;
        }
    }

    return ans;
}
