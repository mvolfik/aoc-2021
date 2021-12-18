use crate::utils::DayResult;
use crate::utils::Failable;
use std::fmt::{Display, Formatter};
use std::str::{Chars, Lines};

pub(crate) fn main(stdin: Lines) -> DayResult {
    let nums: Vec<SnailPair> = stdin
        .map(|l| match SnailNum::parse(&mut l.chars()) {
            Ok(SnailNum::Pair(p)) => Ok(*p),
            x => Err(format!("Expected line to be a pair, got {:?}", x)),
        })
        .collect::<Result<_, _>>()?;
    let mut maxsum = 0;
    for a in &nums {
        for b in &nums {
            if a == b {
                continue;
            }
            let m = add(a.clone(), b.clone()).mag();
            if m > maxsum {
                maxsum = m
            }
        }
    }
    let sum = nums.into_iter().reduce(add).ok_or("Empty input")?;
    Ok((Ok(sum.mag().to_string()), Ok(maxsum.to_string())))
}

fn add(a: SnailPair, b: SnailPair) -> SnailPair {
    let mut s = SnailPair(SnailNum::Pair(Box::new(a)), SnailNum::Pair(Box::new(b)));
    loop {
        if !(s.explode() || s.split()) {
            break;
        }
    }
    s
}

fn expect_char(chariter: &mut Chars, c: char) -> Failable<()> {
    match chariter.next() {
        Some(cc) if cc == c => Ok(()),
        x => Err(format!(
            "Expected `{}`, found `{}`",
            c,
            match x {
                None => "EOL".to_string(),
                Some(x) => x.to_string(),
            }
        )),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct SnailPair(SnailNum, SnailNum);

impl Display for SnailPair {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl SnailPair {
    fn do_explode(&mut self, d: u8) -> Option<(bool, u32, u32)> {
        if d >= 4 {
            if let (SnailNum::Num(l), SnailNum::Num(r)) = (&self.0, &self.1) {
                return Some((true, *l, *r));
            }
        }

        let left_res = match &mut self.0 {
            SnailNum::Num(_) => None,
            SnailNum::Pair(p) => p.do_explode(d + 1),
        };
        if let Some((first, a, b)) = left_res {
            if first {
                self.0 = SnailNum::Num(0);
            }
            let mut adjacent_right = &mut self.1;
            let x;
            loop {
                match adjacent_right {
                    SnailNum::Pair(p) => adjacent_right = &mut p.0,
                    SnailNum::Num(y) => {
                        x = *y;
                        break;
                    }
                }
            }
            *adjacent_right = SnailNum::Num(x + b);
            Some((false, a, 0))
        } else {
            let right_res = match &mut self.1 {
                SnailNum::Num(_) => None,
                SnailNum::Pair(p) => p.do_explode(d + 1),
            };
            if let Some((first, a, b)) = right_res {
                if first {
                    self.1 = SnailNum::Num(0);
                }
                let mut adjacent_left = &mut self.0;
                let x;
                loop {
                    match adjacent_left {
                        SnailNum::Pair(p) => adjacent_left = &mut p.1,
                        SnailNum::Num(y) => {
                            x = *y;
                            break;
                        }
                    }
                }
                *adjacent_left = SnailNum::Num(x + a);
                Some((false, 0, b))
            } else {
                None
            }
        }
    }
    fn explode(&mut self) -> bool {
        !matches!(self.do_explode(0), None)
    }
    fn split(&mut self) -> bool {
        (match &mut self.0 {
            SnailNum::Num(n) => {
                if *n >= 10 {
                    self.0 = SnailNum::Pair(Box::new(Self(
                        SnailNum::Num(*n / 2),
                        SnailNum::Num(*n / 2 + *n % 2),
                    )));
                    true
                } else {
                    false
                }
            }
            SnailNum::Pair(p) => p.split(),
        } // if returned true, short-circuits, else tries on right child
         || match &mut self.1 {
            SnailNum::Num(n) => {
                if *n >= 10 {
                    self.1 = SnailNum::Pair(Box::new(Self(
                        SnailNum::Num(*n / 2),
                        SnailNum::Num(*n / 2 + *n % 2),
                    )));
                    true
                } else {
                    false
                }
            }
            SnailNum::Pair(p) => p.split(),
        })
    }
    fn mag(&self) -> u32 {
        3 * match &self.0 {
            SnailNum::Num(x) => *x,
            SnailNum::Pair(p) => p.mag(),
        } + 2 * match &self.1 {
            SnailNum::Num(x) => *x,
            SnailNum::Pair(p) => p.mag(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum SnailNum {
    Num(u32),
    Pair(Box<SnailPair>),
}

impl SnailNum {
    fn parse(c: &mut Chars) -> Failable<SnailNum> {
        match c.next() {
            Some('[') => {
                let left = Self::parse(c)?;
                expect_char(c, ',')?;
                let right = Self::parse(c)?;
                expect_char(c, ']')?;
                Ok(Self::Pair(Box::new(SnailPair(left, right))))
            }
            Some(x) => Ok(Self::Num(
                x.to_digit(10)
                    .ok_or(format!("Expected '[' or digit, found {}", x))?,
            )),
            None => Err("Unexpected EOL".to_string()),
        }
    }
}

impl Display for SnailNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Num(x) => x.fmt(f),
            Self::Pair(x) => x.fmt(f),
        }
    }
}
