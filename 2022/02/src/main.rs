use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
}

fn part1() {
    let mut score = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // score based on your input
                let opponent: char = ip.chars().nth(0).unwrap();
                let mine: char = ip.chars().nth(2).unwrap();
                // if ip.chars().nth(2).unwrap() == 'X' {
                //     score += 1;
                // } else if ip.chars().nth(2).unwrap() == 'Y' {
                //     score += 2;
                // } else {
                //     score += 3;
                // }
                // score based on outcome of game
                println!("{}, {}", opponent, mine);
                // if opponent == 'A' && mine == 'X' {
                //     score += 3;
                // } else if opponent == 'B' && mine == 'Y' {
                //     score += 3;
                // } else if opponent == 'C' && mine == 'Z' {
                //     score += 3;
                // }
                score += match ip.as_str() {
                    "A X" => 3,
                    "A Y" => 4,
                    "A Z" => 8,
                    "B X" => 1,
                    "B Y" => 5,
                    "B Z" => 9,
                    "C X" => 2,
                    "C Y" => 6,
                    "C Z" => 7,
                    _ => 0,
                };
            }
        }
        println!("{}", score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
