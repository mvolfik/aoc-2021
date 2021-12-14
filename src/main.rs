mod day1;
mod day2;
mod day12;

fn main() {
    let days: Vec<Option<for<'r> fn(std::str::Lines<'r>) -> Result<_, _>>> = vec![
        Some(crate::day1::main),
        Some(crate::day2::main),
        None,
        None,
        None, // 5
        None,
        None,
        None,
        None,
        None, // 10
        None,
        Some(crate::day12::main),
        None,
        None,
        None, // 15
        None,
        None,
        None,
        None,
        None, // 20
        None,
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
                        match day_fn(lines) {
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
