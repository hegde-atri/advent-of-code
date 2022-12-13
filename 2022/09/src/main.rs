use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    part1(parse_input());
    part2(parse_input());
}

/// Directions Up, Down, Left, Right
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Takes in a String value and returns its Direction wrapped in a result.
    fn parse_direction(input: &str) -> Result<Self, String> {
        match input {
            "U" => Result::Ok(Direction::Up),
            "D" => Result::Ok(Direction::Down),
            "L" => Result::Ok(Direction::Left),
            "R" => Result::Ok(Direction::Right),
            _ => Result::Err(String::from("Error parsing direction")),
        }
    }
}

fn parse_input() -> String {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    contents
}

fn do_instruction<'a>(
    instruction: &str,
    head: &mut [i32; 2],
    tail: &mut [i32; 2],
    visited: &'a mut HashSet<[i32; 2]>,
) {
    let (d, a) = instruction.split_once(' ').unwrap();
    let amount = a.parse::<u8>().unwrap();
    let direction = Direction::parse_direction(d).unwrap();
    for _ in 0..amount {
        // now sort through direction
        match direction {
            Direction::Up => {
                head[1] += 1;
            }
            Direction::Down => {
                head[1] += -1;
            }
            Direction::Left => {
                head[0] += -1;
            }
            Direction::Right => {
                head[0] += 1;
            }
        }
        // make tail follow the head.
        // if tail is along x/y axis.
        // if tail is diagonal.

        // update visited
        println!("working? {:?}", tail);
        visited.insert(*tail);
    }
}

fn part1(input: String) -> u32 {
    // lets initialize a visited hashset to hold our visited locations
    let mut visited: HashSet<[i32; 2]> = HashSet::new();
    // lets intialize position of head and tail
    let mut head: [i32; 2] = [0, 0];
    let mut tail: [i32; 2] = [0, 0];
    for line in input.lines() {
        // do instruction
        let (d, a) = line.split_once(' ').unwrap();
        let amount = a.parse::<u8>().unwrap();
        let direction = Direction::parse_direction(d).unwrap();

        for _ in 0..amount {
            // now sort through direction
            match direction {
                Direction::Up => {
                    head[1] += 1;
                }
                Direction::Down => {
                    head[1] += -1;
                }
                Direction::Left => {
                    head[0] += -1;
                }
                Direction::Right => {
                    head[0] += 1;
                }
            }
            let x_diff = head[0] - tail[0];
            let y_diff = head[1] - tail[1];
            // make tail follow the head.
            // if tail is along x/y axis.
            if x_diff.is_positive() && y_diff == 0 {
                tail[0] += 1;
            } else if head[0] < tail[0] && head[1] == tail[1] {
                tail[0] -= 1;
            } else if head[0] == tail[0] && head[1] > tail[1] {
                tail[1] += 1;
            } else if head[0] == tail[0] && head[1] < tail[1] {
                tail[1] -= 1;
            } else if x_diff.abs() == y_diff.abs() {
            }

            // update visited
            visited.insert(tail);
        }
    }
    // calulate visited cells
    println!("{}", visited.len() as u32);
    visited.len() as u32
}

fn part2(input: String) -> u8 {
    for line in input.lines() {}
    let a: u8 = 0;
    a
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_test() {
        assert_eq!(2, part1(EXAMPLE.to_owned()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(0, part2(EXAMPLE.to_owned()));
    }
}
