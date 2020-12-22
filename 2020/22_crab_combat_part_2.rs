use std::io::prelude::*;
use std::io;
use std::collections::{
    HashSet,
    HashMap,
    VecDeque,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut sections = buffer
        .trim()
        .split("\n\n")
        .map(|part| {
            part
                .trim()
                .split('\n')
                .skip(1)
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<VecDeque<_>>()
        });

    let (mut alice, mut bob) = (sections.next().unwrap(), sections.next().unwrap());
    let mut memo = HashMap::new();
    let result = simulate_game(&mut alice, &mut bob, &mut memo);
    let winner = if result { alice } else { bob };

    let ans: usize = winner
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum();

    println!("{}", ans);

    return Ok(());
}

fn simulate_game(
    alice: &mut VecDeque<usize>, 
    bob: &mut VecDeque<usize>, 
    memo: &mut HashMap<(VecDeque<usize>, VecDeque<usize>), bool>
) -> bool {
    if let Some(&ans) = memo.get(&(alice.clone(), bob.clone())) {
        return ans;
    }
    if let Some(&ans) = memo.get(&(bob.clone(), bob.clone())) {
        return !ans;
    }

    let mut status = HashSet::new();

    while !alice.is_empty() && !bob.is_empty() {
        if !status.insert((alice.clone(), bob.clone())) {
            return true;
        }

        let (alice_front, bob_front) = (alice.pop_front().unwrap(), bob.pop_front().unwrap());
        
        let mut result = alice_front > bob_front;
        if alice.len() as usize >= alice_front && bob.len() as usize >= bob_front {
            let mut alice_copy: VecDeque<_> = alice.iter().cloned().take(alice_front).collect();
            let mut bob_copy: VecDeque<_> = bob.iter().cloned().take(bob_front).collect();
            result = simulate_game(&mut alice_copy, &mut bob_copy, memo);
        }

        if result {
            alice.push_back(alice_front);
            alice.push_back(bob_front);
        } else {
            bob.push_back(bob_front);
            bob.push_back(alice_front);
        }
    }
    let ans = !alice.is_empty();
    memo.insert((alice.clone(), bob.clone()), ans);

    return ans;
}
