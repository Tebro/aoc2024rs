use crate::helpers;
use regex::Regex;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");

        assert_eq!(run_part1(&lines), 161);
    }

    #[test]
    fn test_part1_alt() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");

        assert_eq!(run_part1_alt(&lines), 161);
    }

    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");

        assert_eq!(run_part2(&lines), 48);
    }
    #[test]
    fn test_part2_alt() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");

        assert_eq!(run_part2_alt(&lines), 48);
    }
}

fn parse_mul_instruction(s: &str) -> (usize, usize) {
    let re_nums = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let caps = re_nums.captures(s).unwrap();
    let a = caps[1].parse().unwrap();
    let b = caps[2].parse().unwrap();
    (a, b)
}

pub fn run_part1(input: &[String]) -> usize {
    let re_instruction = Regex::new(r"(mul\(\d\d?\d?,\d\d?\d?\))").unwrap();

    let matches: Vec<Vec<String>> = input
        .iter()
        .map(|l| {
            re_instruction
                .captures_iter(l)
                .map(|cap| cap[0].to_string())
                .collect()
        })
        .collect();

    matches
        .iter()
        .flat_map(|l| l.iter().map(|i| parse_mul_instruction(i)))
        .map(|(a, b)| a * b)
        .sum()
}

pub fn run_part1_alt(input: &[String]) -> usize {
    let re_nums = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let all_lines = input.join("");
    re_nums
        .captures_iter(&all_lines)
        .map(|caps| caps[1].parse::<usize>().unwrap() * caps[2].parse::<usize>().unwrap())
        .sum()
}

fn process_line(line: &str, doing: &mut bool) -> usize {
    let re = Regex::new(r"^mul\((\d\d?\d?),(\d\d?\d?)\).*").unwrap();
    let re_do = Regex::new(r"^do\(\).*").unwrap();
    let re_dont = Regex::new(r"^don't\(\).*").unwrap();
    let mut result = 0;
    let mut it = line.char_indices();
    while let Some((i, _)) = it.next() {
        let rest = &line[i..];
        if re_do.is_match(rest) {
            *doing = true;
            it.nth(2);
        } else if re_dont.is_match(rest) {
            *doing = false;
            it.nth(5);
        } else if *doing && re.is_match(rest) {
            let caps = re.captures(rest).unwrap();
            let a = caps[1].parse::<usize>().unwrap();
            let b = caps[2].parse::<usize>().unwrap();
            result += a * b;
            it.nth(4 + caps[1].len() + caps[2].len());
        }
    }
    result
}

pub fn run_part2(input: &[String]) -> usize {
    let mut doing = true;
    let mut sum = 0;
    for l in input {
        let result = process_line(l, &mut doing);
        sum += result;
    }
    sum
}

pub fn run_part2_alt(input: &[String]) -> usize {
    let re = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let lines = input.join("");

    let mut parts = lines.split("don't()");
    let mut relevant = vec![];
    relevant.push(parts.next().unwrap());
    for part in parts {
        let sub = part.split("do()").skip(1);
        for s in sub {
            relevant.push(s);
        }
    }

    let all_relevant = relevant.join("");
    re.captures_iter(&all_relevant)
        .map(|caps| caps[1].parse::<usize>().unwrap() * caps[2].parse::<usize>().unwrap())
        .sum()
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 3");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day3.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    let p1 = run_part1_alt(&lines);
    println!("Part1 alt: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    let p1 = run_part2_alt(&lines);
    println!("Part2 alt: {:?}", p1);

    Ok(())
}
