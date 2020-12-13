use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut lines = buffer.trim().split('\n');
    lines.next();

    let buses: Vec<_> = lines
        .next()
        .unwrap()
        .trim()
        .split(',')
        .enumerate()
        .filter(|(_, x)| *x != "x")
        .map(|(i, x)| {
            let num = x.parse::<i128>().unwrap();
            return ((num - (i as i128 % num)) % num, num);
        })
        .collect();
    
    let ans = solve(&buses);
    println!("{:?}", ans);

    return Ok(());
}

fn solve(congruences: &[(i128, i128)]) -> i128 {
    let n: i128 = congruences
        .iter()
        .map(|(_, n_i)| n_i)
        .product();
    
    let x: i128 = congruences
        .iter()
        .map(|(a_i, n_i)| {
            let tmp = n / n_i;   
            return a_i * mod_inv(tmp, *n_i) * tmp;
        })
        .sum();

    return x % n;
}

fn mod_inv(x: i128, n: i128) -> i128 {
    let (g, x, _) = egdc(x, n);
    if g == 1 {
        return (x % n + n) % n;
    } else {
        return 0;
    }
}

fn egdc(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1)
    }
    let (g, x, y) = egdc(b % a, a);
    return (g, y - (b / a) * x, x);
}
