use std::io::prelude::*;
use std::io;
use std::collections::VecDeque;

const MEMORY_SIZE: usize = 25;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;


    let iterable = buffer
        .trim()
        .split('\n')
        .map(|value| value.parse::<i32>().unwrap());

    let mut ans = -1;
    let mut queue = VecDeque::new();
    for value in iterable {
        if queue.len() < MEMORY_SIZE {
            queue.push_back(value);
            continue;
        }

        if !contains_pair(&queue, value) {
            ans = value;
            break;
        }

        queue.push_back(value);
        queue.pop_front();
    }

    println!("{:?}", ans);

    return Ok(());
}

fn contains_pair(queue: &VecDeque<i32>, value: i32) -> bool {
    let n = queue.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if queue[i] + queue[j] == value {
                return true;
            }
        }
    }
    return false;
}
