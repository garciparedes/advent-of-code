use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let lines: Vec<_> = buffer.trim().split('\n').map(|line| line.trim()).collect();
    let (n, m) = (lines.len() + 2, lines[0].len() + 2);
    let mut board = vec![vec![9; m]; n];
    for (i, line) in lines.into_iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            board[i + 1][j + 1] = c.to_digit(10).unwrap() as i32;
        }
    }

    let mut ans = 0;
    for i in 1..(n - 1) {
        for j in 1..(m - 1) {
            if board[i][j] < board[i + 1][j] 
                && board[i][j] < board[i - 1][j] 
                && board[i][j] < board[i][j + 1] 
                && board[i][j] < board[i][j - 1] {
                ans += 1 + board[i][j];
            }
        }
    }

    println!("{}", ans);

    return Ok(());
}
