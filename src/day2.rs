use crate::helpers;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day2_test.txt");

        assert_eq!(run_part1(&lines), 100_i128);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day2_test.txt");

        assert_eq!(run_part2(&lines), 70_i128);
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn is_serial(input: &Vec<i32>) -> bool {
    let going_up = input[0] < input[1];

    for i in 1..input.len() {
        if going_up {
            if input[i] < input[i - 1] {
                return false;
            }
        } else {
            if input[i] > input[i - 1] {
                return false;
            }
        }
    }
    true
}

fn within_stepsize_limit(input: &Vec<i32>, min: i32, max: i32) -> bool {
    for i in 1..input.len() {
        let diff = (input[i] - input[i - 1]).abs();
        if diff > max || diff < min {
            return false;
        }
    }
    true
}

pub fn run_part1(input: &Vec<String>) -> i128 {
    input
        .iter()
        .map(|l| parse_line(l))
        .filter(is_serial)
        .filter(|x| within_stepsize_limit(x, 1, 3))
        .count() as i128
}

fn drop_one_options(input: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut options = vec![];

    for i in 0..input.len() {
        let mut new_option = input.clone();
        new_option.remove(i);
        options.push(new_option);
    }
    options
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    input
        .iter()
        .map(|l| parse_line(l))
        .map(|x| drop_one_options(&x))
        .filter(|x| {
            x.iter()
                .any(|x| is_serial(x) && within_stepsize_limit(x, 1, 3))
        })
        .count() as i128
}

fn is_safe(x: &Vec<i32>) -> bool {
    is_serial(x) && within_stepsize_limit(x, 1, 3)
}

pub fn run_part2_alt(input: &Vec<String>) -> i128 {
    let parsed = input.iter().map(|l| parse_line(l));

    let mut count = 0;

    for p in parsed {
        if is_safe(&p) {
            count += 1;
            continue;
        }

        for i in 0..p.len() {
            let mut p2 = p.clone();
            p2.remove(i);
            if is_safe(&p2) {
                count += 1;
                break;
            }
        }
    }

    count
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 2");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day2.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    let p2 = run_part2_alt(&lines);
    println!("Part2 alt: {:?}", p2);

    Ok(())
}
