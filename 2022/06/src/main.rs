use std::{collections::HashSet, fs::File, io::Read};

fn main() {
    println!("{}", part1(parse_input()));
    println!("{}", part2(parse_input()));
}

fn parse_input() -> String {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    contents
}

fn part1(input: String) -> usize {
    return input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .map(|w| -> HashSet<char> { HashSet::from_iter(w.to_vec()) })
        .position(|w| w.len() == 4)
        .unwrap()
        + 4;
}

fn part2(input: String) -> usize {
    return input
        .chars()
        .collect::<Vec<char>>()
        .windows(14)
        .map(|w| -> HashSet<char> { HashSet::from_iter(w.to_vec()) })
        .position(|w| w.len() == 14)
        .unwrap()
        + 14;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn part1_test() {
        assert_eq!(7, part1(EXAMPLE.to_owned()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(19, part2(EXAMPLE.to_owned()));
    }
}
