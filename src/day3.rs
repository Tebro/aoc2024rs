use crate::helpers;
use regex::Regex;
use std::io;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");

        assert_eq!(run_part1(&lines), 161_i128);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day3_test.txt");

        assert_eq!(run_part2(&lines), 48_i128);
    }
}

fn parse_mul_instruction(s: &str) -> (i32, i32) {
    let re_nums = Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let caps = re_nums.captures(s).unwrap();
    let a = caps[1].parse::<i32>().unwrap();
    let b = caps[2].parse::<i32>().unwrap();
    (a, b)
}

pub fn run_part1(input: &Vec<String>) -> i128 {
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
        .map(|(a, b)| a as i128 * b as i128)
        .sum()
}

fn do_line(line: &str, doing: &mut bool) -> i128 {
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
            let a = caps[1].parse::<i128>().unwrap();
            let b = caps[2].parse::<i128>().unwrap();
            result += a * b;
            it.nth(4 + caps[1].len() + caps[2].len());
        }
    }
    result
}

pub fn run_part2(input: &Vec<String>) -> i128 {
    let mut doing = true;
    let mut sum = 0;
    for l in input {
        let result = do_line(l, &mut doing);
        sum += result;
    }
    sum
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 3");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day3.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
