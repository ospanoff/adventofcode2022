use std::fs;

const FILE_PATH: &str = "src/day2/input";

pub fn solution() {
    let strategy =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(strategy.clone());
    part_two(strategy);
}

fn part_one(strategy: String) {
    let total_score: i32 = strategy
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|moves| {
            let mut score = 0;
            match moves.chars().nth(2).unwrap() {
                'X' => score = 1,
                'Y' => score = 2,
                'Z' => score = 3,
                _ => (),
            }

            match moves {
                "C X" | "A Y" | "B Z" => score += 6,
                "A X" | "B Y" | "C Z" => score += 3,
                _ => (),
            }

            return score;
        })
        .sum();

    println!("Total score for part 1 is {total_score}");
}

fn part_two(strategy: String) {
    let total_score: i32 = strategy
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|moves| {
            let mut score = 0;

            match calculate_move(moves.chars().nth(0).unwrap(), moves.chars().nth(2).unwrap()) {
                'A' => score = 1,
                'B' => score = 2,
                'C' => score = 3,
                _ => (),
            }

            match moves.chars().nth(2).unwrap() {
                'X' => score += 0,
                'Y' => score += 3,
                'Z' => score += 6,
                _ => (),
            }

            return score;
        })
        .sum();

    println!("Total score for part 2 is {total_score}");
}

fn calculate_move(opponent_move: char, outcome: char) -> char {
    match outcome {
        'X' => match opponent_move {
            'A' => return 'C',
            'B' => return 'A',
            'C' => return 'B',
            _ => return ' ',
        },
        'Y' => return opponent_move,
        'Z' => match opponent_move {
            'A' => return 'B',
            'B' => return 'C',
            'C' => return 'A',
            _ => return ' ',
        },
        _ => return ' ',
    }
}
