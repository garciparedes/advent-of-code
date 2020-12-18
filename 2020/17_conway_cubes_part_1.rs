use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut space = HashMap::new();

    let i_len = buffer.trim().split('\n').count() as isize;
    let j_len = buffer.trim().split('\n').next().unwrap().trim().len() as isize;
    
    let (mut il, mut ir) = (- i_len / 2, i_len / 2);
    let (mut jl, mut jr) = (- j_len / 2, j_len / 2);
    let (mut kl, mut kr) = (0, 0);

    for (i, line) in buffer.trim().split('\n').enumerate() {
        for (j, cell) in line.trim().chars().enumerate() {
            space.insert((i as isize- i_len / 2, j as isize - j_len / 2, 0), cell == '#');
        }
    }
        
    for _ in 0..6 {
        il -= 1;
        ir += 1;
        jl -= 1;
        jr += 1; 
        kl -= 1;
        kr += 1;

        let old = space.clone();
        for i in il..=ir {
            for j in jl..=jr {
                for k in kl..=kr {
                    let count = count_active(&old, i, j, k); 
                    if *old.get(&(i, j, k)).unwrap_or(&false) {
                        space.insert((i, j, k), count == 2 || count == 3);
                    } else {
                        space.insert((i, j, k), count == 3);
                    }
                }
            }
        }
    }

    let ans = space.values().filter(|&x| *x).count();
    println!("{:?}", ans);




    return Ok(());
}

fn count_active(space: &HashMap<(isize, isize, isize), bool>, i: isize, j: isize, k: isize) -> usize {
    let mut ans = 0;
    for ii in (i - 1)..=(i + 1) {
        for jj in (j - 1)..=(j + 1) {
            for kk in (k - 1)..=(k + 1) {
                if (ii, jj, kk) != (i, j, k) && *space.get(&(ii, jj, kk)).unwrap_or(&false) {
                    ans += 1;
                }
            }
        }
    }
    return ans;
}
