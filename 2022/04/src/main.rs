use std::{fs::File, io::Read};

fn main() {
    part1(parse_input());
    part2(parse_input());
}

fn parse_input() -> String {
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    contents
}

fn part1(input: String) -> u32 {
    let mut ass_pairs = 0;

    for line in input.lines() {
        let (pair1, pair2) = line.split_once(',').unwrap();
        let (start1, end1) = pair1.split_once('-').unwrap();
        let (start2, end2) = pair2.split_once('-').unwrap();
        if start1.parse::<u32>().unwrap() <= start2.parse::<u32>().unwrap()
            && end1.parse::<u32>().unwrap() >= end2.parse::<u32>().unwrap()
        {
            ass_pairs += 1;
        } else if start1.parse::<u32>().unwrap() >= start2.parse::<u32>().unwrap()
            && end1.parse::<u32>().unwrap() <= end2.parse::<u32>().unwrap()
        {
            ass_pairs += 1;
        }
    }
    println!("Assignment pairs: {ass_pairs}");
    ass_pairs
}

fn part2(input: String) -> u32 {
    let mut ass_overlap = 0;
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    for line in input.lines() {
        let (pair1, pair2) = line.split_once(',').unwrap();
        let (start1, end1) = pair1.split_once('-').unwrap();
        let (start2, end2) = pair2.split_once('-').unwrap();
        let s1 = start1.parse::<u32>().unwrap();
        let s2 = start2.parse::<u32>().unwrap();
        let e1 = end1.parse::<u32>().unwrap();
        let e2 = end2.parse::<u32>().unwrap();
        if (s1 <= s2 && e1 >= s2) || (s2 <= s1 && e2 >= s1) {
            ass_overlap += 1;
        }
    }
    println!("Assignment overlap: {ass_overlap}");
    ass_overlap
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1_test() {
        assert_eq!(2, part1(EXAMPLE.to_owned()));
    }

    #[test]
    fn part2_test() {
        assert_eq!(4, part2(EXAMPLE.to_owned()));
    }
}
