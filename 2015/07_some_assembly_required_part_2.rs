use std::io;
use std::io::prelude::*;
use std::collections::HashMap;


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut circuit: Vec<_> = buffer
        .trim()
        .split('\n')
        .map(|line| line.trim().split(" -> ").collect::<Vec<_>>())
        .collect();

    let mut signals = HashMap::from([("b", 46065)]);

    while circuit.len() > 0 {
        let mut i = 0;
        while i < circuit.len() {
            let (op, target) = (circuit[i][0], circuit[i][1]);
            if target == "b" {
                circuit.remove(i);
            } else if op.contains("NOT") {
                let another = op.trim_start_matches("NOT ");
                if let Some(&v) = signals.get(another).or((&another).parse::<u16>().ok().as_ref()) {
                    signals.insert(target, !v);
                    circuit.remove(i);
                } else {
                    i += 1;
                }
            } else if op.contains("LSHIFT") {
                let parts: Vec<_> = op.split(" LSHIFT ").collect();
                let (another, offset) = (parts[0], parts[1].parse::<u16>().unwrap());
                if let Some(&v) = signals.get(another).or((&another).parse::<u16>().ok().as_ref()) {
                    signals.insert(target, v << offset);
                    circuit.remove(i);
                } else {
                    i += 1;
                }
            } else if op.contains("RSHIFT") {
                let parts: Vec<_> = op.split(" RSHIFT ").collect();
                let (another, offset) = (parts[0], parts[1].parse::<u16>().unwrap());
                if let Some(&v) = signals.get(another).or((&another).parse::<u16>().ok().as_ref()) {
                    signals.insert(target, v >> offset);
                    circuit.remove(i);
                } else {
                    i += 1;
                }
            } else if op.contains("OR") {
                let parts: Vec<_> = op.split(" OR ").collect();
                let (a, b) = (parts[0], parts[1]);
                if let (Some(&aa), Some(&bb)) = (
                    signals.get(a).or((&a).parse::<u16>().ok().as_ref()), 
                    signals.get(b).or((&b).parse::<u16>().ok().as_ref())
                ) {
                    signals.insert(target, aa | bb);
                    circuit.remove(i);
                } else {
                    i += 1;
                }
            } else if op.contains("AND") {
                let parts: Vec<_> = op.split(" AND ").collect();
                let (a, b) = (parts[0], parts[1]);
                if let (Some(&aa), Some(&bb)) = (
                    signals.get(a).or((&a).parse::<u16>().ok().as_ref()), 
                    signals.get(b).or((&b).parse::<u16>().ok().as_ref())
                ) {
                    signals.insert(target, aa & bb);
                    circuit.remove(i);
                } else {
                    i += 1;
                }
            } else {
                if let Ok(value) = op.parse::<u16>() {
                    signals.insert(target, value);
                    circuit.remove(i);
                } else if signals.contains_key(op) {
                    let value = *signals.get(op).unwrap();
                    signals.insert(target, value);
                    circuit.remove(i);
                } else {
                    i += 1;
                }
            }
        }
    }

    let ans = signals.get("a").unwrap();
    println!("{}", ans);

    return Ok(());
}
