use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day7_test.txt");

        assert_eq!(run_part1(&lines), 3749);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day7_test.txt");

        assert_eq!(run_part2(&lines), 11387);
    }
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
    Concat,
}

impl Operator {
    fn execute(self, a: usize, b: usize) -> usize {
        match self {
            Operator::Add => a + b,
            Operator::Mul => a * b,
            Operator::Concat => format!("{a}{b}").parse::<usize>().unwrap(),
        }
    }
}

fn calc_all(expected: usize, nums: Vec<usize>) -> Option<usize> {
    if nums.len() == 1 {
        return if nums[0] == expected {
            Some(expected)
        } else {
            None
        };
    }

    // Generate all possible combinations of operations
    let ops_count = nums.len() - 1;
    let max_combinations = 1 << ops_count; // 2^(n-1) combinations

    // Try each combination
    for i in 0..max_combinations {
        let mut result = nums[0];

        // For each position, check if we should add (0) or multiply (1)
        for j in 0..ops_count {
            let op = if (i >> j) & 1 == 0 {
                Operator::Add
            } else {
                Operator::Mul
            };
            result = op.execute(result, nums[j + 1]);
        }

        if result == expected {
            return Some(expected);
        }
    }
    None
}

pub fn run_part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| l.split(": "))
        .map(|mut l| {
            (
                l.next().unwrap().parse::<usize>().unwrap(),
                l.next()
                    .unwrap()
                    .split(" ")
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(a, b)| calc_all(a, b))
        .filter_map(|x| x)
        .sum()
}

fn calc_all2(expected: usize, nums: Vec<usize>) -> Option<usize> {
    if nums.len() == 1 {
        return if nums[0] == expected {
            Some(expected)
        } else {
            None
        };
    }

    let ops_count = nums.len() - 1;
    let max_combinations = 3_usize.pow(ops_count as u32); // 3^(n-1) combinations

    // Try each combination
    for i in 0..max_combinations {
        let mut result = nums[0];
        let mut temp = i;

        // For each position, check which operator to use
        for j in 0..ops_count {
            let op = match temp % 3 {
                0 => Operator::Add,
                1 => Operator::Mul,
                2 => Operator::Concat,
                _ => unreachable!(),
            };
            result = op.execute(result, nums[j + 1]);
            temp /= 3;
        }

        if result == expected {
            return Some(expected);
        }
    }
    None
}

pub fn run_part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|l| l.split(": "))
        .map(|mut l| {
            (
                l.next().unwrap().parse::<usize>().unwrap(),
                l.next()
                    .unwrap()
                    .split(" ")
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            )
        })
        .map(|(a, b)| calc_all2(a, b))
        .filter_map(|x| x)
        .sum()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 7");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day7.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
