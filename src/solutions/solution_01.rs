use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() {
    let string_to_digit = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let total_sum: i32 = read_lines("inputs/input_01.txt")
        .unwrap()
        .flatten()
        .filter_map(|line| {
            // Find the first and last number representations
            let mut first_digit: Option<i32> = None;
            let mut last_digit: Option<i32> = None;

            let mut i = 0;
            while i < line.len() {
                // Check if current char is beginning of a word representation
                if let Some((_, &digit)) = string_to_digit
                    .iter()
                    .find(|(str, _)| line[i..].starts_with(*str))
                {
                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    }
                    last_digit = Some(digit);
                    i += 1; // Increment by 1, numbers may overlap
                    continue;
                }

                // Check if current char is digit representation
                if let Some(&digit) = string_to_digit.get(&line[i..i + 1]) {
                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    }
                    last_digit = Some(digit);
                    i += 1;
                    continue;
                }

                // No match found, move on
                i += 1;
            }

            match (first_digit, last_digit) {
                (Some(first), Some(last)) => {
                    let number_str = format!("{}{}", first, last);
                    number_str.parse::<i32>().ok()
                }
                _ => {
                    println!("NOT FOUND");
                    Some(0)
                }
            }
        })
        .sum();

    println!("Solution: {}", total_sum);
}

/// Read lines from a file
/// From https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
