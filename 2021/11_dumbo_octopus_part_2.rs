use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let raw: Vec<_> = buffer
        .trim()
        .split('\n')
        .collect();

    let mut board = vec![vec![0; raw[0].len() + 2]; raw.len() + 2];
    for (i, line) in raw.into_iter().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            board[i + 1][j + 1] = c.to_digit(10).unwrap();
        } 
    }

    let mut ans = 1;
    while !simulate(&mut board) {
        ans += 1;
    }

    println!("{}", ans);

    return Ok(());
}

fn simulate(board: &mut Vec<Vec<u32>>) -> bool {
    for i in 1..board.len() - 1 {
        for j in 1..board[i].len() - 1 {
            increment(board, i, j);
        }
    }

    let mut synchronized = true;
    for i in 1..board.len() - 1 {
        for j in 1..board.len() - 1 {
            if board[i][j] > 9 {
                board[i][j] = 0;
            } else {
                synchronized = false;
            }
        }
    }
    return synchronized;
}

fn increment(board: &mut Vec<Vec<u32>>, x: usize, y: usize) {
    if x < 1 || x > board.len() - 2 || y < 1 || y > board[x].len() - 2 {
        return;
    }
    board[x][y] += 1;
    if board[x][y] == 10 {
        increment(board, x - 1, y - 1);
        increment(board, x - 1, y);
        increment(board, x - 1, y + 1);
        increment(board, x, y - 1);
        increment(board, x, y + 1);
        increment(board, x + 1, y - 1);
        increment(board, x + 1, y);
        increment(board, x + 1, y + 1);
    }
}
