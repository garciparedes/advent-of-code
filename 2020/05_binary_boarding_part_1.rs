use std::io;
use std::io::prelude::*;

struct Seat<'a> {
    encoded: &'a str,
    decoded: Option<(u16, u16)>,
}

impl<'a> Seat<'a> {

    pub fn new(encoded: &'a str) -> Self {
        Self {encoded: encoded, decoded: None}
    }

    pub fn get_id(&mut self) -> u16 {
        let (x, y) = if let Some(c) = self.decoded { c } else { self.decode() };
        return x * 8 + y
    }

    fn decode(&mut self) -> (u16, u16) {
        let mut iterable = self.encoded.chars();

        let (mut l, mut r) = (0, 127);
        for _ in 0..7 {
            let op = iterable.next().unwrap();
            let mid = (l + r) / 2;
            match op {
                'F' => r = mid,
                _ => l = mid + 1,
            }
        }
        let x = l;

        let (mut l, mut r) = (0, 7);
        for _ in 0..3 {
            let op = iterable.next().unwrap();
            let mid = (l + r) / 2;
            match op {
                'L' => r = mid,
                _ => l = mid + 1,
            }
        }
        let y = l;
        
        self.decoded = Some((x, y));
        return (x, y);
    }
}



fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim()
        .split("\n")
        .map(|raw| Seat::new(raw).get_id())
        .max()
        .unwrap();
     
    println!("{:?}", ans);

    return Ok(());
}

