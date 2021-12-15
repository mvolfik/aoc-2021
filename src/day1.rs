use {
    crate::utils::DayResult,
    std::str::{FromStr, Lines},
};

pub(crate) fn main(stdin: Lines) -> DayResult {
    let numbers: Vec<i32> = stdin
        .map(i32::from_str)
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| format!("Expected to parse number: {}", e))?;
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
