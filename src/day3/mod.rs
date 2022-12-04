use std::collections::HashSet;
use std::fs;

const FILE_PATH: &str = "src/day3/input";

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(input.clone());
    part_two(input);
}

fn part_one(input: String) {
    let total_priority = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            let first_rucksack: HashSet<char> =
                line[..line.len() / 2].chars().into_iter().collect();
            let second_rucksack: HashSet<char> =
                line[line.len() / 2..].chars().into_iter().collect();
            return first_rucksack
                .intersection(&second_rucksack)
                .cloned()
                .collect::<HashSet<char>>();
        })
        .map(|common_item| return get_priority(common_item))
        .sum::<u32>();

    println!("Total priority is {total_priority}")
}

fn get_priority(item: char) -> u32 {
    if item >= 'a' && item <= 'z' {
        return item as u32 - 'a' as u32 + 1;
    }
    return item as u32 - 'A' as u32 + 27;
}

fn part_two(input: String) {
    let rucksacks = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| return line.chars().into_iter().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    let mut total_priority = 0;
    for i in (0..rucksacks.len()).step_by(3) {
        let common_item = rucksacks[i]
            .intersection(&rucksacks[i + 1])
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&rucksacks[i + 2])
            .cloned()
            .collect::<HashSet<char>>();

        total_priority += get_priority(common_item.iter().next().cloned().unwrap())
    }

    println!("Total priority is {total_priority}")
}
