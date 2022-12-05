use std::{fs::File, io::Read};

fn main() {
    println!("{}", part1(parse_input(String::from("input.txt"))));
    println!("{}", part2(parse_input(String::from("input.txt"))));
}

fn parse_input(file: String) -> String {
    let mut file = File::open(file).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Reading from file failed!");
    contents
}

fn part1(input: String) -> String {
    // First lets split the crates and instructions
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    // initialize stack
    let mut stacks = parse_crates(crates);
    instructions
        .lines()
        .map(|l| Instruction::parse_line(l))
        .for_each(|i| {
            (0..i.amount).for_each(|_| {
                let temp = stacks[i.from - 1].pop().expect("Stack is empty");
                stacks[i.to - 1].push(temp);
            })
        });
    // now get the top element of each stack
    stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

fn part2(input: String) -> String {
    // First lets split the crates and instructions
    let (crates, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_crates(crates);
    instructions
        .lines()
        .map(|l| Instruction::parse_line(l))
        .for_each(|i| {
            let block = stacks[i.from - 1].len() - i.amount..;
            let crates: Vec<char> = stacks[i.from - 1].drain(block).collect();
            stacks[i.to - 1].extend(crates);
        });
    // now get the top element of each stack
    stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

/// This will take in the crates section and return a stack of each column
fn parse_crates(crates: &str) -> Vec<Vec<char>> {
    // gets last line using lines().rev() and adds 1 to the length + 1
    // and divides by 4 to get rid of empty spaces
    let width = (crates.lines().rev().next().unwrap().len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(width);
    let height = crates.lines().count() - 1;
    // initialise stack in stacks
    for _ in 0..width {
        stacks.push(Vec::with_capacity(height));
    }
    // iterate through each line in crates
    crates.lines().rev().skip(1).for_each(|l| {
        let mut crates = l.chars().skip(1).step_by(4);
        // now iterate through stacks
        stacks.iter_mut().for_each(|e| {
            // if there is a next element in crates
            if let Some(c) = crates.next() {
                // element is alphabet
                if c.is_alphabetic() {
                    // push into the stack
                    e.push(c)
                }
            }
        })
    });
    stacks
}

struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    /// Will parse line in format "move <num> from <num> to <num>"
    /// into an Instruction struct
    fn parse_line(line: &str) -> Self {
        let mut parts = line.split(" ");
        Self {
            amount: parts.nth(1).unwrap().parse::<usize>().unwrap(),
            from: parts.nth(1).unwrap().parse::<usize>().unwrap(),
            to: parts.nth(1).unwrap().parse::<usize>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        assert_eq!("CMZ", part1(parse_input(String::from("example.txt"))));
    }

    #[test]
    fn part2_test() {
        assert_eq!("MCD", part2(parse_input(String::from("example.txt"))));
    }
}
