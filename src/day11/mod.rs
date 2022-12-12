use std::fs;

const FILE_PATH: &str = "src/day11/input";

const GAME_1_ROUNDS: usize = 20;
const GAME_2_ROUNDS: usize = 10000;

#[derive(Debug)]
struct Operation {
    operation: char,
    operand: Option<u64>,
}

impl Operation {
    pub fn calculate(&self, old: u64) -> u64 {
        match self.operation {
            '+' => return old + self.operand.unwrap_or(old),
            '*' => return old * self.operand.unwrap_or(old),
            operation => panic!("Operation {operation} is not supperted"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    test_mod: u64,
    operation: Operation,
    if_true: usize,
    if_false: usize,
    num_inspected_items: usize,
}

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(&input);
    part_two(&input);
}

fn parse_monkey(input: String) -> Monkey {
    let mut monkey = Monkey {
        items: Vec::new(),
        test_mod: 0,
        operation: Operation {
            operation: ' ',
            operand: None,
        },
        if_true: 0,
        if_false: 0,
        num_inspected_items: 0,
    };

    for mut line in input.split("\n") {
        line = line.trim();
        if line.starts_with("Starting items") {
            let starting_items_str = line.split(": ").last().unwrap();
            monkey.items = starting_items_str
                .split(", ")
                .map(|num_str| num_str.parse().unwrap())
                .collect();
        } else if line.starts_with("Operation") {
            let operation_str = line.split("= old ").last().unwrap();
            monkey.operation.operation = operation_str.chars().next().unwrap();
            monkey.operation.operand = operation_str
                .chars()
                .skip(2)
                .collect::<String>()
                .parse()
                .ok();
        } else if line.starts_with("Test") {
            let test_str = line.split("divisible by ").last().unwrap();
            monkey.test_mod = test_str.parse().unwrap();
        } else if line.starts_with("If true") {
            let test_str = line.split("throw to monkey ").last().unwrap();
            monkey.if_true = test_str.parse().unwrap();
        } else if line.starts_with("If false") {
            let test_str = line.split("throw to monkey ").last().unwrap();
            monkey.if_false = test_str.parse().unwrap();
        }
    }
    return monkey;
}

fn parse_monkeys(input: &String) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_str in input.split("\n\n") {
        let monkey_raw = monkey_str.split_once("\n").unwrap().1;
        monkeys.push(parse_monkey(monkey_raw.to_string()));
    }
    return monkeys;
}

fn play_round(monkeys: &mut Vec<Monkey>, normalize: impl Fn(u64) -> u64) {
    for monkey_id in 0..monkeys.len() {
        let mut new_items_true = Vec::new();
        let mut new_items_false = Vec::new();

        let monkey = &mut monkeys[monkey_id];

        let true_action = monkey.if_true;
        let false_action = monkey.if_false;

        for item_ix in 0..monkey.items.len() {
            let mut worry_level = monkey.operation.calculate(monkey.items[item_ix]);
            worry_level = normalize(worry_level);
            if worry_level % monkey.test_mod == 0 {
                new_items_true.push(worry_level);
            } else {
                new_items_false.push(worry_level);
            }
        }
        monkey.num_inspected_items += monkey.items.len();
        monkey.items.clear();

        // This is all done separately and using spearate vars
        // because of Rust's memory management
        monkeys[true_action].items.append(&mut new_items_true);
        monkeys[false_action].items.append(&mut new_items_false);
    }
}

fn print_monkey_business(monkeys: &mut Vec<Monkey>) {
    monkeys.sort_by(|a, b| b.num_inspected_items.cmp(&a.num_inspected_items));
    println!(
        "Level of monkey business is {}",
        monkeys[0].num_inspected_items * monkeys[1].num_inspected_items
    );
}

fn part_one(input: &String) {
    let mut monkeys = parse_monkeys(input);

    for _ in 0..GAME_1_ROUNDS {
        play_round(&mut monkeys, |x| x / 3);
    }

    print_monkey_business(&mut monkeys);
}

fn part_two(input: &String) {
    let mut monkeys = parse_monkeys(input);

    let norm_mod = monkeys
        .iter()
        .map(|monkey| monkey.test_mod)
        .product::<u64>();

    for _ in 0..GAME_2_ROUNDS {
        play_round(&mut monkeys, |x| x % norm_mod);
    }
    print_monkey_business(&mut monkeys);
}
