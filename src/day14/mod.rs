use std::{collections::HashMap, fs, thread, time};

const FILE_PATH: &str = "src/day14/input";

const ANIMATE: bool = false;
const SLEEP_MS: time::Duration = time::Duration::from_millis(100);

type Point = (i64, i64);
struct Boundaries {
    left_edge: i64,
    right_edge: i64,
    bottom_edge: i64,
}
struct Environment {
    objects: HashMap<Point, char>,
    boundaries: Boundaries,
    is_infinite: bool,
}

impl Environment {
    fn get_block(&self, x: i64, y: i64) -> char {
        if let Some(object) = self.objects.get(&(x, y)) {
            return *object;
        } else if self.is_infinite {
            if self.boundaries.bottom_edge + 2 == y {
                return '#';
            }
        } else if x < self.boundaries.left_edge
            || self.boundaries.right_edge < x
            || y > self.boundaries.bottom_edge
        {
            return 'v';
        }

        return '.';
    }

    fn set_block(&mut self, x: i64, y: i64, object: char) {
        self.objects.insert((x, y), object);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Status {
    AtRest,
    ForeverFalling,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Down,
}

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(&input);
    part_two(&input);
}

fn parse_paths(input: &String) -> Vec<Vec<Point>> {
    let mut paths = Vec::new();
    for path_str in input.split("\n").filter(|line| !line.is_empty()) {
        let mut path = Vec::new();
        for line_str in path_str.split(" -> ") {
            let coord_str = line_str.split_once(',').unwrap();
            let x = coord_str.0.parse().unwrap();
            let y = coord_str.1.parse().unwrap();
            path.push((x, y));
        }
        paths.push(path);
    }
    return paths;
}

fn get_boundaries(objects: HashMap<(i64, i64), char>) -> Boundaries {
    let (mut min_x, mut max_x) = (i64::MAX, i64::MIN);
    let (mut min_y, mut max_y) = (i64::MAX, i64::MIN);
    for ((x, y), _) in objects {
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    return Boundaries {
        left_edge: min_x,
        right_edge: max_x,
        bottom_edge: max_y,
    };
}

fn build_environment(input: &String, is_infinite: bool) -> Environment {
    let paths = parse_paths(input);
    let mut objects = HashMap::new();
    for path in paths {
        let mut start = path[0];
        for k in 1..path.len() {
            let end = path[k];
            for y in start.1.min(end.1)..=start.1.max(end.1) {
                objects.insert((start.0, y), '#');
            }
            for x in start.0.min(end.0)..=start.0.max(end.0) {
                objects.insert((x, start.1), '#');
            }
            start = end;
        }
    }
    objects.insert((500, 0), '+');

    return Environment {
        objects: objects.to_owned(),
        boundaries: get_boundaries(objects.to_owned()),
        is_infinite,
    };
}

fn simluate_sand_unit(environment: &mut Environment) -> Status {
    let mut direction = Direction::Down;
    let mut curr_x = 500;
    let mut curr_y = 0;
    loop {
        match direction {
            Direction::Down => match environment.get_block(curr_x, curr_y + 1) {
                'v' => return Status::ForeverFalling,
                '.' => {
                    curr_y += 1;
                }
                '#' | 'o' => {
                    direction = Direction::Left;
                }
                _ => (),
            },
            Direction::Left => match environment.get_block(curr_x - 1, curr_y + 1) {
                'v' => return Status::ForeverFalling,
                '#' | 'o' => {
                    direction = Direction::Right;
                }
                '.' => {
                    curr_x -= 1;
                    curr_y += 1;
                    direction = Direction::Down;
                }
                _ => (),
            },
            Direction::Right => match environment.get_block(curr_x + 1, curr_y + 1) {
                'v' => return Status::ForeverFalling,
                '#' | 'o' => {
                    environment.set_block(curr_x, curr_y, 'o');
                    return Status::AtRest;
                }
                '.' => {
                    curr_x += 1;
                    curr_y += 1;
                    direction = Direction::Down;
                }
                _ => (),
            },
        }
    }
}

fn display(environment: &Environment, clear_screen: bool) {
    if clear_screen {
        print!("\x1bc");
    }
    let boundaries = get_boundaries(environment.objects.to_owned());
    for y in 0..boundaries.bottom_edge {
        for x in boundaries.left_edge..boundaries.right_edge {
            print!("{}", environment.get_block(x, y));
        }
        println!()
    }
}

fn part_one(input: &String) {
    let mut environment = build_environment(&input, false);
    let mut sand_units = 0;
    while simluate_sand_unit(&mut environment) != Status::ForeverFalling {
        sand_units += 1;
        if ANIMATE {
            display(&environment, true);
            thread::sleep(SLEEP_MS);
        }
    }
    println!("{sand_units} units of sand came to rest");
}

fn part_two(input: &String) {
    let mut environment = build_environment(&input, true);
    let mut sand_units = 0;
    while simluate_sand_unit(&mut environment) != Status::ForeverFalling {
        if ANIMATE {
            display(&environment, true);
            thread::sleep(SLEEP_MS);
        }
        sand_units += 1;
        if environment.get_block(500, 0) == 'o' {
            break;
        }
    }
    println!("{sand_units} units of sand came to rest");
}
