use std::{fs::File, io::Read};

fn main() {
    part1();
    part2();
}

fn part1() -> u32 {
    let mut ass_pairs = 0;
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    for line in contents.lines() {
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

fn part2() -> u32 {
    let mut ass_overlap = 0;
    let mut file = File::open("input.txt").expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    for line in contents.lines() {
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

    #[test]
    fn part1_test() {
        assert_eq!(534, part1());
    }

    #[test]
    fn part2_test() {
        assert_eq!(841, part2());
    }
}
