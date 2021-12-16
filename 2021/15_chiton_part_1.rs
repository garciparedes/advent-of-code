use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let board = buffer
        .trim()
        .split('\n')
        .map(|line| line
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let ans = shortest_path(&board, (0, 0), (board.len() - 1, board[0].len() - 1));

    println!("{}", ans);

    return Ok(());
}

fn shortest_path(
    board: &Vec<Vec<u32>>, 
    origin: (usize, usize),
    destination: (usize, usize),
) -> u32 {
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), origin));

    let mut distances = vec![vec![u32::MAX; board[0].len()]; board[0].len()];

    while let Some((Reverse(dist), (x, y))) = queue.pop() {
        if (x, y) == destination {
            return dist;
        }

        if distances[x][y] < dist {
            continue;
        }

        if x > 0 && distances[x - 1][y] > dist + board[x - 1][y] {
            distances[x - 1][y] = dist + board[x - 1][y];
            queue.push((Reverse(dist + board[x - 1][y]),  (x - 1, y)));
        } 
        if x < board.len() - 1 && distances[x + 1][y] > dist + board[x + 1][y] {
            distances[x + 1][y] = dist + board[x + 1][y];
            queue.push((Reverse(dist + board[x + 1][y]), (x + 1, y)));
        } 
        if y > 0 && distances[x][y - 1] > dist + board[x][y - 1] {
            distances[x][y - 1] = dist + board[x][y - 1];
            queue.push((Reverse(dist + board[x][y - 1]), (x, y - 1)));
        } 
        if y < board[0].len() - 1 && distances[x][y + 1] > dist + board[x][y + 1] {
            distances[x][y + 1] = dist + board[x][y + 1];
            queue.push((Reverse(dist + board[x][y + 1]), (x, y + 1)));
        } 
    }
    panic!();
}
