use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let iterable = buffer
        .trim()
        .split('\n')
        .map(|raw| {
            let (op, offset) = raw.split_at(1);
            let offset = offset.parse::<i32>().unwrap();
            return (op, offset);
        });

    let (mut position, mut waypoint) = (vec![0, 0], vec![1, 10]);
    for (op, offset) in iterable {
        match op {
            "N" => waypoint[0] += offset,
            "S" => waypoint[0] -= offset,
            "E" => waypoint[1] += offset,
            "W" => waypoint[1] -= offset,
            "L" => {
                let steps = ((offset % 360) / 90) % 4;
                for _ in 0..steps {
                    waypoint = vec![waypoint[1], -waypoint[0]]
                }
            }
            "R" => {
                let steps = ((offset % 360) / 90) % 4;
                for _ in 0..steps {
                    waypoint = vec![-waypoint[1], waypoint[0]]
                }
            },
            "F" => {
                position[0] += waypoint[0] * offset;
                position[1] += waypoint[1] * offset;
            },
            _ => (),
        }
    }
    
    let ans = position[0].abs() + position[1].abs();

    println!("{:?}", ans);


    return Ok(());
}
