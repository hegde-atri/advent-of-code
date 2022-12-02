use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

/// Solution to Part 1
fn part1() {
    let mut score = 0;
    if let Ok(input) = read_lines("input.txt") {
        for lines in input {
            if let Ok(line) = lines {
                let (left, right) = line.split_once(' ').unwrap();
                let opponent_hand = Choice::parse_opponent(left).unwrap();
                let my_hand = Choice::parse_me_part1(right).unwrap();
                score += get_score(&my_hand, &opponent_hand);
            }
        }
    }
    println!("Part 1: {}", score);
}

/// Solution to Part 2
fn part2() {
    let mut score = 0;
    if let Ok(input) = read_lines("input.txt") {
        for lines in input {
            if let Ok(line) = lines {
                let (left, right) = line.split_once(' ').unwrap();
                let opponent_hand = Choice::parse_opponent(left).unwrap();
                let my_hand = Choice::parse_me_part2(right, &opponent_hand).unwrap();
                score += get_score(&my_hand, &opponent_hand);
            }
        }
    }
    println!("Part 2: {}", score);
}

/// What hand did we play (Rock, Paper, Scissors)
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    /// Convert oppenent character to a Choice
    fn parse_opponent(input: &str) -> Result<Self, String> {
        match input {
            "A" => Result::Ok(Choice::Rock),
            "B" => Result::Ok(Choice::Paper),
            "C" => Result::Ok(Choice::Scissors),
            _ => Result::Err(String::from("Could not parse opponent input")),
        }
    }

    /// Convert our character to a Choice for part1
    fn parse_me_part1(input: &str) -> Result<Self, String> {
        match input {
            "X" => Result::Ok(Choice::Rock),
            "Y" => Result::Ok(Choice::Paper),
            "Z" => Result::Ok(Choice::Scissors),
            _ => Result::Err(String::from("Could not parse opponent input")),
        }
    }

    /// Convert our character to a Choice for part2
    fn parse_me_part2(input: &str, opponent: &Choice) -> Result<Self, String> {
        // Rock A, X
        // Paper B, Y
        // Scissors C,Z
        // Z - Win
        // Y - Draw
        // X - Lose
        match opponent {
            Choice::Rock => match input {
                "X" => Result::Ok(Choice::Scissors),
                "Y" => Result::Ok(Choice::Rock),
                "Z" => Result::Ok(Choice::Paper),
                _ => Result::Err(String::from("Couldn\'t parse player choice (part 2)")),
            },
            Choice::Paper => match input {
                "X" => Result::Ok(Choice::Rock),
                "Y" => Result::Ok(Choice::Paper),
                "Z" => Result::Ok(Choice::Scissors),
                _ => Result::Err(String::from("Couldn\'t parse player choice (part 2)")),
            },
            Choice::Scissors => match input {
                "X" => Result::Ok(Choice::Paper),
                "Y" => Result::Ok(Choice::Scissors),
                "Z" => Result::Ok(Choice::Rock),
                _ => Result::Err(String::from("Couldn\'t parse player choice (part 2)")),
            },
        }
    }
}

/// Convert the hands to a score
fn get_score(my_choice: &Choice, oppenent_choice: &Choice) -> i32 {
    let choice_score = match my_choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };
    let result_score = match my_choice {
        Choice::Rock => match oppenent_choice {
            Choice::Rock => 3,
            Choice::Paper => 0,
            Choice::Scissors => 6,
        },
        Choice::Paper => match oppenent_choice {
            Choice::Rock => 6,
            Choice::Paper => 3,
            Choice::Scissors => 0,
        },
        Choice::Scissors => match oppenent_choice {
            Choice::Rock => 0,
            Choice::Paper => 6,
            Choice::Scissors => 3,
        },
    };
    return choice_score + result_score;
}

/// Read lines from file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
