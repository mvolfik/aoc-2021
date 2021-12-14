use std::str::FromStr;
use std::str::Lines;

pub(crate) fn main(
    stdin: Lines,
) -> Result<(Result<String, String>, Result<String, String>), String> {
    let numbers: Vec<i32> = stdin
        .map(|l| i32::from_str(l))
        .collect::<Result<Vec<i32>, _>>()
        .or(Err("Hello"))?;
    let mut n1 = 0;
    for i in 1..numbers.len() {
        if numbers[i] > numbers[i - 1] {
            n1 += 1
        }
    }
    let mut n2 = 0;
    for i in 3..numbers.len() {
        if numbers[i] > numbers[i - 3] {
            n2 += 1
        }
    }
    Ok((Ok(n1.to_string()), Ok(n2.to_string())))
}
