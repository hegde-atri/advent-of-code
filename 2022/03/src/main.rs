use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() -> i32 {
    let mut priorities_sum = 0;
    if let Ok(input) = read_lines("input.txt") {
        for lines in input {
            if let Ok(line) = lines {
                let length = line.len();
                let first_half = &line[..length / 2];
                let second_half = &line[length / 2..length];
                let mut common: char = ' ';
                for c in first_half.chars() {
                    for cc in second_half.chars() {
                        if c == cc {
                            common = c;
                        }
                    }
                }
                if common.is_lowercase() {
                    priorities_sum += common as i32 - 96;
                } else {
                    priorities_sum += common as i32 - 38;
                }
            }
        }
    }
    println!("Part 1 priorities sum: {}", priorities_sum);
    priorities_sum
}

fn part2() -> i32 {
    let mut priorities_sum = 0;
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    let mut queue: VecDeque<String> = VecDeque::new();
    for (i, line) in contents.lines().enumerate() {
        queue.push_front(String::from(line));
        if (i + 1) % 3 == 0 {
            let r1 = queue.pop_back().unwrap();
            let r2 = queue.pop_back().unwrap();
            let r3 = queue.pop_back().unwrap();

            for i in r3.chars() {
                if r2.contains(i) && r1.contains(i) {
                    if i.is_lowercase() {
                        priorities_sum += i as i32 - 96;
                    } else {
                        priorities_sum += i as i32 - 38;
                    }
                    break;
                }
            }
        }
    }

    println!("Part 2 badges sum: {}", priorities_sum);

    priorities_sum
}

/// Read lines from file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!(part1(), 8085);
    }

    #[test]
    fn part2_test() {
        assert_eq!(part2(), 2515);
    }
}
