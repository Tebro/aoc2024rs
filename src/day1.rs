use crate::helpers;
use std::{collections::HashMap, io};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day1_test.txt");

        assert_eq!(run_part1(&lines), 100);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day1_test.txt");

        assert_eq!(run_part2(&lines), 70);
    }
}

fn parse_input(input: &[String]) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.iter() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse::<usize>().unwrap());
        right.push(parts.next().unwrap().parse::<usize>().unwrap());
    }
    left.sort();
    right.sort();

    (left, right)
}

pub fn run_part1(input: &[String]) -> usize {
    let (left, right) = parse_input(input);

    let pairs = left.iter().zip(right.iter());
    pairs.map(|(l, r)| l.abs_diff(*r)).sum()
}

pub fn run_part2(input: &[String]) -> usize {
    let (left, right) = parse_input(input);

    left.iter()
        .filter(|l| right.contains(l))
        .map(|l| (right.iter().filter(|r| *r == l).count()) * l)
        .sum()
}

pub fn run_part2_alt(input: &[String]) -> usize {
    let mut left = HashMap::new();
    let mut right = vec![];

    for line in input.iter() {
        let mut parts = line.split_whitespace();
        left.insert(parts.next().unwrap().parse::<usize>().unwrap(), 0_usize);
        right.push(parts.next().unwrap().parse::<usize>().unwrap());
    }
    right.sort();

    for r in right.iter() {
        left.entry(*r).and_modify(|x| *x += 1);
    }

    left.iter().map(|(k, v)| k * v).sum()
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

    let p2_alt = run_part2_alt(&lines);
    println!("Part2_alt: {:?}", p2_alt);

    Ok(())
}
