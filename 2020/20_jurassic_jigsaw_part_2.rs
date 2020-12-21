use std::io::prelude::*;
use std::io;
use std::cmp;
use std::collections::{
    HashSet,
    HashMap,
};

const PATTERN: &'static str = 
"                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";


#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Tile {
    id: usize,
    content: Vec<Vec<bool>>,
    borders: Vec<Vec<bool>>,
}

impl Tile {
    fn new(raw: &str) -> Self {
        let mut iterable = raw
            .trim()
            .split('\n');


        let id = if let Some(line) = iterable.next() {
            let mut parts = line.trim().split_whitespace();
            parts.next();
            if let Some(raw_number) = parts.next() {
                raw_number[..raw_number.len() - 1].parse::<usize>().unwrap()
            } else {
                panic!()
            }
        } else {
            panic!()
        };

        let content: Vec<_> = iterable
            .map(|line| line.trim().chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect();
        
        let top = content[0].clone();
        let right: Vec<_> = content.iter().map(|row| row[row.len() - 1]).collect();
        let bottom = content[content.len() - 1].clone();
        let left: Vec<_> = content.iter().map(|row| row[0]).collect();
        
        return Tile { id: id, content: content, borders: vec![top, right, bottom, left]};
    }

    fn matched_borders(&self, another: &Tile) -> Option<(usize, usize, bool)> {
        for i in 0..4 {
            for j in 0..4 {
                if self.borders[i] == another.borders[j] {
                    return Some((i, j, false));
                } 
                if self.borders[i] == another.borders[j].iter().cloned().rev().collect::<Vec<_>>() {
                    return Some((i, j, true));
                } 
            }
        }
        return None;
    }

    fn matches_with(&self, another: &Tile) -> bool {
        self.matched_borders(another).is_some()
    }


    fn rotate(&self) -> Self {
    
        let mut content = Vec::new();
        for j in (0..self.content[0].len()).rev() {
            let mut row = Vec::new();
            for i in 0..self.content.len() {
                row.push(self.content[i][j].clone());
            }
            content.push(row);
        }

        let mut borders = self.borders.clone();
        let first = borders.remove(0);
        borders.push(first);

        return Tile { id: self.id, content: content, borders: borders };
     }

    fn swap(&self) -> Self {
        let mut content = Vec::new();
        for row in self.content.iter() {
            let tmp: Vec<_> = row.iter().cloned().rev().collect();
            content.push(tmp);
        }

        let borders = vec![
            self.borders[0].iter().cloned().rev().collect(),
            self.borders[3].clone(),
            self.borders[2].iter().cloned().rev().collect(),
            self.borders[1].clone(),
        ];


        return Tile { id: self.id, content: content, borders: borders };
    }

    fn content_without_border(&self) -> Vec<Vec<bool>> {
        let mut ans = Vec::new();
        for i in 1..self.content.len() - 1 {
            let mut row = Vec::new();
            for j in 1..self.content[0].len() - 1 {
                row.push(self.content[i][j]);
            }
            ans.push(row);
        }
        return ans;
    }
}


fn rotate_content(content: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut rotated_content = Vec::new();
    for j in (0..content[0].len()).rev() {
        let mut row = Vec::new();
        for i in 0..content.len() {
            row.push(content[i][j].clone());
        }
        rotated_content.push(row);
    }
    return rotated_content;
}

fn swap_content(content: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut swapped_content = Vec::new();

    for row in content.iter() {
        let tmp: Vec<_> = row.iter().cloned().rev().collect();
        swapped_content.push(tmp);
    }
    return swapped_content;
}



fn compose_image(matches: &HashMap<&Tile, HashSet<&Tile>>) -> Vec<Vec<bool>> {
    let mut structure = compose_board_structure(matches);

    let mut board = Vec::new();
    let mut matched = false;
    for _ in 0..4 {
        let mut first_tile = structure[0][0].clone();
        if let Some(ans) = compose_board(&structure, &first_tile) {
            println!("lalalalaa!");
            board = ans;
            break;
        }
        for _ in 0..2 {
            first_tile = first_tile.swap();
            for _ in 0..4 {
                first_tile = first_tile.rotate();
                if let Some(ans) = compose_board(&structure, &first_tile) {
                    println!("tadaaaa!");
                    board = ans;
                    matched = true;
                    break;
                }
            }
            if matched {
                break;
            }
        }
        if matched {
            break;
        }
        let mut new_structure = Vec::new();
        for j in (0..structure[0].len()).rev() {
            let mut row = Vec::new();
            for i in 0..structure.len() {
                row.push(structure[i][j]);
            }
            new_structure.push(row);
        }
        structure = new_structure;
    }
    if board.len() == 0 {
        panic!();
    }


    let contents = board
        .into_iter()
        .fold(Vec::new(), |mut full, row| {
            let n_r = row[0].content_without_border().len();
            let contents = row
                .into_iter()
                .fold(vec![Vec::new(); n_r], |mut full_row, tile| {
                    let content = tile.content_without_border();

                    for i in 0..content.len() {
                        full_row[i].extend(content[i].clone());
                    } 
                    return full_row;
                });

            for c in contents {
                full.push(c);
            }

            return full;
            
        });

    return contents;
}


fn compose_board<'a>(structure: &Vec<Vec<&Tile>>, first_tile: &Tile) -> Option<Vec<Vec<Tile>>> {
    let mut board: Vec<Vec<Tile>> = Vec::new();
    for i in 0..structure.len() {
        let mut row = Vec::new();
        for j in 0..structure[0].len() {
            if i == 0 && j == 0 {
                row.push(first_tile.clone());
                continue;
            }
            let mut tile = structure[i][j].clone();

            if i > 0 {
                loop {
                    let (first_rot, second_rot, swap) = board[i - 1][j].matched_borders(&tile).unwrap();
                    if first_rot != 2 {
                        return None;
                    }
                    if first_rot == 2 && second_rot == 0 && !swap {
                        break;
                    }
                    if swap {
                        tile = tile.swap();
                    }
                    tile = tile.rotate();
                }
            }
            if j > 0 {
                loop {
                    let (first_rot, second_rot, swap) = row[j - 1].matched_borders(&tile).unwrap();
                    if first_rot != 1 {
                        return None;
                    }
                    if first_rot == 1 && second_rot == 3 && !swap {
                        break;
                    }
                    if swap {
                        tile = tile.swap();
                    }
                   tile = tile.rotate();
                        
                }
            }

            row.push(tile);
        }
        board.push(row);
    }

    return Some(board);
}

fn compose_board_structure<'a>(matches: &HashMap<&'a Tile, HashSet<&'a Tile>>) -> Vec<Vec<&'a Tile>> {
    let mut chosen = HashSet::new();

    let mut board = Vec::new();

    let first = matches.iter().filter(|(_, v)| v.len() == 2).map(|(k, _)| k).next().unwrap();

    let mut row = Vec::new();
    let mut current = first;
    while let Some(next) = matches[current]
        .iter()
        .filter(|&tile| !chosen.contains(tile) && matches[tile].len() == 3)
        .next() 
    {
        row.push(*current);
        chosen.insert(current);
        current = next;
    }
    row.push(*current);
    chosen.insert(current);

    if let Some(next) = matches[current]
        .iter()
        .filter(|&tile| !chosen.contains(tile) && matches[tile].len() == 2)
        .next()
    {
        row.push(*next);
    }

    let m = row.len();
    board.push(row);

    while let Some(first) = matches[board[board.len() - 1][0]]
        .iter()
        .filter(|&tile| !chosen.contains(tile) && matches[tile].len() < 4)
        .next() 
    {
        let mut row = Vec::new();
        let mut current = first;
        let mut i = 0;
        while let Some(next) = matches[current]
            .iter()
            .filter(|&tile| !chosen.contains(tile) && i < m - 1 && matches[tile].contains(board[board.len() - 1][i + 1]))
            .next() 
        {
            row.push(*current);
            chosen.insert(current);
            i += 1;
            current = next;
        }
        row.push(*current);
        chosen.insert(current);

        board.push(row);

    }
   
    return board;
}
 
fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let tiles: Vec<_> = buffer
        .trim()
        .split("\n\n")
        .map(|raw| Tile::new(raw))
        .collect();
    
    
    let mut matches = HashMap::new(); 
    for i in 0..tiles.len() {
        let tile_i = &tiles[i];
        for j in (i + 1)..tiles.len() {
            let tile_j = &tiles[j];
            if tile_i.matches_with(tile_j) {
                matches.entry(tile_i).or_insert_with(HashSet::new).insert(tile_j);
                matches.entry(tile_j).or_insert_with(HashSet::new).insert(tile_i);
            }
        }
    }
    
    let image = compose_image(&matches);
    
    let mut pattern: Vec<_> = PATTERN
        .split('\n')
        .map(|row| row.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect();

    let mut ans = 0;
    for _ in 0..2 {
        pattern = swap_content(&pattern);
        for _ in 0..4 {
            pattern = rotate_content(&pattern);
            
            let tmp = find_pattern(&image, &pattern);
            ans = cmp::max(ans, tmp);
        }
    }

    println!("{:?}", ans);
    return Ok(());
}


fn find_pattern(image: &Vec<Vec<bool>>, pattern: &Vec<Vec<bool>>) -> usize {
    let mut image = image.clone();
    let mut any_matched = false;
    for i in 0..(image.len() - pattern.len()) {
        for j in 0..(image[0].len() - pattern[0].len()) {

            let mut matched = true;
            for k in 0..pattern.len() {
                for l in 0..pattern[0].len() {
                    if !pattern[k][l] {
                        continue;
                    }
                    if !image[i + k][j + l] {
                        matched = false;
                        break;
                    }
                }
                if !matched {
                    break;
                }
            }
            if matched {
                any_matched = true;
                for k in 0..pattern.len() {
                    for l in 0..pattern[0].len() {
                        if !pattern[k][l] {
                            continue;
                        }
                        if image[i + k][j + l] {
                           image[i + k][j + l] = false; 
                        }
                    }
                }
            }
        }
    }
    if !any_matched {
        return 0;
    }
    return image
        .into_iter()
        .map(|row| row.into_iter().filter(|cell| *cell).count())
        .sum();
}
