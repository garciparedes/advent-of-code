use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut sections = buffer
        .trim()
        .split("\n\n");
    
    let schema: HashMap<_,_> = sections
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
                        raw_intervals.next().unwrap().parse::<i32>().unwrap(), 
                        raw_intervals.next().unwrap().parse::<i32>().unwrap()
                    );
                })
                .collect();

            return (key, values);
        })
        .collect();


    sections.next();  // ignoring my ticket...


    let ans: i32 = sections
        .next()
        .unwrap()
        .trim()
        .split('\n')
        .skip(1)
        .map(|line| {
            line
                .trim()
                .split(',')
                .map(|field| {
                    let number = field.parse::<i32>().unwrap();
                    
                    let contained = schema
                        .values()
                        .any(|intervals| intervals.iter().any(|interval| interval.0 <= number && number <= interval.1));

                    if contained {
                        return 0;
                    }

                    return number;
                })
                .sum::<i32>()
        })
        .sum();

    println!("{:?}", ans);

    return Ok(());
}
