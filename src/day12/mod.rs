mod dijkstra;

use std::{collections::HashMap, fs};

use crate::day12::dijkstra::{dijkstra, Vertex};

const FILE_PATH: &str = "src/day12/input";
pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(&input);
    part_two(&input);
}

fn in_range(matrix: &Vec<Vec<Vertex>>, pos: (i32, i32)) -> bool {
    return 0 <= pos.0
        && pos.0 < matrix.len() as i32
        && 0 <= pos.1
        && pos.1 < matrix[0].len() as i32;
}

fn is_achieveable(matrix: &Vec<Vec<Vertex>>, pos: (i32, i32), nbr: (i32, i32)) -> bool {
    return in_range(matrix, pos)
        && in_range(matrix, nbr)
        && matrix[nbr.0 as usize][nbr.1 as usize].height as i32
            - matrix[pos.0 as usize][pos.1 as usize].height as i32
            <= 1;
}

fn build_matrix(input: &String) -> (Vec<Vec<Vertex>>, Vertex, Vertex) {
    let mut matrix: Vec<Vec<Vertex>> = Vec::new();
    let mut start = Vertex::new((0, 0), 0);
    let mut end = Vertex::new((0, 0), 0);

    for (i, line) in input
        .split("\n")
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        let mut row = Vec::new();
        for (j, height_chr) in line.chars().enumerate() {
            let height = match height_chr {
                'S' => 'a' as u32 - 'a' as u32,
                'E' => 'z' as u32 - 'a' as u32,
                height_chr => height_chr as u32 - 'a' as u32,
            };
            let vertex = Vertex::new((i, j), height);
            if height_chr == 'S' {
                start = vertex;
            } else if height_chr == 'E' {
                end = vertex;
            }
            row.push(vertex);
        }
        matrix.push(row);
    }
    return (matrix, start, end);
}

fn part_one(input: &String) {
    let (matrix, start, end) = build_matrix(input);

    let mut adjacency_list = HashMap::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let list = adjacency_list.entry(matrix[i][j]).or_insert(Vec::new());
            // left neighbour
            if is_achieveable(&matrix, (i as i32, j as i32), (i as i32, j as i32 - 1)) {
                list.push((matrix[i][j - 1], 1));
            }
            // right neighbour
            if is_achieveable(&matrix, (i as i32, j as i32), (i as i32, j as i32 + 1)) {
                list.push((matrix[i][j + 1], 1));
            }
            // top neighbour
            if is_achieveable(&matrix, (i as i32, j as i32), (i as i32 - 1, j as i32)) {
                list.push((matrix[i - 1][j], 1));
            }
            // bottom neighbour
            if is_achieveable(&matrix, (i as i32, j as i32), (i as i32 + 1, j as i32)) {
                list.push((matrix[i + 1][j], 1));
            }
        }
    }

    let distances = dijkstra(start, &adjacency_list);
    println!("Feweest steps to {:?} is {:?}", end, distances.get(&end));
}

fn part_two(input: &String) {
    let (matrix, _, end) = build_matrix(input);

    let mut adjacency_list = HashMap::new();
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let list = adjacency_list.entry(matrix[i][j]).or_insert(Vec::new());
            // left neighbour
            if is_achieveable(&matrix, (i as i32, j as i32 - 1), (i as i32, j as i32)) {
                list.push((matrix[i][j - 1], 1));
            }
            // right neighbour
            if is_achieveable(&matrix, (i as i32, j as i32 + 1), (i as i32, j as i32)) {
                list.push((matrix[i][j + 1], 1));
            }
            // top neighbour
            if is_achieveable(&matrix, (i as i32 - 1, j as i32), (i as i32, j as i32)) {
                list.push((matrix[i - 1][j], 1));
            }
            // bottom neighbour
            if is_achieveable(&matrix, (i as i32 + 1, j as i32), (i as i32, j as i32)) {
                list.push((matrix[i + 1][j], 1));
            }
        }
    }

    let distances = dijkstra(end, &adjacency_list);
    let closest_vertex = distances
        .iter()
        .filter(|(vertex, _)| vertex.height == 0)
        .min_by(|v1, v2| v1.1.cmp(v2.1))
        .unwrap();
    println!("Feweest steps from {:?} is {:?}", end, closest_vertex.1);
}
