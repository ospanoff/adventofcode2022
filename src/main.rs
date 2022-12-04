mod day1;
mod day2;
mod day3;
mod day4;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        panic!("Enter a day for solution!");
    }

    match args[1].as_str() {
        "1" => day1::solution(),
        "2" => day2::solution(),
        "3" => day3::solution(),
        "4" => day4::solution(),
        day => panic!("No solution for day {day}"),
    }
}
