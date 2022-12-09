use std::{collections::HashSet, fs};

const FILE_PATH: &str = "src/day9/input";
const VERBOSE: bool = false;

type Position = (i32, i32);

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(input.to_owned());
    part_two(input);
}

fn move_head(mut head: Position, direction: char) -> Position {
    match direction {
        'R' => head.0 += 1,
        'L' => head.0 -= 1,
        'U' => head.1 += 1,
        'D' => head.1 -= 1,
        _ => panic!("Move is not supported"),
    }
    return head;
}

fn follow_head(head: Position, mut tail: Position) -> Position {
    if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
        tail.0 += (head.0 - tail.0).signum();
        tail.1 += (head.1 - tail.1).signum();
    }

    return tail;
}

fn print_state_part_one(head: Position, tail: Position) {
    for j in (0..5).rev() {
        for i in 0..6 {
            if head.0 == i && head.1 == j {
                print!("H")
            } else if tail.0 == i && tail.1 == j {
                print!("T")
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

fn part_one(input: String) {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            let mut movement = line.split(" ");
            let direction = movement.next().unwrap().chars().next().unwrap();
            let num_steps = movement.last().unwrap().parse::<usize>().unwrap();
            return vec![direction; num_steps];
        })
        .map(|direction| {
            head = move_head(head, direction);
            tail = follow_head(head, tail);
            if VERBOSE {
                print_state_part_one(head, tail);
            }
            visited.insert(tail);
        })
        .for_each(drop);

    println!("Visited positions {}", visited.len());
}

fn part_two(input: String) {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut head = (0, 0);
    let mut tails = vec![(0, 0); 9];

    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            let mut movement = line.split(" ");
            let direction = movement.next().unwrap().chars().next().unwrap();
            let num_steps = movement.last().unwrap().parse::<usize>().unwrap();
            return vec![direction; num_steps];
        })
        .map(|direction| {
            head = move_head(head, direction);
            let mut head_ptr = head;

            for i in 0..tails.len() {
                tails[i] = follow_head(head_ptr, tails[i]);
                head_ptr = tails[i];
            }
            visited.insert(head_ptr);
        })
        .for_each(drop);

    println!("Visited positions {}", visited.len());
}
