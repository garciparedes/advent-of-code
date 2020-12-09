use std::io::prelude::*;
use std::io;
use std::cmp;
use std::collections::VecDeque;

const MEMORY_SIZE: usize = 25;

fn main() -> io::Result<()> {
    let mut buffer = String::new();

    io::stdin().read_to_string(&mut buffer)?;

    let iterable = buffer
        .trim()
        .split('\n')
        .map(|value| value.parse::<i32>().unwrap());

    let mut sequence = Vec::new();
    let mut invalid= -1;
    let mut queue = VecDeque::new();
    for value in iterable {
        sequence.push(value);
        if queue.len() < MEMORY_SIZE {
            queue.push_back(value);
            continue;
        }

        if !contains_pair(&queue, value) {
            invalid = value;
            break;
        }

        queue.push_back(value);
        queue.pop_front();
    }

    let (mut l, mut r, mut cum) = (0, 1, 0);
    for i in 0..sequence.len() {
        while invalid < cum + sequence[i] {
            cum -= sequence[l];
            l += 1;
        }
        cum += sequence[i];
        r += 1;
        if invalid == cum {
            break;
        }
    }

    let ans: i32 = sequence[l..r]
        .into_iter()
        .fold(vec![i32::max_value(), i32::min_value()], |mut min_max, value| {
            min_max[0] = cmp::min(min_max[0], *value);
            min_max[1] = cmp::max(min_max[1], *value);
            return min_max;
        })
        .iter()
        .sum();

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
