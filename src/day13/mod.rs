use std::fs;

const FILE_PATH: &str = "src/day13/input";

#[derive(Debug, Clone)]
struct Item {
    pub value: Option<i32>,
    pub children: Vec<Box<Item>>,
}

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(&input);
    part_two(&input);
}

fn parse_list(line: &String, start: usize) -> (Item, usize) {
    let mut item = Item {
        value: None,
        children: Vec::new(),
    };
    let mut cur_num = String::new();
    let mut i = start;
    while i < line.len() {
        match line.chars().nth(i).unwrap() {
            '[' => {
                let (child, end) = parse_list(line, i + 1);
                i = end;
                item.children.push(Box::from(child));
            }
            ch if ch.is_ascii_digit() => cur_num.push(ch),
            ',' => {
                if cur_num.len() > 0 {
                    item.children.push(Box::from(Item {
                        value: cur_num.parse::<i32>().ok(),
                        children: Vec::new(),
                    }));
                    cur_num.clear();
                }
            }
            ']' => {
                if cur_num.len() > 0 {
                    item.children.push(Box::from(Item {
                        value: cur_num.parse::<i32>().ok(),
                        children: Vec::new(),
                    }));
                    cur_num.clear();
                }
                return (item, i);
            }
            _ => (),
        }
        i += 1;
    }
    return (item, i + 1);
}

fn _print(root: Item, depth: usize) {
    let tabs = vec![" "; 4 * depth];
    println!("{}{:?}", tabs.join(""), root.value);
    for child in root.children {
        _print(*child, depth + 1);
    }
}

fn in_right_order(root1: Item, root2: Item) -> i32 {
    if let Some(val1) = root1.value {
        if let Some(val2) = root2.value {
            return (val1 - val2).signum();
        } else {
            return in_right_order(
                Item {
                    value: None,
                    children: vec![Box::new(root1)],
                },
                root2,
            );
        }
    } else if root2.value.is_some() {
        return in_right_order(
            root1,
            Item {
                value: None,
                children: vec![Box::new(root2)],
            },
        );
    }

    let min = if root1.children.len() < root2.children.len() {
        root1.children.len()
    } else {
        root2.children.len()
    };
    for i in 0..min {
        let in_order = in_right_order(*root1.children[i].to_owned(), *root2.children[i].to_owned());
        if in_order != 0 {
            return in_order;
        }
    }
    return (root1.children.len() as i32 - root2.children.len() as i32).signum();
}

fn part_one(input: &String) {
    let mut sum_ixs = 0;
    for (i, group_str) in input
        .split("\n\n")
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        let group = group_str.split_once("\n").unwrap();
        let (root1, _) = parse_list(&String::from(group.0), 1);
        let (root2, _) = parse_list(&String::from(group.1), 1);
        if in_right_order(root1, root2) < 0 {
            sum_ixs += i + 1;
        }
    }
    println!("Sum of indexes is {sum_ixs}");
}

fn part_two(_input: &String) {}
