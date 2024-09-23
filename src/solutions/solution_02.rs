use crate::util::read_lines;
use regex::Regex;

#[derive(Debug)]
struct Combination {
    red: i32,
    green: i32,
    blue: i32,
}

pub fn run() {
    let total_sum: i32 = read_lines("inputs/input_02.txt")
        .unwrap()
        .flatten()
        .filter_map(|line| {
            // Compile regex
            let color_regex = Regex::new(r"(\d+) (green|red|blue)").unwrap();

            // Get color counts for each section
            let mut combinations: Vec<Combination> = Vec::new();
            let sections: Vec<&str> = line.split(';').collect();

            for section in sections {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                for capture in color_regex.captures_iter(section) {
                    let count: i32 = capture[1].parse().unwrap();
                    match &capture[2] {
                        "red" => red = count,
                        "green" => green = count,
                        "blue" => blue = count,
                        _ => {}
                    }
                }

                combinations.push(Combination { red, green, blue })
            }

            // Part 2, find largest number of each color
            let max_red = combinations
                .iter()
                .map(|combination| combination.red)
                .max()
                .unwrap();
            let max_green = combinations
                .iter()
                .map(|combination| combination.green)
                .max()
                .unwrap();
            let max_blue = combinations
                .iter()
                .map(|combination| combination.blue)
                .max()
                .unwrap();

            // Part 2, return "power" of the minimal set
            Some(max_red * max_green * max_blue)
        })
        .sum();

    println!("Solution: {}", total_sum);
}
