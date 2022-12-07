use std::{collections::HashMap, fs};

struct Data {
    pub name: String,
    pub is_dir: bool,
    pub size: u32,
}

type FileSystem = HashMap<String, HashMap<String, Data>>;

const FILE_PATH: &str = "src/day7/input";

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(input.to_owned());
    part_two(input);
}

fn build_file_system(input: String) -> FileSystem {
    let mut fs = HashMap::new();
    let mut cur_path: Vec<&str> = Vec::new();
    for line in input.split("\n").filter(|line| !line.is_empty()) {
        if line.starts_with("$ cd") {
            let dir = &line[5..];
            if dir == ".." {
                cur_path.pop();
            } else {
                cur_path.push(dir);
            }
        } else if line.starts_with("$ ls") {
        } else if line.starts_with("dir") {
            let name = line[4..].to_owned();
            let files = fs.entry(cur_path.join("/")).or_insert(HashMap::new());
            files.insert(
                name.to_owned(),
                Data {
                    name: name.to_owned(),
                    is_dir: true,
                    size: 0,
                },
            );
        } else {
            let mut split = line.split(" ");
            let size = split.next().unwrap().parse::<u32>().unwrap();
            let name = split.last().unwrap().to_owned();
            let files = fs.entry(cur_path.join("/")).or_insert(HashMap::new());
            files.insert(
                name.to_owned(),
                Data {
                    name: name.to_owned(),
                    is_dir: false,
                    size,
                },
            );
        }
    }
    return fs;
}

fn get_dir_sizes(fs: FileSystem) -> HashMap<String, u32> {
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut paths_sorted = fs.keys().to_owned().collect::<Vec<&String>>();
    // Sort to calculate from the leaves. That way we don't need recursion,
    // as we'll have all the children sizes calculated for parent folders
    paths_sorted.sort();
    paths_sorted.reverse();
    for path_name in paths_sorted {
        let mut total_size = 0;
        for file in fs.get(path_name).unwrap().values() {
            if file.is_dir {
                let full_path = vec![path_name.as_str(), file.name.as_str()].join("/");
                total_size += dir_sizes.get(&full_path).unwrap()
            } else {
                total_size += file.size;
            }
        }
        dir_sizes.insert(path_name.to_owned(), total_size);
    }
    return dir_sizes;
}

fn part_one(input: String) {
    let fs = build_file_system(input);
    let dir_sizes = get_dir_sizes(fs);

    let total_size: u32 = dir_sizes.values().filter(|&&size| size <= 100000).sum();

    println!("Total size for files to delete {total_size}\n");
}

fn part_two(input: String) {
    let fs = build_file_system(input);
    let dir_sizes = get_dir_sizes(fs);

    let free_space = 70000000 - dir_sizes["/"];
    let needed_space = 30000000 - free_space;

    println!("Free space {free_space}. Needed space {needed_space}");

    let min_size: u32 = dir_sizes
        .values()
        .filter(|&&size| size >= needed_space)
        .min()
        .unwrap()
        .to_owned();

    println!("Size of the dir to be deleted is {min_size}");
}
