mod solutions;
mod util;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    match day.as_str() {
        "1" => solutions::solution_01::run(),
        "2" => solutions::solution_02::run(),
        "3" => solutions::solution_03::run(),
        "4" => solutions::solution_04::run(),
        "5" => solutions::solution_05::run(),
        _ => println!("Provide a valid day"),
    }
}
