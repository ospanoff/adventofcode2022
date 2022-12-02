mod day1;
mod day2;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Enter a day for solution!");
    }

    match args[1].as_str() {
        "1" => day1::solution(),
        "2" => day2::solution(),
        day => panic!("No solution for day {day}"),
    }
}
