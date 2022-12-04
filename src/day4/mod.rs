use std::fs;

const FILE_PATH: &str = "src/day4/input";

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(input.clone());
    part_two(input);
}

fn is_contained(elf1: (u32, u32), elf2: (u32, u32)) -> bool {
    if elf1.0 < elf2.0 {
        return elf1.1 >= elf2.1;
    } else if elf1.0 > elf2.0 {
        return elf1.1 <= elf2.1;
    }
    return true;
}

fn get_parsed_pairs(input: String) -> Vec<((u32, u32), (u32, u32))> {
    return input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let elves_str = line.split(',');
            let mut elves = Vec::new();
            for elf_str in elves_str {
                let mut elf = elf_str.split('-');
                elves.push((
                    elf.next().unwrap().parse::<u32>().unwrap(),
                    elf.next().unwrap().parse::<u32>().unwrap(),
                ))
            }
            return (
                elves.get(0).unwrap().to_owned(),
                elves.get(1).unwrap().to_owned(),
            );
        })
        .collect();
}

fn part_one(input: String) {
    let total_fully_containing = get_parsed_pairs(input)
        .iter()
        .filter(|(efl1, efl2)| is_contained(*efl1, *efl2))
        .count();

    println!("Total of fully containing pairs is {total_fully_containing}")
}

fn do_overlap(elf1: (u32, u32), elf2: (u32, u32)) -> bool {
    if elf1.1 < elf2.0 {
        return false;
    } else if elf1.0 > elf2.1 {
        return false;
    }
    return true;
}

fn part_two(input: String) {
    let total_overlapping = get_parsed_pairs(input)
        .iter()
        .filter(|(efl1, efl2)| do_overlap(*efl1, *efl2))
        .count();

    println!("Total of overlapping pairs is {total_overlapping}")
}
