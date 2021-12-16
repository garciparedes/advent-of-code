use std::io;
use std::io::prelude::*;
use std::iter::{ Enumerate, Peekable };
use std::vec::IntoIter;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    
    let binary = hex_to_binary(&buffer);

    let mut bits = binary.into_iter().enumerate().peekable();

    let ans = process_packet(&mut bits);
    println!("{}", ans);

    return Ok(());
}

fn hex_to_binary(hex: &str) -> Vec<bool> {
    hex
        .trim()
        .chars()
        .map(|c| {
            match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => "",
            }
        })
        .map(|binary| binary.chars().map(|c| c == '1').collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .concat()
}

fn process_packet(bits: &mut Peekable<Enumerate<IntoIter<bool>>>) -> usize {
    let mut _version = 0;
    for k in (0..3).rev() {
        if bits.next().unwrap().1 {
            _version += usize::pow(2, k);
        }
    }

    let mut type_ = 0;
    for k in (0..3).rev() {
        if bits.next().unwrap().1 {
            type_ += usize::pow(2, k);
        }
    }

    match type_ {
        4 => {
            let mut again = true;
            let mut number_bits = Vec::new();
            while again {
                again = bits.next().unwrap().1;
                for _ in 0..4 {
                    number_bits.push(bits.next().unwrap().1);
                }
            }
            let mut number = 0;
            for (k, b) in number_bits.into_iter().rev().enumerate() {
                if b {
                    number += usize::pow(2, k as u32);
                }
            }
            return number;
        },
        _ => {
            let mut numbers = Vec::new();
            let length_type = bits.next().unwrap().1;
            if length_type {
                let mut packets_length = 0;
                for k in (0..11).rev() {
                    if bits.next().unwrap().1 {
                        packets_length += usize::pow(2, k);
                    }
                }
                for _ in 0..packets_length {
                    numbers.push(process_packet(bits));
                }
            } else {
                let mut bits_length = 0;
                for k in (0..15).rev() {
                    if bits.next().unwrap().1 {
                        bits_length += usize::pow(2, k);
                    }
                }
                let index = bits.peek().unwrap().0;
                loop {
                    if let Some(&(b, _)) = bits.peek() {
                        if b >= index + bits_length {
                            break;
                        }
                    } else {
                        break;
                    }
                    numbers.push(process_packet(bits));
                }
            }
            match type_ {
                0 => numbers.into_iter().sum(),
                1 => numbers.into_iter().product(),
                2 => numbers.into_iter().min().unwrap(),
                3 => numbers.into_iter().max().unwrap(),
                5 => if numbers[0] > numbers[1] { 1 } else { 0 },
                6 => if numbers[0] < numbers[1] { 1 } else { 0 },
                7 => if numbers[0] == numbers[1] { 1 } else { 0 },
                _ => panic!(),
            }
        },
    }
}
