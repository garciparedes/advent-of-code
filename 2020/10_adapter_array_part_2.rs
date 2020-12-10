use std::io::prelude::*;
use std::io;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut jolts: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|jolt| jolt.parse::<usize>().unwrap())
        .collect();

    jolts.sort_unstable();
    jolts.insert(0, 0);
    let mut memory = HashMap::new();
    let ans = helper(&jolts, 0, &mut memory);
    println!("{:?}", ans);

    return Ok(());
}


fn helper(jolts: &[usize], index: usize, memory: &mut HashMap<usize, usize>) -> usize {
    match index.cmp(&(jolts.len() - 1)) {
        Ordering::Greater => 0,
        Ordering::Equal => 1,
        Ordering::Less => {
            if let Some(&ans) = memory.get(&index) {
                return ans;
            } 
            let mut ans = 0; 
            let mut i = index + 1;
            while i < jolts.len() && jolts[i] - jolts[index] <= 3 {
                ans += helper(jolts, i, memory);
                i += 1;
            } 

            memory.insert(index, ans);
            return ans;
        }
    }
} 
