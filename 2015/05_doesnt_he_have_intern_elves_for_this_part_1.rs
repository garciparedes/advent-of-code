use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split('\n')
        .map(|word| {
            if word.contains("ab") || word.contains("cd") || word.contains("pq") || word.contains("xy") {
                return false;
            }
            let mut prev = ' ';
            let mut twice_letter = false;
            let mut vowel_count = 0;
            for c in word.trim().chars() {
                if !twice_letter && c == prev {
                   twice_letter = true; 
                }
                prev = c;
                if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
                    vowel_count += 1;
                }
            }

            return twice_letter && vowel_count >= 3;
        })
        .fold(0, |cum, valid| if valid { cum + 1 } else { cum });

    println!("{}", ans);

    return Ok(());
}
