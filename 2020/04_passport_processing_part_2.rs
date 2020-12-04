use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

struct Entry<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Entry<'a> {

    fn new(raw: &'a str) -> Self {

        let fields: HashMap<_, _> = raw
            .split_whitespace()
            .map(|pair| {
                let kv: Vec<_> = pair.split(':').collect();
                return (kv[0], kv[1]);
            })
            .collect();

        return Self { fields: fields };
    }

    fn is_valid(&self) -> bool {
        self.is_valid_byr()
            && self.is_valid_iyr()
            && self.is_valid_eyr()
            && self.is_valid_hgt()
            && self.is_valid_hcl()
            && self.is_valid_ecl()
            && self.is_valid_pid()
    }

    fn is_valid_byr(&self) -> bool {
        if let Some(value) = self.fields.get(&"byr") {
            if let Ok(value) = value.parse::<u16>() {
                if value < 1920 {
                    return false;
                }
                if value > 2002 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        return true;
    }

    fn is_valid_iyr(&self) -> bool {
        if let Some(value) = self.fields.get(&"iyr") {
            if let Ok(value) = value.parse::<u16>() {
                if value < 2010  {
                    return false;
                }
                if value > 2020 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        return true;
    }

    fn is_valid_eyr(&self) -> bool {
        if let Some(value) = self.fields.get(&"eyr") {
            if let Ok(value) = value.parse::<u16>() {
                if value < 2020  {
                    return false;
                }
                if value > 2030 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        return true;
    }

    fn is_valid_hgt(&self) -> bool {
        if let Some(value) = self.fields.get(&"hgt") {
            let n = value.len();
            if n < 2 {
                return false;
            }

            let (raw_number, unit) = value.split_at(n - 2);
            if let Ok(number) = raw_number.parse::<u8>() {
                return  match unit {
                    "cm" => (150 <= number && number <= 193),
                    "in" => (59 <= number && number <= 76),
                    _ => false,
                } 
            } else {
                return false;
            }
        } else {
            return false;
        }
    }

    fn is_valid_hcl(&self) -> bool {
        if let Some(value) = self.fields.get(&"hcl") {
            if value.len() != 7 {
                return false;
            }
            let mut iterable = value.chars();
            if let Some('#') = iterable.next() {
                if !iterable.all(|c| c.is_digit(16)){
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        return true;
    }

    fn is_valid_ecl(&self) -> bool {
        if let Some(value) = self.fields.get(&"ecl") {
            return match *value {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            } 
        } else {
            return false;
        } 
    }

    fn is_valid_pid(&self) -> bool {
        if let Some(value) = self.fields.get(&"pid") {
            if value.len() != 9 {
                return false;
            }
            if !value.chars().all(|c| c.is_digit(10)) {
                return false;
            }
        } else {
            return false;
        }
        return true;
    }
}


fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans = buffer
        .trim().split("\n\n")
        .map(|raw| Entry::new(raw))
        .filter(|entry| entry.is_valid())
        .count();
    
    println!("{:?}", ans);

    return Ok(());
}

