use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut sections = buffer
        .trim()
        .split("\n\n");
    
    let schema: Vec<_> = sections
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let mut parts = line.trim().split(": ");
            let (key, raw_values) = (parts.next().unwrap(), parts.next().unwrap());

            let values: Vec<_> = raw_values
                .trim()
                .split(" or ")
                .map(|raw_intervals| {
                    let mut raw_intervals = raw_intervals.trim().split('-');
                    return (
                        raw_intervals.next().unwrap().parse::<u64>().unwrap(), 
                        raw_intervals.next().unwrap().parse::<u64>().unwrap()
                    );
                })
                .collect();

            return (key, values);
        })
        .collect();

    let ticket: Vec<_> = sections
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|field| field.parse::<u64>().unwrap())
        .collect();
    
    let indexes: Vec<_> = schema
        .iter()
        .map(|(name, _)| name)
        .enumerate()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(index, _)| index)
        .collect();

    let nearby: Vec<_> = sections
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .skip(1)
        .map(|line| line.trim().split(',').map(|field| field.parse::<u64>().unwrap()).collect::<Vec<_>>())
        .filter(|line| valid_line(line, &schema))
        .collect();

    let confusion_matrix = generate_confusion_matrix(&nearby, &schema);
    let matching = solve(&confusion_matrix);

    let ans: u64 = indexes
        .into_iter()
        .map(|i| ticket[matching.iter().position(|&x| x == i).unwrap()])
        .product();

    println!("{:?}", ans);

    return Ok(());
}

fn valid_line(numbers: &Vec<u64>, schema: &Vec<(&str, Vec<(u64, u64)>)>) -> bool {
    numbers
        .iter()
        .all(|&number| {
            schema
                .iter()
                .any(|(_, intervals)| intervals.iter().any(|interval| interval.0 <= number && number <= interval.1))
        })
}

fn generate_confusion_matrix(
    nearby: &Vec<Vec<u64>>,
    schema: &Vec<(&str, Vec<(u64, u64)>)>, 
) -> Vec<Vec<bool>> {
    let columns = transpose(&nearby);
    let m = schema.len();
    let mut ans = vec![vec![false; m]; m];
    for i in 0..m {
        for j in 0..m {
            if check(&columns[i], &schema[j].1) {
                ans[i][j] = true;
            }
        }
    }
    return ans;
}

fn transpose(nearby: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let m = nearby[0].len();

    let mut ans = vec![Vec::new(); m];
    for numbers in nearby {
        for (i, &number) in numbers.iter().enumerate() {
            ans[i].push(number);
        }
    }
    return ans;
}

fn check(numbers: &[u64], intervals: &[(u64, u64)]) -> bool {
    numbers
        .iter()
        .all(|num| intervals.iter().any(|(left, right)| left <= num && num <= right))
}

fn solve(
    valid: &Vec<Vec<bool>>,    
) -> Vec<usize> {
    let mut valid = valid.clone();
    
    let mut ans = vec![999_999; valid.len()];
    for _ in 0..valid.len() {
        let (i, v) = valid.iter().enumerate().min_by_key(|&(_, item)| {
            let v = item.iter().filter(|x| **x).count();
            if v == 0 {
                return 999_999;
            }
            return v;
        }).unwrap();
        let indexes: Vec<_> = v.iter().enumerate().filter(|(_, x)| **x).map(|(i, _)| i).collect();
        if indexes.len() > 1 {
            panic!();
        }
        for i in 0..valid.len() {
            valid[i][indexes[0]] = false;
        }
        ans[i] = indexes[0];
    }

    return ans;
}

