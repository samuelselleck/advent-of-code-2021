use std::{fmt::Display, fs, ops::Add};

fn main() {
    let input = fs::read_to_string("test.txt").expect("file not found");
    let numbers: Vec<_> = input
        .lines()
        .map(|s| SnailNumber::parse_from_str(s))
        .collect();
    let sum = numbers.into_iter().fold(None, |a, e| match a {
            None => Some(e),
            Some(a) => Some(a + e),
        }).unwrap();
    //[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]

    //TODO: need to let numbers be changed further to the left/right
    println!("{}", sum);
}

#[derive(Debug, Clone)]
enum SnailNumber {
    Pair(Box<(SnailNumber, SnailNumber)>),
    Literal(u32),
}

impl SnailNumber {
    fn parse_from_str(str: &str) -> Self {
        Self::parse(&mut str.chars())
    }

    fn parse(str: &mut impl Iterator<Item = char>) -> Self {
        let tok = str.next().unwrap();
        if tok == '[' {
            let left = Self::parse(str);
            str.next(); // skip ,
            let right = Self::parse(str);
            str.next(); // skip ]
            Self::Pair(Box::new((left, right)))
        } else {
            let num = tok.to_digit(10).unwrap();
            Self::Literal(num)
        }
    }

    fn add(self, other: Self) -> Self {
        let mut num = Self::Pair(Box::new((self, other)));
        num.reduce();
        num
    }

    fn reduce(&mut self) {
        println!("{}", self);
        loop {
            if self.explode(0) {
                println!("Exploded to: {}", self);
                continue;
            }
            if self.split() {
                println!("Split to: {}", self);
                continue;
            }
            break;
        }
    }

    fn explode(&mut self, lvl: i32) -> bool {
        match self {
            Self::Pair(p) => {
                if lvl == 3 {
                    return match p.as_ref() {
                        (Self::Pair(_), Self::Pair(_)) => {
                            *self = Self::Pair(Box::new((Self::Literal(0), Self::Literal(0))));
                            true
                        }
                        (Self::Pair(pi), Self::Literal(n)) => {
                            let (_, r) = match pi.as_ref() {
                                (Self::Literal(l), Self::Literal(r)) => (l, r),
                                _ => panic!("Should not contain Pairs"),
                            };
                            *self = Self::Pair(Box::new((Self::Literal(0), Self::Literal(n + r))));
                            true
                        }
                        (Self::Literal(n), Self::Pair(pi)) => {
                            let (l, _) = match pi.as_ref() {
                                (Self::Literal(l), Self::Literal(r)) => (l, r),
                                _ => panic!("Should not contain Pairs"),
                            };
                            *self = Self::Pair(Box::new((Self::Literal(l + n), Self::Literal(0))));
                            true
                        }
                        (Self::Literal(_), Self::Literal(_)) => false,
                    };
                }
                let (l, r) = p.as_mut();
                l.explode(lvl + 1) || r.explode(lvl + 1)
            }
            Self::Literal(_) => false,
            //[[[[0,7],4],[[7,8],[6,0]]],[8,1]]
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Literal(n) => {
                let n = *n;
                if n >= 10 {
                    *self =
                        Self::Pair(Box::new((Self::Literal(n / 2), Self::Literal((n + 1) / 2))));
                    true
                } else {
                    false
                }
            }
            Self::Pair(p) => {
                let (l, r) = p.as_mut();
                l.split() || r.split()
            }
        }
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let parts = format!("{} + {}", self, other);
        let res = self.add(other);
        println!("{} = {}", parts, res);
        res
    }
}

impl Display for SnailNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Pair(p) => {
                    let (l, r) = p.as_ref();
                    format!("[{},{}]", l, r)
                }
                Self::Literal(n) => {
                    n.to_string()
                }
            }
        )
    }
}
