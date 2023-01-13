use std::{fs::File, io::Read};

fn main() {
    part1(parse_input());
    part2(parse_input());
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Visibility {
    Visible,
    Invisible,
}

fn parse_input() -> Vec<Vec<u32>> {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    contents
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn get_visible_array(input: Vec<Vec<u32>>) -> Vec<Vec<Visibility>> {
    let height = input.len();
    let width = input.get(0).unwrap().len();
    // initialize visible array
    let mut visible_grid: Vec<Vec<Visibility>> = Vec::new();
    for _ in 0..height {
        visible_grid.push(vec![Visibility::Invisible; width]);
    }
    let mut max = 0;
    for y in 0..height {
        max = input[y][0];
        visible_grid[y][0] = Visibility::Visible;
        for x in 1..width {
            if input[y][x] > max {
                max = input[y][x];
                visible_grid[y][x] = Visibility::Visible;
            }
        }
    }
    for y in 0..height {
        max = input[y][width - 1];
        visible_grid[y][width - 1] = Visibility::Visible;
        for x in (0..width).rev() {
            if input[y][x] > max {
                max = input[y][x];
                visible_grid[y][x] = Visibility::Visible;
            }
        }
    }
    // now lets compare vertically
    for x in 0..width {
        max = input[0][x];
        visible_grid[0][x] = Visibility::Visible;
        for y in 0..height {
            if input[y][x] > max {
                max = input[y][x];
                visible_grid[y][x] = Visibility::Visible;
            }
        }
    }
    for x in 0..width {
        max = input[height - 1][x];
        visible_grid[height - 1][x] = Visibility::Visible;
        for y in (0..height).rev() {
            if input[y][x] > max {
                max = input[y][x];
                visible_grid[y][x] = Visibility::Visible;
            }
        }
    }
    visible_grid
}

fn calculate_scenic(input: Vec<Vec<u32>>) -> u32 {
    let height = input.len();
    let width = input.get(0).unwrap().len();
    let mut score: u32 = 0;
    let mut up: u32 = 0;
    let mut down: u32 = 0;
    let mut right: u32 = 0;
    let mut left: u32 = 0;

    for y in 1..height {
        for x in 1..width {
            let current_height = input[y][x];
            // compare up
            up = 0;
            for n in (0..y).rev() {
                up += 1;
                if input[n][x] >= current_height {
                    break;
                }
            }
            // compare down
            down = 0;
            for n in (y + 1)..height {
                down += 1;
                if input[n][x] >= current_height {
                    break;
                }
            }
            // compare right
            right = 0;
            for n in (x + 1)..width {
                right += 1;
                if input[y][n] >= current_height {
                    break;
                }
            }
            // compare left
            left = 0;
            for n in (0..x).rev() {
                left += 1;
                if input[y][n] >= current_height {
                    break;
                }
            }

            // set score
            if (up * down * right * left) > score {
                score = up * down * right * left;
            }
        }
    }
    score
}

fn part1(input: Vec<Vec<u32>>) {
    let mut visible_count = 0;
    let height = input.len();
    let width = input.get(0).unwrap().len();
    let visible_grid = get_visible_array(input);
    for y in 0..height {
        for x in 0..width {
            if visible_grid[y][x] == Visibility::Visible {
                visible_count += 1;
            }
        }
    }
    println!("Visible Tree count = {visible_count}");
}

fn part2(input: Vec<Vec<u32>>) {
    println!("Scenic Score: {}", calculate_scenic(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"30373
25512
65332
33549
35390";

    #[test]
    fn part1_test() {
        //assert_eq!(2, part1(EXAMPLE.to_owned()));
    }

    #[test]
    fn part2_test() {
        // assert_eq!(0, part2(parse_input()));
    }
}
