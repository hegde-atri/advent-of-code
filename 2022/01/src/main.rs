use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut highest_calories = 0;
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    // If new line, check if Highest and reset sum
                    if sum > highest_calories {
                        highest_calories = sum
                    }
                    sum = 0;
                } else {
                    sum += ip.parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("Highest calories: {}", highest_calories);
}

fn part2() {
    let mut top1: u32 = 0;
    let mut top2: u32 = 0;
    let mut top3: u32 = 0;
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                if ip == "" {
                    // If new line, check if Highest and reset sum
                    if sum > top1 {
                        top3 = top2;
                        top2 = top1;
                        top1 = sum;
                    } else if sum > top2 {
                        top3 = top2;
                        top2 = sum;
                    } else if sum > top3 {
                        top3 = sum;
                    }
                    sum = 0;
                } else {
                    sum += ip.parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("Sum of Top 3 calories: {}", top1 + top2 + top3);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
