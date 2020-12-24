use std::io::prelude::*;
use std::io;

const STEPS: usize = 100;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut cups: Vec<_> = buffer
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect();

    for _ in 0..STEPS {
        let mut pickup = Vec::new();
        for _ in 0..3 {
            let cup = cups.remove(1);
            pickup.push(cup);
        }

        let mut i = 1;
        let mut found = false;
        for target in (1..cups[0]).rev() {
            i = 1;
            while i < cups.len() {
                if cups[i] == target {
                    found = true;
                    break;
                } 
                i += 1;
            }

            if found {
                break;
            }

        }
        let j = if found { 
            i + 1
        } else {
            let mut k = 1;
            for i in 2..cups.len() {
                if cups[k] < cups[i] {
                    k = i;
                }
            }
            k + 1
        };
        while let Some(cup) = pickup.pop() {
            cups.insert(j, cup)
        }
        let tmp = cups.remove(0);
        cups.push(tmp);
    }
    while cups[0] != 1 {
        let tmp = cups.remove(0);
        cups.push(tmp);
    }
    let ans = cups[1..].into_iter().map(|d| d.to_string()).collect::<Vec<_>>().join("");
    println!("{}", ans);

    return Ok(());
}
