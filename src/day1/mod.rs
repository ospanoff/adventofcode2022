use std::fs;

const FILE_PATH: &str = "src/day1/input";

pub fn solution() {
    let contents =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    let mut elf_calories: Vec<i32> = contents
        .split("\n\n")
        .map(|lines| {
            lines
                .trim()
                .split("\n")
                .map(|num_str| num_str.parse::<i32>().unwrap())
        })
        .map(|nums| nums.sum::<i32>())
        .collect();
    elf_calories.sort_unstable();

    println!("Max is {0}", elf_calories[elf_calories.len() - 1]);
    println!(
        "Top3 sum is {0}",
        elf_calories[elf_calories.len() - 3..].iter().sum::<i32>()
    );
}
