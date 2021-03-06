#![deny(clippy::all)]
#![allow(clippy::type_complexity)]

mod day1;
mod day12;
mod day15;
mod day16;
mod day18;
mod day2;
mod day20;
mod day21;
mod day6;
mod utils;

fn main() {
    if let Some(x) = std::env::args().nth(1) {
        test(&x);
        return;
    }
    use crate::utils::DayResult;
    let days: Vec<Option<for<'r> fn(std::str::Lines<'r>) -> DayResult>> = vec![
        Some(crate::day1::main),
        Some(crate::day2::main),
        None,
        None,
        None, // 5
        Some(crate::day6::main),
        None,
        None,
        None,
        None, // 10
        None,
        Some(crate::day12::main),
        None,
        None,
        Some(crate::day15::main), // 15
        Some(crate::day16::main),
        None,
        Some(crate::day18::main),
        None,
        Some(crate::day20::main), // 20
        Some(crate::day21::main),
        None,
        None,
        None,
        None, // 25
    ];
    for (i, day_fn) in days.into_iter().enumerate() {
        print!("Day {}:", i + 1);
        match day_fn {
            None => {
                println!(" Not implemented (yet)")
            }
            Some(day_fn) => {
                match std::fs::read_to_string(std::path::Path::new(
                    format!("inputs/{}", i + 1).as_str(),
                )) {
                    Err(e) => {
                        println!(" Failed to open input file: {}", e)
                    }
                    Ok(input_data) => {
                        let lines = input_data.lines();
                        let now = std::time::Instant::now();
                        let res = day_fn(lines);
                        let duration = now.elapsed();
                        print!(" (took {:?})", duration);
                        match res {
                            Err(e) => {
                                println!(" Exited with error: {}", e)
                            }
                            Ok((a, b)) => {
                                println!();
                                for (i, day_res) in vec![a, b].into_iter().enumerate() {
                                    println!(
                                        "  Part {}: {}",
                                        i + 1,
                                        day_res.unwrap_or_else(|e| format!(
                                            "Exited with error: {}",
                                            e
                                        ))
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn test(p: &str) {
    let s = std::fs::read_to_string(std::path::Path::new(&format!("/tmp/{}", p))).unwrap();
    println!("{:?}", day20::main(s.lines()).unwrap())
}
