use crate::helpers;
use std::{
    collections::{HashMap, HashSet},
    io,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day8_test.txt");

        assert_eq!(run_part1(&lines), 14);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day8_test.txt");

        assert_eq!(run_part2(&lines), 70);
    }
}

type Pos = (isize, isize);

type Vector = (isize, isize);

fn new_vector(a: Pos, b: Pos) -> Vector {
    (b.0 - a.0, b.1 - a.1)
}

fn pos_add(pos: Pos, v: Vector) -> Pos {
    (pos.0 + v.0, pos.1 + v.1)
}

fn vec_double(v: &Vector) -> Vector {
    (v.0 * 2, v.1 * 2)
}

pub fn run_part1(input: &[String]) -> usize {
    let mut grouped: HashMap<char, Vec<Pos>> = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let c = input[i].chars().nth(j).unwrap();
            if c == '.' {
                continue;
            }
            if grouped.contains_key(&c) {
                grouped.get_mut(&c).unwrap().push((i as isize, j as isize));
            } else {
                grouped.insert(c, vec![(i as isize, j as isize)]);
            }
        }
    }

    let max_i = input.len() as isize;
    let max_j = input[0].len() as isize;

    let mut unique_anti_nodes = HashSet::new();

    for (_k, v) in grouped.iter() {
        for i in 0..v.len() {
            let current = v[i];
            for j in 0..v.len() {
                if i == j {
                    continue;
                }
                let other = v[j];
                let vec = new_vector(current, other);
                let doubled = vec_double(&vec);
                let anti_node = pos_add(current, doubled);
                if anti_node.0 < max_i
                    && anti_node.0 >= 0
                    && anti_node.1 < max_j
                    && anti_node.1 >= 0
                {
                    unique_anti_nodes.insert(anti_node);
                }
            }
        }
    }

    unique_anti_nodes.len()
}

pub fn run_part2(input: &[String]) -> isize {
    200
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 8");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day8.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
