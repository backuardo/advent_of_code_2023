mod solutions;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match day.as_str() {
        "1" => solutions::solution_01::run(),
        _ => println!("Provide a valid day"),
    }
}
