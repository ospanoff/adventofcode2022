mod no_zero_hashmap;

use no_zero_hashmap::NoZeroHashMap;
use std::fs;

const FILE_PATH: &str = "src/day6/input";

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(input.to_owned());
    part_two(input);
}

fn find_marker_pos(signal: Vec<char>, length: usize) -> usize {
    let mut window = NoZeroHashMap::new();
    let mut start_ix = 0;
    for end_ix in 0..signal.len() {
        window.inc(signal[end_ix]);
        if window.len() == length {
            return end_ix + 1;
        }
        if end_ix >= length - 1 {
            window.dec(signal[start_ix]);
            start_ix += 1;
        }
    }
    return 0;
}

fn part_one(input: String) {
    let signal: Vec<char> = input.split("\n").next().unwrap().chars().collect();

    println!(
        "First marker after character {}",
        find_marker_pos(signal, 4)
    );
}

fn part_two(input: String) {
    let signal: Vec<char> = input.split("\n").next().unwrap().chars().collect();

    println!(
        "First marker after character {}",
        find_marker_pos(signal, 14)
    );
}
