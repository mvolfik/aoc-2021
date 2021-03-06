use {
    crate::utils::{DayResult, Failable},
    std::str::{FromStr, Lines},
};

fn parseline(l: &str) -> Failable<(&str, i32)> {
    let mut iterator = l.split(' ');
    match (iterator.next(), iterator.next()) {
        (Some(a), Some(b)) => Ok((a, i32::from_str(b).map_err(|e| e.to_string())?)),
        _ => Err("Unexpected line format".to_string()),
    }
}
pub(crate) fn main(stdin: Lines) -> DayResult {
    let commands: Vec<(&str, i32)> = stdin
        .map(parseline)
        .collect::<Failable<Vec<(&str, i32)>>>()?;

    Ok((Ok(run_part1(&commands)), Ok(run_part2(&commands))))
}

fn run_part1(commands: &[(&str, i32)]) -> String {
    let (mut x, mut d) = (0, 0);
    for (c, n) in commands {
        if *c == "up" {
            d -= n
        } else if *c == "down" {
            d += n
        } else if *c == "forward" {
            x += n
        }
    }
    (x * d).to_string()
}
fn run_part2(commands: &[(&str, i32)]) -> String {
    let (mut x, mut d, mut aim) = (0, 0, 0);
    for (c, n) in commands {
        if *c == "up" {
            aim -= n
        } else if *c == "down" {
            aim += n
        } else if *c == "forward" {
            x += n;
            d += n * aim
        }
    }
    (x * d).to_string()
}
