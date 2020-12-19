use std::io::prelude::*;
use std::io;
use std::collections::{
    HashSet,
    HashMap,
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut sections = buffer
        .trim()
        .split("\n\n");

    let rules: HashMap<_,_> = sections
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .map(|line| {
            let mut parts = line.trim().split(": ");
            let id = parts.next().unwrap().parse::<usize>().unwrap();
            let rule = Rule::new(parts.next().unwrap());
            return (id, rule);
        })
        .collect();

    let mut memo = HashMap::new();

    let ans = sections
        .next()
        .unwrap()
        .trim()
        .split("\n")
        .filter(|line| matches(&rules, 0, line.trim(), &mut memo))
        .count();

    println!("{}", ans);

    return Ok(());
}

#[derive(Debug)]
enum Rule<'a> {
    Ref(HashSet<Vec<usize>>),
    Exact(&'a str),
}

impl<'a> Rule<'a> {
    fn new(raw: &'a str) -> Self {
        if raw.chars().next().unwrap() == '"' && raw.chars().last().unwrap() == '"' {
            let l = raw.len();
            return Self::Exact(&raw[1..l - 1]);
        }

        let sub_rules: HashSet<_> = raw
            .split(" | ")
            .map(|part| {
                part
                    .trim()
                    .split_whitespace()
                    .map(|entry| entry.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        return Self::Ref(sub_rules);
    }
}

fn matches<'a>(
    rules: &HashMap<usize, Rule>, ref_id: usize, text: &'a str, memo: &mut HashMap<(usize, &'a str), bool>
) -> bool {
    if let Some(&ans) = memo.get(&(ref_id, text)) {
        return ans;
    }
    let ans = match &rules[&ref_id] {
        Rule::Exact(pattern) => *pattern == text,
        Rule::Ref(sub_rules) => {
            sub_rules
                .iter()
                .any(|sub_rule| matches_sub_rule(rules, sub_rule, text, memo))
        },
    };

    memo.insert((ref_id, text), ans);

    return ans;
}

fn matches_sub_rule<'a>(
    rules: &HashMap<usize, Rule>, sub_rule: &Vec<usize>, text: &'a str, memo: &mut HashMap<(usize, &'a str), bool>
) -> bool {
    let mut l = 0;
    for &ref_id in sub_rule.iter() {
        let mut matched = false;
        let mut new_l = l;
        for r in (l + 1)..=text.len() {
            if matches(rules, ref_id, &text[l..r], memo) {
                new_l = r;
                matched = true;
                break;
            }
        }
        l = new_l;
        if !matched {
            return false;
        }
    }
    return l == text.len();
}
