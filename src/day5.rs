use crate::helpers;
use std::{collections::HashMap, io};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day5_test.txt");

        assert_eq!(run_part1(&lines), 143);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day5_test.txt");

        assert_eq!(run_part2(&lines), 70);
    }
}

pub fn run_part1(input: &[String]) -> usize {
    let mut parts = input.split(|l| l == "");
    let rules_raw = parts.next().unwrap();
    let orders = parts.next().unwrap();

    let mut rules: HashMap<&str, Vec<&str>> = HashMap::new();
    for rule in rules_raw {
        let mut parts = rule.split("|");
        let m = parts.next().unwrap();
        let v = parts.next().unwrap();
        if rules.contains_key(m) {
            rules.get_mut(m).unwrap().push(v);
        } else {
            rules.insert(m, vec![v]);
        }
    }

    let mut middles = vec![];

    'orderLoop: for order in orders {
        let parts = order.split(",").collect::<Vec<&str>>();
        for i in 0..parts.len() {
            let relevant_rules = rules.get(&parts[i]).unwrap();
            for j in 0..i {
                if relevant_rules.contains(&parts[j]) {
                    continue 'orderLoop;
                }
            }
        }
        let middle = parts.len() / 2;
        middles.push(parts[middle]);
    }

    middles.iter().map(|x| x.parse::<usize>().unwrap()).sum()
}

pub fn run_part2(input: &[String]) -> usize {
    let mut parts = input.split(|l| l == "");
    let rules_raw = parts.next().unwrap();
    let orders = parts.next().unwrap();

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    for rule in rules_raw {
        let mut parts = rule.split("|");
        let m = parts.next().unwrap().parse::<usize>().unwrap();
        let v = parts.next().unwrap().parse::<usize>().unwrap();
        if rules.contains_key(&m) {
            rules.get_mut(&m).unwrap().push(v);
        } else {
            rules.insert(m, vec![v]);
        }
    }

    let mut invalid_lines = vec![];
    'orderLoop: for order in orders {
        let parts = order
            .split(",")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        for i in 0..parts.len() {
            let relevant_rules = rules.get(&parts[i]).unwrap();
            for j in 0..i {
                if relevant_rules.contains(&parts[j]) {
                    invalid_lines.push(parts);
                    continue 'orderLoop;
                }
            }
        }
    }

    let mut res = 0;

    for mut order in invalid_lines {
        order.sort_by(|a, b| {
            let relevant_rules_a = rules.get(&a).unwrap();
            if relevant_rules_a.contains(b) {
                return std::cmp::Ordering::Less;
            }

            let relevant_rules_b = rules.get(&b).unwrap();
            if relevant_rules_b.contains(a) {
                return std::cmp::Ordering::Greater;
            }

            std::cmp::Ordering::Equal
        });

        let middle = order.len() / 2;
        res += order[middle];
    }

    res
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 5");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day5.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
