use std::fs;

const FILE_PATH: &str = "src/day10/input";

const SNAPSHOT_STEP: i32 = 40;

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(input.to_owned());
    part_two(input);
}

fn part_one(input: String) {
    let mut register_x = 1;
    let mut current_cycle = 1;
    let mut sum_signal_strengths = 0;
    let mut snapshot_cycle = 20;
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut operation = line.split(' ');
            let command = operation.next().unwrap();
            let operand = operation.last().unwrap_or("0").parse::<i32>().unwrap();

            if current_cycle == snapshot_cycle {
                sum_signal_strengths += current_cycle * register_x;
                snapshot_cycle += SNAPSHOT_STEP;
            }

            match command {
                "noop" => current_cycle += 1,
                "addx" => {
                    current_cycle += 1;
                    if current_cycle == snapshot_cycle {
                        sum_signal_strengths += current_cycle * register_x;
                        snapshot_cycle += SNAPSHOT_STEP;
                    }
                    register_x += operand;
                    current_cycle += 1;
                }
                _ => (),
            }
        })
        .for_each(drop);

    println!("Sum of signal strengthes {}", sum_signal_strengths);
}

fn should_draw(sprite_pos: i32, pixel_position: i32) -> bool {
    return sprite_pos - 1 <= pixel_position && pixel_position <= sprite_pos + 1;
}

fn part_two(input: String) {
    let mut register_x = 1;
    let mut current_cycle = 1;
    let mut pixels = vec!['.'; 240];
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut operation = line.split(' ');
            let command = operation.next().unwrap();
            let operand = operation.last().unwrap_or("0").parse::<i32>().unwrap();

            if should_draw(register_x, (current_cycle - 1) % SNAPSHOT_STEP) {
                pixels[current_cycle as usize - 1] = '#';
            }

            match command {
                "noop" => current_cycle += 1,
                "addx" => {
                    current_cycle += 1;
                    if should_draw(register_x, (current_cycle - 1) % SNAPSHOT_STEP) {
                        pixels[current_cycle as usize - 1] = '#';
                    }
                    current_cycle += 1;
                    register_x += operand;
                }
                _ => (),
            }
        })
        .for_each(drop);

    for i in (0..pixels.len()).step_by(SNAPSHOT_STEP as usize) {
        println!(
            "{}",
            &pixels[i..i + SNAPSHOT_STEP as usize]
                .iter()
                .collect::<String>()
        );
    }
}
