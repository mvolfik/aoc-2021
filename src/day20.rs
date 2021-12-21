use crate::utils::DayResult;
use crate::utils::Failable;

use std::str::Lines;

pub(crate) fn main(mut stdin: Lines) -> DayResult {
    let mut algo = [false; 512];
    for (i, c) in stdin.next().ok_or("Empty input")?.char_indices() {
        if c == '#' {
            algo[i] = true;
        }
    }
    if !matches!(stdin.next(), Some("")) {
        return Err("Expected empty line after enhancement algorithm".to_string());
    }

    let mut map = Map::new(stdin, 100, 2)?;
    for _ in 0..2 {
        map = map.step(&algo);
    }
    let pt1 = map.count_lit();
    for _ in 2..50 {
        map = map.step(&algo);
    }
    let pt2 = map.count_lit();
    Ok((Ok(pt1.to_string()), Ok(pt2.to_string())))
}

struct Map {
    data: Vec<Vec<bool>>,
    size: usize,
    outer_lit: bool,
}
impl Map {
    fn new(lines: Lines, expect_size: usize, expect_rounds: usize) -> Failable<Self> {
        let s = expect_size + expect_rounds;
        let mut map = Vec::with_capacity(s);
        let mut size = None;
        for l in lines {
            let mut mapl = Vec::with_capacity(s);
            for c in l.chars() {
                mapl.push(c == '#');
            }
            match size {
                None => size = Some(mapl.len()),
                Some(x) => {
                    if mapl.len() != x {
                        return Err("Uneven map width".to_string());
                    }
                }
            }
            map.push(mapl)
        }
        match size {
            None => Err("No map lines on input".to_string()),
            Some(x) if x == map.len() => Ok(Self {
                data: map,
                size: x,
                outer_lit: false,
            }),
            _ => Err("Map isn't square shape".to_string()),
        }
    }
    fn get_at(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= self.size as i32 {
            return self.outer_lit;
        }
        if y < 0 || y >= self.size as i32 {
            return self.outer_lit;
        }
        self.data[y as usize][x as usize]
    }
    fn step(&self, algo: &[bool]) -> Self {
        let mut new_map = Vec::with_capacity(self.size + 2);
        for y in -1..self.size as i32 + 1 {
            let mut new_mapl = Vec::with_capacity(self.size + 2);
            for x in -1..self.size as i32 + 1 {
                let mut n = 0;
                for y2 in y - 1..=y + 1 {
                    for x2 in x - 1..=x + 1 {
                        n = n * 2 + self.get_at(x2, y2) as usize;
                    }
                }
                new_mapl.push(algo[n])
            }
            new_map.push(new_mapl)
        }
        Self {
            data: new_map,
            size: self.size + 2,
            outer_lit: !self.outer_lit,
        }
    }
    fn count_lit(&self) -> u32 {
        let mut n = 0;
        for l in &self.data {
            for x in l {
                if *x {
                    n += 1
                }
            }
        }
        n
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(fmt, "Map of size {s}x{s}:", s = self.size)?;
        for l in &self.data {
            for c in l {
                write!(fmt, "{}", if *c { '#' } else { '.' })?
            }
            writeln!(fmt)?
        }
        Ok(())
    }
}
