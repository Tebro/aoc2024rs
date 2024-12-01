use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day1_test.txt");

        assert_eq!(run_part1(&lines), 100_i128);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day1_test.txt");

        assert_eq!(run_part2(&lines), 70_i128);
    }
}

fn parse_input(input: &Vec<String>) -> (Vec<i128>, Vec<i128>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.iter() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<i128>().unwrap());
        right.push(parts.next().unwrap().parse::<i128>().unwrap());
    }
    left.sort();
    right.sort();

    (left, right)
}

pub fn run_part1(input: &Vec<String>) -> i128 {
    let (left, right) = parse_input(input);

    let pairs = left.iter().zip(right.iter());
    pairs.map(|(l, r)| (l - r).abs()).sum()
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    let (left, right) = parse_input(input);

    left.iter()
        .filter(|l| right.contains(l))
        .map(|l| (right.iter().filter(|r| *r == l).count() as i128) * l)
        .sum()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 1");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day1.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
