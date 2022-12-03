use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
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
}

fn part2() {
    let mut priorities_sum = 0;
    if let Ok(input) = read_lines("input.txt") {
        let mut count = 1;
        let mut rugsacks: [String; 3] = [String::new(), String::new(), String::new()];
        for lines in input {
            if let Ok(line) = lines {
                let mut common: char = ' ';
                rugsacks[count % 3] = line;
                if count % 3 == 0 {
                    for c1 in rugsacks[0].chars() {
                        for c2 in rugsacks[1].chars() {
                            for c3 in rugsacks[2].chars() {
                                if c1 == c2 && c2 == c3 {
                                    common = c3;
                                }
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
            count += 1;
        }
    }
    println!("Part 2 badges sum: {}", priorities_sum);
}

/// Read lines from file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
