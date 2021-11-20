use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split('\n')
        .map(|word| {
            let chars: Vec<_> = word.trim().chars().collect();

            let mut good_pair = false;
            let mut good_triplet = false;
            
            let mut pairs = HashSet::new();
            for triplet in chars.windows(3) {
                let (a, b, c) = (triplet[0], triplet[1], triplet[2]);

                if !good_pair && pairs.contains(&(b, c)) {
                    good_pair = true;
                }

                if !good_triplet && a == c {
                    good_triplet = true;
                }

                pairs.insert((a, b));
            }

            return good_pair && good_triplet;
        })
        .fold(0, |cum, valid| if valid { cum + 1 } else { cum });

    println!("{}", ans);

    return Ok(());
}
