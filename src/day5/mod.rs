use regex::Regex;
use std::fs;

const FILE_PATH: &str = "src/day5/input";

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(input.to_owned());
    part_two(input);
}

fn parse_stacks(input: String) -> Vec<Vec<char>> {
    let levels: Vec<&str> = input
        .split("\n\n")
        .next()
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .collect();

    let num_stacks = levels[levels.len() - 1]
        .trim()
        .split("   ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];

    for level_stacks in levels[..levels.len() - 1].iter().rev() {
        for i in 0..num_stacks {
            let item = level_stacks.chars().nth(4 * i + 1).unwrap();
            if item != ' ' {
                let stack = stacks.get_mut(i).unwrap();
                stack.push(item);
            }
        }
    }
    return stacks;
}

fn parse_moves(input: String) -> Vec<(u32, u32, u32)> {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    return input
        .split("\n\n")
        .last()
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| re.captures(line).unwrap())
        .map(|cap| {
            (
                cap[1].parse::<u32>().unwrap(),
                cap[2].parse::<u32>().unwrap() - 1,
                cap[3].parse::<u32>().unwrap() - 1,
            )
        })
        .collect();
}

fn part_one(input: String) {
    let mut stacks = parse_stacks(input.to_owned());
    for item_move in parse_moves(input) {
        for _ in 0..item_move.0 {
            let stack_out = stacks.get_mut(item_move.1 as usize).unwrap();
            let item = stack_out.pop().unwrap();
            let stack_in = stacks.get_mut(item_move.2 as usize).unwrap();
            stack_in.push(item);
        }
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!()
}

fn part_two(input: String) {
    let mut stacks = parse_stacks(input.to_owned());
    for item_move in parse_moves(input) {
        // This is better done with linked lists
        let stack_out = &stacks[item_move.1 as usize];
        let ptr = stack_out.len() - item_move.0 as usize;

        let mut items = Vec::from(&stack_out.as_slice()[ptr..]);

        stacks[item_move.1 as usize] = Vec::from(&stack_out.as_slice()[..ptr]);

        let stack_in = stacks.get_mut(item_move.2 as usize).unwrap();
        stack_in.append(&mut items);
    }
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!()
}
