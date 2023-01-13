use itertools::Itertools;
use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    part1(parse_input());
    part2(parse_input());
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
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

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new() -> Self {
        Coord { x: 0, y: 0 }
    }
}

fn parse_input() -> String {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    contents
}
fn part1(input: String) -> usize {
    let mut head = Coord::new();
    let mut tail = Coord::new();
    let mut seen = HashSet::new();
    seen.insert(tail);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let dir = Direction::parse_direction(dir).unwrap();
        let amount: i32 = amount.parse().unwrap();

        for _ in 0..amount {
            match dir {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
            }
            // if head is touching tail
            let diff = Coord {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };
            let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;
            // tail moves diagonally if not touching
            if not_touching {
                tail.x += diff.x.signum();
                tail.y += diff.y.signum();
                seen.insert(tail);
            }
        }
    }
    println!("Part 1 - Seen size: {}", seen.len());
    seen.len()
}

fn part2(input: String) -> usize {
    let start = Coord::new();
    let mut rope = vec![start; 10];
    let mut seen = HashSet::new();
    seen.insert(start);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount: i32 = amount.parse().unwrap();

        for _ in 0..amount {
            match dir {
                "U" => rope[0].y += 1,
                "D" => rope[0].y -= 1,
                "L" => rope[0].x -= 1,
                "R" => rope[0].x += 1,
                _ => println!("Invalid move direction called"),
            }
            // loop for rest of the rope
            for (h, t) in (0..rope.len()).tuple_windows() {
                let diff = Coord {
                    x: rope[h].x - rope[t].x,
                    y: rope[h].y - rope[t].y,
                };
                let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;
                // tail moves diagonally if not touching
                if not_touching {
                    rope[t].x += diff.x.signum();
                    rope[t].y += diff.y.signum();
                    // only add last knot
                    if t == rope.len() - 1 {
                        seen.insert(rope[rope.len() - 1]);
                    }
                }
            }
        }
    }
    println!("Part 2 - Seen size: {}", seen.len());
    seen.len()
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

    const EXAMPLE2: &str = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_test() {
        assert_eq!(13, part1(EXAMPLE.to_owned()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(36, part2(EXAMPLE2.to_owned()));
    }
}
