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

    let (mut position, mut direction) = (vec![0, 0], 1);
    for (op, offset) in iterable {
        match op {
            "N" => position[0] += offset,
            "S" => position[0] -= offset,
            "E" => position[1] += offset,
            "W" => position[1] -= offset,
            "L" => {
                let dir_offset = (offset % 360) / 90;
                direction = ((direction - dir_offset) % 4 + 4) % 4;
            }
            "R" => {
                let dir_offset = (offset % 360) / 90;
                direction = (direction + dir_offset) % 4;
            },
            "F" => {
                match direction {
                    0 => position[0] += offset,
                    1 => position[1] += offset,
                    2 => position[0] -= offset, 
                    3 => position[1] -= offset,
                    _ => (),
                }
            },
            _ => (),
        }
    }
    
    let ans = position[0].abs() + position[1].abs();

    println!("{:?}", ans);


    return Ok(());
}
