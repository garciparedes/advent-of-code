use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let ans: u128 = buffer
        .trim()
        .split('\n')
        .map(evaluate)
        .sum();

    println!("{}", ans);

    return Ok(());
}

#[derive(Debug)]
enum Token {
    Sum,
    Product,
    Number(u128),
    Expr(String)
}

fn evaluate(expr: &str) -> u128 {
    let mut iterable = tokenize(expr).into_iter();

    let mut ans = if let Some(token) = iterable.next() {
        match token {
            Token::Number(num) => num,
            Token::Expr(expr) => evaluate(&expr),
            _ => panic!(),
        }
    } else {
        panic!();
    };

    while let Some(first_token) = iterable.next() {
        if let Some(second_token) = iterable.next() {
            let second = match second_token {
                Token::Number(num) => num,
                Token::Expr(expr) => evaluate(&expr),
                _ => panic!(),
            };
            match first_token {
                Token::Sum => ans += second,
                Token::Product => ans *= second,
                _ => panic!(),
            }
        } else {
            panic!()
        }
        
    }

    return ans;
}

fn tokenize(expr: &str) -> Vec<Token> {
    let mut ans = Vec::new(); 
    
    let mut tmp = String::new();
    let mut depth = 0;
    for c in expr.trim().chars() {
        if c.is_whitespace() {
            continue;
        }
        if c == '(' {
            depth += 1;
            if depth == 1 {
                continue;
            }
        }
        if depth > 0 {
            if c == ')' {
                depth -= 1;
            }
            if depth == 0 {
                ans.push(Token::Expr(tmp));
                tmp = String::new();
            } else {
                tmp.push(c);
            }
        } else if c == '+' {
            if !tmp.is_empty() {
                ans.push(Token::Number(tmp.parse::<u128>().unwrap()));
                tmp.clear();
            }
            ans.push(Token::Sum);
        } else if c == '*' {
            if !tmp.is_empty() {
                ans.push(Token::Number(tmp.parse::<u128>().unwrap()));
                tmp.clear();
            }
            ans.push(Token::Product);
        } else if c.is_digit(10) {
            tmp.push(c);
        }
    }
    if !tmp.is_empty() {
        if depth > 0 {
            ans.push(Token::Expr(tmp));
        } else {
            ans.push(Token::Number(tmp.parse::<u128>().unwrap()));
        }
    }
    return ans;
}
