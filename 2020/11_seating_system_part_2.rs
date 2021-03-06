use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut current: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|row| row.trim().chars().collect::<Vec<_>>())
        .collect();
   
    while let Some(new) = update(&current) {
        current = new;
    }

    let ans = count(&current);
    println!("{:?}", ans);

    return Ok(());
}

fn update(board: &Vec<Vec<char>>) -> Option<Vec<Vec<char>>> {
    let mut new = board.clone();
    let (n, m) = (new.len(), new[0].len());

    let mut has_changes = false;
    for i in 0..n {
        for j in 0..m {
            new[i][j] = match board[i][j] {
                'L' => if neighbors_count(board, i, j) == 0 { '#' } else { 'L' }, 
                '#' => if neighbors_count(board, i, j) < 5 { '#' } else { 'L' },
                _ => '.'
            };
            if !has_changes && new[i][j] != board[i][j] {
                has_changes = true;
            }
        }
    }

    if !has_changes {
        return None;
    }
    
    return Some(new);
}

fn neighbors_count(board: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let (n, m) = (board.len(), board[0].len());
    let mut c = 0;

    let mut k = 1;
    while i >= k && j >= k && board[i - k][j - k] == '.' {
        k += 1
    }
    if i >= k && j >= k && board[i - k][j - k] == '#' {
        c += 1;
    }

    let mut k = 1;
    while i >= k && board[i - k][j] == '.' {
        k += 1
    }
    if i >= k && board[i - k][j] == '#' {
        c += 1;
    }

    let mut k = 1;
    while i >= k && j + k <= m - 1 && board[i - k][j + k] == '.' {
        k += 1
    }
    if i >= k && j + k <= m - 1 && board[i - k][j + k] == '#' {
        c += 1;
    }

    let mut k = 1;
    while j + k <= m - 1 && board[i][j + k] == '.' {
        k += 1
    }
    if j + k <= m - 1 && board[i][j + k] == '#' {
        c += 1;
    }

    let mut k = 1;
    while i + k <= n - 1 && j + k <= m - 1 && board[i + k][j + k] == '.' {
        k += 1
    }
    if i + k <= n - 1 && j + k <= m - 1 && board[i + k][j + k] == '#' {
        c += 1;
    }
    
    let mut k = 1;
    while i + k <= n - 1 && board[i + k][j] == '.' {
        k += 1
    }
    if i + k <= n - 1 && board[i + k][j] == '#' {
        c += 1;
    }

    let mut k = 1;
    while i + k <= n - 1 && j >= k && board[i + k][j - k] == '.' {
        k += 1
    }
    if i + k <= n - 1 && j >= k && board[i + k][j - k] == '#' {
        c += 1;
    }

    let mut k = 1;
    while j >= k && board[i][j - k] == '.' {
        k += 1
    }
    if j >= k && board[i][j - k] == '#' {
        c += 1;
    }

    return c;
}

fn count(board: &Vec<Vec<char>>) -> usize {
    board
        .iter()
        .fold(0, |cum, row| cum + row.iter().filter(|x| *x == &'#').count())
}
