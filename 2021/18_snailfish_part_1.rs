use std::io;
use std::io::prelude::*;
use std::convert::From;

#[derive(Debug, Clone)]
enum Number {
    Regular(i128),
    Pair(Box<Number>, Box<Number>),
}

impl From<&str> for Number {
    fn from(item: &str) -> Self {
        if !item.starts_with('[') {
            return Number::Regular(item.parse::<i128>().unwrap());
        }

        let mut cut_index = 0;
        let mut depth = 0;
        for (i, c) in item.chars().enumerate() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
            }
            if depth == 1 && c == ',' {
                cut_index = i;
                break;
            }
        }

        let a = Number::from(&item[1..cut_index]);
        let b = Number::from(&item[cut_index + 1..item.len() - 1]);

        return Number::Pair(Box::new(a), Box::new(b));
    }
}

impl Number {
    fn magnitude(&self) -> i128 {
        match self {
            Number::Regular(v) => *v,
            Number::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude()
        }
    }

    fn sum(a: Number, b: Number) -> Number {
        let mut ans = Number::Pair(Box::new(a), Box::new(b));
        
        let mut another = true;
        while another {
            another = ans.explode(0).2;
            if !another {
                another = ans.split();
            }
        }
        return ans;
    }

    fn explode(&mut self, depth: usize) -> (Option<i128>, Option<i128>, bool) {
        match self {
            Number::Pair(a, b) => {
                if depth >= 4 {
                    if let Number::Regular(aa) = a.as_ref() {
                        if let Number::Regular(bb) = b.as_ref() {
                            return (Some(*aa), Some(*bb), true);
                        } 
                    }
                    panic!();
                } 

                let (la, ra, ea) = a.explode(depth + 1);
                if ea {
                    let mut l = None;
                    if la.is_some() && ra.is_some() {
                        *a = Box::new(Number::Regular(0));
                    }
                    if la.is_some() {
                       l = la; 
                    }
                    if let Some(aa) = ra {
                        b.inject_left(aa);
                    }
                    return (l, None, true);
                }

                let (lb, rb, eb) = b.explode(depth + 1);
                if eb {
                    let mut r = None;
                    if lb.is_some() && rb.is_some() {
                        *b = Box::new(Number::Regular(0));
                    }
                    if let Some(bb) = lb {
                       a.inject_right(bb); 
                    }
                    if rb.is_some() {
                        r = rb;
                    }
                    return (None, r, true);
                }

                return (None, None, false);
            },
            _ => (None, None, false),
        }
    }

    fn inject_left(&mut self, increment: i128) -> bool {
        match self {
            Number::Regular(v) => {
                *v += increment;
                return true;
            },
            Number::Pair(a, _) => {
                if a.inject_left(increment) {
                    return true;
                }
                panic!();
            },
        }
    }

    fn inject_right(&mut self, increment: i128) -> bool {
        match self {
            Number::Regular(v) => {
                *v += increment;
                return true;
            },
            Number::Pair(_, b) => {
                if b.inject_right(increment) {
                    return true;
                }
                panic!();
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Regular(v) => {
                if *v >= 10 {
                    *self = Number::Pair(
                        Box::new(Number::Regular(*v / 2)), Box::new(Number::Regular(*v / 2 + *v % 2))
                    );
                    return true;
                }
                return false;
            },
            Number::Pair(a, b) => {
                if a.split() {
                    return true;
                }
                return b.split();
            }
        }
    }

}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let numbers: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(Number::from)
        .collect();

    let number = numbers
        .iter()
        .cloned()
        .reduce(|cum, curr| Number::sum(cum, curr))
        .or(Some(numbers[0].clone()))
        .unwrap();

    let ans = number.magnitude();
    println!("{}", ans);

    return Ok(());
}

