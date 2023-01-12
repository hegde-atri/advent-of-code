use std::{collections::HashMap, fs, path::PathBuf};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Could not find input.txt in cwd");
    println!("part1 : {}", part1(&input));
    println!("part2 : {}", part2(&input));
}

fn part1(input: &String) -> usize {
    let mut table = HashMap::new();
    let mut dirs = Vec::new();

    for line in input.lines().skip(1) {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }
        let split: Vec<_> = line.split_whitespace().collect();
        match split[..] {
            ["$", "cd", ".."] => {
                dirs.pop();
            }
            ["$", "cd", d] => {
                dirs.push(d);
            }
            [size, _name] => {
                let size: i32 = size.parse().unwrap();
                // lets update the size of of each path now.
                for i in 0..dirs.len() {
                    let path = PathBuf::from_iter(&dirs[..=i]);
                    *table.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }
    // return sum of all dirs less than 100_000
    table.values().filter(|v| *v <= &100_000).sum::<i32>() as usize
}

fn part2(input: &String) -> usize {
    let mut table = HashMap::new();
    let mut dirs = Vec::new();

    for line in input.lines() {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }
        let split: Vec<_> = line.split_whitespace().collect();
        match split[..] {
            ["$", "cd", ".."] => {
                dirs.pop();
            }
            ["$", "cd", d] => {
                dirs.push(d);
            }
            [size, _name] => {
                let size: i32 = size.parse().unwrap();
                // lets update the size of of each path now.
                for i in 0..dirs.len() {
                    let path = PathBuf::from_iter(&dirs[..=i]);
                    *table.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        }
    }
    // return sum of all dirs less than 100_000
    let full_size = table.get(&PathBuf::from("/")).unwrap();
    *table
        .values()
        .filter(|v| *v + 70_000_000 - full_size >= 30_000_000)
        .min()
        .unwrap() as usize
}
