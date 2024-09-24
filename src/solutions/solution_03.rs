use crate::util::read_lines;
use std::collections::HashSet;

const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (1, 0),
    (0, -1),
    (0, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
    (1, 1),
];

/// Check if `r` and `c` exist as indices in 2D vec
fn is_valid_position(r: i32, c: i32, num_rows: i32, num_cols: i32) -> bool {
    r >= 0 && r < num_rows && c >= 0 && c < num_cols
}

/// Extracts a full number from the grid, starting at (r, c)
fn extract_full_number(r: i32, c: i32, grid: &Vec<Vec<char>>) -> Option<i32> {
    let cols = grid[0].len();
    let mut start_c = c;
    let mut end_c = c;

    // Traverse left to find the start of the number
    while start_c > 0 && grid[r as usize][(start_c - 1) as usize].is_digit(10) {
        start_c -= 1;
    }

    // Traverse right to find the end of the number
    while end_c < (cols - 1) as i32 && grid[r as usize][(end_c + 1) as usize].is_digit(10) {
        end_c += 1;
    }

    // Collect full number as string
    let number_str: String = (start_c..=end_c)
        .map(|col| grid[r as usize][col as usize])
        .collect();

    // Parse string to int
    number_str.parse::<i32>().ok()
}

pub fn run() {
    // Preprocess a grid (2D vec) of chars
    let grid: Vec<Vec<char>> = read_lines("inputs/input_03.txt")
        .unwrap() // Ignore any file read errors
        .filter_map(Result::ok) // Unwrap lines, again ignoring read errors
        .map(|line| line.chars().collect()) // Convert each line to Vec<char>
        .collect(); // Into final type

    let rows = grid.len();
    let cols = grid[0].len();
    let mut total_sum = 0;

    for row in 0..rows {
        let mut col = 0;
        while col < cols {
            if grid[row][col] == '*' {
                // Character is a "gear"
                let mut number_values: HashSet<i32> = HashSet::new();
                for (dx, dy) in DIRECTIONS {
                    // Attempt to find exactly 2 adjacent number values
                    let adj_row = row as i32 + dx;
                    let adj_col = col as i32 + dy;
                    if is_valid_position(adj_row, adj_col, rows as i32, cols as i32)
                        && grid[adj_row as usize][adj_col as usize].is_digit(10)
                    {
                        if let Some(number_value) = extract_full_number(adj_row, adj_col, &grid) {
                            number_values.insert(number_value);
                        }
                    }
                }

                // Add the "gear ratio" to total_sum if we have exactly 2 adjacent numbers
                if number_values.len() == 2 {
                    let product = number_values.iter().product::<i32>();
                    total_sum += product;
                }
            }
            // Move to next char
            col += 1;
        }
    }

    println!("Solution: {}", total_sum);
}
