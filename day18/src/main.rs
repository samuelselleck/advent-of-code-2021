use std::{fmt::Display, fs};

fn main() {
    let input = fs::read_to_string("numbers.txt").expect("file not found");
    let numbers: Vec<_> = input.lines().map(|s| SnailNumber::parse(s)).collect();

    let mut m = 0;
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            let (a, b) = (&numbers[i], &numbers[j]);
            let r = a.add(b).magnitude();
            let l = b.add(a).magnitude();
            m = m.max(r);
            m = m.max(l);
        }
    }
    println!("{}", m);
}

#[derive(Debug, Clone, Copy)]
enum SnailToken {
    LeftBracket,
    RightBracket,
    Number(u32),
}

#[derive(Debug, Clone)]
struct SnailNumber {
    tokens: Vec<SnailToken>,
}

impl SnailNumber {
    fn parse(s: &str) -> Self {
        Self {
            tokens: s
                .chars()
                .flat_map(|c| match c {
                    '[' => Some(SnailToken::LeftBracket),
                    ']' => Some(SnailToken::RightBracket),
                    ',' => None,
                    n => Some(SnailToken::Number(n.to_digit(10).unwrap())),
                })
                .collect(),
        }
    }

    fn add(&self, other: &Self) -> Self {
        let mut new_toks = self.tokens.clone();
        new_toks.insert(0, SnailToken::LeftBracket);
        new_toks.extend(other.tokens.clone());
        new_toks.push(SnailToken::RightBracket);
        let mut res = Self { tokens: new_toks };
        res.reduce();
        res
    }

    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }

    fn explode(&mut self) -> bool {
        let ind = self.find_leftmost_explosion_pair_ind();
        match ind {
            None => false,
            Some(i) => {
                let e: Vec<_> = self
                    .tokens
                    .splice(i..(i + 4), [SnailToken::Number(0)])
                    .collect();
                let mut explode_elem = |token, dir| {
                    let val = match token {
                        SnailToken::Number(n) => n,
                        _ => panic!("not a number in explode"),
                    };
                    let mut i = i as i32 + dir;
                    while let Some(t) = self.tokens.get_mut(i as usize) {
                        match t {
                            SnailToken::Number(n) => {
                                *n += val;
                                return true;
                            }
                            _ => (),
                        }
                        i += dir;
                    }
                    false
                };
                let l = explode_elem(e[1], -1);
                let r = explode_elem(e[2], 1);
                l || r
            }
        }
    }

    fn find_leftmost_explosion_pair_ind(&self) -> Option<usize> {
        let mut depth = 0;
        for i in 0..self.tokens.len() {
            match self.tokens[i] {
                SnailToken::LeftBracket => {
                    if depth >= 4 {
                        return Some(i);
                    } else {
                        depth += 1;
                    }
                }
                SnailToken::RightBracket => depth -= 1,
                SnailToken::Number(_) => (),
            }
        }
        None
    }

    fn split(&mut self) -> bool {
        for i in 0..self.tokens.len() {
            match self.tokens[i] {
                SnailToken::Number(n) => {
                    if n >= 10 {
                        self.tokens.splice(
                            i..=i,
                            [
                                SnailToken::LeftBracket,
                                SnailToken::Number(n / 2),
                                SnailToken::Number((n + 1) / 2),
                                SnailToken::RightBracket,
                            ],
                        );
                        return true;
                    }
                }
                _ => (),
            }
        }
        false
    }

    fn magnitude(&self) -> u128 {
        Self::mag_of_tokens(0, &self.tokens).1
    }

    fn mag_of_tokens(i: usize, tokens: &[SnailToken]) -> (usize, u128) {
        match tokens[i] {
            SnailToken::LeftBracket => {
                let (i, l_mag) = Self::mag_of_tokens(i + 1, tokens);
                let (i, r_mag) = Self::mag_of_tokens(i + 1, tokens);
                return (i + 1, l_mag * 3 + r_mag * 2);
            }
            SnailToken::RightBracket => {
                println!("{:?}", tokens[i]);
                panic!();
            }
            SnailToken::Number(n) => (i, n as u128),
        }
    }
}

impl Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for t in &self.tokens {
            match t {
                SnailToken::LeftBracket => s.push('['),
                SnailToken::RightBracket => s.push(']'),
                SnailToken::Number(n) => s.extend(format!(" {} ", n).chars()),
            };
        }
        write!(f, "{}", s)
    }
}
