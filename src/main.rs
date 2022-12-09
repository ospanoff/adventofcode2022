mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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
        "5" => day5::solution(),
        "6" => day6::solution(),
        "7" => day7::solution(),
        "8" => day8::solution(),
        "9" => day9::solution(),
        day => panic!("No solution for day {day}"),
    }
}
