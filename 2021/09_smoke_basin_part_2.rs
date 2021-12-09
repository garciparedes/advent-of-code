use std::io;
use std::io::prelude::*;
use std::cmp;

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

    let points = find_points(&board);

    let mut visited = vec![vec![false; m]; n];
    let mut sizes: Vec<_> = points
        .into_iter()
        .map(|point| get_size(&board, &mut visited, point))
        .collect();
    sizes.sort_unstable_by_key(|&s| cmp::Reverse(s));

    let ans: u32 = sizes[..3].into_iter().product();
    println!("{}", ans);

    return Ok(());
}

fn find_points(board: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for i in 1..(board.len() - 1) {
        for j in 1..(board[0].len() - 1) {
            if board[i][j] < board[i + 1][j] 
                && board[i][j] < board[i - 1][j] 
                && board[i][j] < board[i][j + 1] 
                && board[i][j] < board[i][j - 1] {
                points.push((i, j));
            }
        }
    }
    return points;
}

fn get_size(board: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, point: (usize, usize)) -> u32 {
    if visited[point.0][point.1] || board[point.0][point.1] == 9 {
        return 0;
    }
    visited[point.0][point.1] = true;
    let mut ans = 1;
    if board[point.0][point.1] < board[point.0 + 1][point.1] {
        ans += get_size(&board, visited, (point.0 + 1, point.1));
    }
    if board[point.0][point.1] < board[point.0 - 1][point.1] {
        ans += get_size(&board, visited, (point.0 - 1, point.1));
    }
    if board[point.0][point.1] < board[point.0][point.1 + 1] {
        ans += get_size(&board, visited, (point.0, point.1 + 1));
    }
    if board[point.0][point.1] < board[point.0][point.1 - 1] {
        ans += get_size(&board, visited, (point.0, point.1 - 1));
    }
    return ans;
}
