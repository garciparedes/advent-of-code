use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut password: Vec<_> = buffer.trim().chars().map(|c| c as u8).collect();
    while !evaluate(&password) {
        increment(&mut password);
    }
    increment(&mut password);
    while !evaluate(&password) {
        increment(&mut password);
    }
    let ans: String = password.into_iter().map(|p| p as char).collect();

    println!("{}", ans);

    return Ok(());
}

fn increment(password: &mut [u8]) {
    let mut i = password.len() - 1;
    password[i] += 1;
    while i > 0 && password[i] == 'z' as u8 + 1 {
        password[i] = 'a' as u8;
        password[i - 1] += 1;
        i -= 1;
    }

}

fn evaluate(password: &[u8]) -> bool {
    for &p in password.iter() {
        if p == 'i' as u8 || p == 'o' as u8 || p == 'l' as u8 {
            return false;
        }
    }

    let mut first_tuple = None;
    let mut has_valid_tuples = false;
    let mut has_valid_triplet = false;
    for triplet in password.windows(3) {
        if !has_valid_tuples && (triplet[0] == triplet[1] || triplet[1] == triplet[2]) {
            if let Some(first) = first_tuple {
                if first != triplet[1] {
                    has_valid_tuples = true;
                }
            } else {
                first_tuple = Some(triplet[1]);
            }
            
        }

        if !has_valid_triplet && triplet[2] == triplet[1] + 1 && triplet[1] == triplet[0] + 1 {
            has_valid_triplet = true;
        }

        if has_valid_tuples && has_valid_triplet {
            return true;
        }
    }

    return false;
}
