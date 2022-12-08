use std::fs;

const FILE_PATH: &str = "src/day8/input";

pub fn solution() {
    let input =
        fs::read_to_string(FILE_PATH).expect(format!("Couldn't read file {FILE_PATH}").as_str());

    part_one(input.to_owned());
    part_two(input);
}

fn parse_matrix(input: String) -> Vec<Vec<u8>> {
    return input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|height_str| height_str as u8 - '0' as u8)
                .collect()
        })
        .collect();
}

fn is_visible(matrix: Vec<Vec<u8>>) -> Vec<Vec<bool>> {
    let mut visible = vec![vec![true; matrix[0].len()]; matrix.len()];
    // traverse by rows
    for i in 1..matrix.len() - 1 {
        let mut max = matrix[i][0];
        for j in 1..matrix[i].len() - 1 {
            if matrix[i][j] > max {
                max = matrix[i][j];
                visible[i][j] = true;
            } else {
                visible[i][j] = false;
            }
        }

        let mut max = matrix[i][matrix[i].len() - 1];
        for j in (1..matrix[i].len() - 1).rev() {
            if matrix[i][j] > max {
                max = matrix[i][j];
                visible[i][j] |= true;
            } else {
                visible[i][j] |= false;
            }
        }
    }
    // traverse by columns
    for j in 1..matrix[0].len() - 1 {
        let mut max = matrix[0][j];
        for i in 1..matrix.len() - 1 {
            if matrix[i][j] > max {
                max = matrix[i][j];
                visible[i][j] |= true;
            } else {
                visible[i][j] |= false;
            }
        }

        let mut max = matrix[matrix.len() - 1][j];
        for i in (1..matrix.len() - 1).rev() {
            if matrix[i][j] > max {
                max = matrix[i][j];
                visible[i][j] |= true;
            } else {
                visible[i][j] |= false;
            }
        }
    }
    return visible;
}

fn part_one(input: String) {
    let matrix = parse_matrix(input);
    let num_visible = is_visible(matrix)
        .iter()
        .flatten()
        .map(|&value| value as u32)
        .sum::<u32>();
    println!("Number of visible trees is {num_visible}");
}

// naive solution
fn scenic_score(matrix: Vec<Vec<u8>>) -> Vec<Vec<u32>> {
    let mut scores = vec![vec![1; matrix[0].len()]; matrix.len()];
    // traverse by rows
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let mut score = 0;
            for k in (0..j).rev() {
                score += 1;
                if matrix[i][j] <= matrix[i][k] {
                    break;
                }
            }
            scores[i][j] *= score;
        }

        for j in (0..matrix[i].len()).rev() {
            let mut score = 0;
            for k in j + 1..matrix[i].len() {
                score += 1;
                if matrix[i][j] <= matrix[i][k] {
                    break;
                }
            }
            scores[i][j] *= score;
        }
    }
    // traverse by columns
    for j in 0..matrix[0].len() {
        for i in 0..matrix.len() {
            let mut score = 0;
            for k in (0..i).rev() {
                score += 1;
                if matrix[i][j] <= matrix[k][j] {
                    break;
                }
            }
            scores[i][j] *= score;
        }

        for i in (0..matrix.len()).rev() {
            let mut score = 0;
            for k in i + 1..matrix.len() {
                score += 1;
                if matrix[i][j] <= matrix[k][j] {
                    break;
                }
            }
            scores[i][j] *= score;
        }
    }
    return scores;
}

fn part_two(input: String) {
    let matrix = parse_matrix(input);
    let total_scores = scenic_score(matrix);
    println!(
        "Max scenic score is {}",
        total_scores.iter().flatten().max().unwrap()
    );
}
