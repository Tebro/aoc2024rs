use crate::helpers;
use std::cell::RefCell;
use std::rc::Rc;
use std::{collections::HashMap, io};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day4_test.txt");

        assert_eq!(run_part1(&lines), 18);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day4_test.txt");

        assert_eq!(run_part2(&lines), 9);
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    Any,
}

struct Link {
    direction: Direction,
    node: Rc<RefCell<GraphNode>>,
}

struct GraphNode {
    value: char,
    links: Vec<Link>,
}

type Graph = HashMap<(usize, usize), Rc<RefCell<GraphNode>>>;

fn build_graph(input: &[Vec<char>]) -> Graph {
    let mut coords = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            let node = GraphNode {
                value: input[i][j],
                links: vec![],
            };
            coords.insert((i, j), Rc::new(RefCell::new(node)));
        }
    }

    let positions: Vec<(usize, usize)> = coords.keys().cloned().collect();

    // Set up links
    for &(i, j) in &positions {
        let mut links = Vec::new();
        if i > 0 {
            links.push(Link {
                direction: Direction::Up,
                node: Rc::clone(&coords[&(i - 1, j)]),
            });
        }
        if j > 0 {
            links.push(Link {
                direction: Direction::Left,
                node: Rc::clone(&coords[&(i, j - 1)]),
            });
        }
        if i < input.len() - 1 {
            links.push(Link {
                direction: Direction::Down,
                node: Rc::clone(&coords[&(i + 1, j)]),
            });
        }
        if j < input[0].len() - 1 {
            links.push(Link {
                direction: Direction::Right,
                node: Rc::clone(&coords[&(i, j + 1)]),
            });
        }
        if i > 0 && j > 0 {
            links.push(Link {
                direction: Direction::UpLeft,
                node: Rc::clone(&coords[&(i - 1, j - 1)]),
            });
        }
        if i > 0 && j < input[0].len() - 1 {
            links.push(Link {
                direction: Direction::UpRight,
                node: Rc::clone(&coords[&(i - 1, j + 1)]),
            });
        }
        if i < input.len() - 1 && j > 0 {
            links.push(Link {
                direction: Direction::DownLeft,
                node: Rc::clone(&coords[&(i + 1, j - 1)]),
            });
        }
        if i < input.len() - 1 && j < input[0].len() - 1 {
            links.push(Link {
                direction: Direction::DownRight,
                node: Rc::clone(&coords[&(i + 1, j + 1)]),
            });
        }

        coords[&(i, j)].borrow_mut().links = links;
    }
    coords
}

fn count_all_from(
    node: &Rc<RefCell<GraphNode>>,
    current_char: char,
    direction: Direction,
) -> usize {
    let xmas = ['X', 'M', 'A', 'S'];
    let current_char_index = xmas.iter().position(|&c| c == current_char).unwrap();
    if current_char_index == xmas.len() - 1 {
        return 1;
    }

    match direction {
        Direction::Any => {
            let mut count = 0;
            for link in node.borrow().links.iter() {
                if link.node.borrow().value == xmas[current_char_index + 1] {
                    count +=
                        count_all_from(&link.node, xmas[current_char_index + 1], link.direction);
                }
            }
            count
        }
        dir => {
            let mut count = 0;
            for link in node.borrow().links.iter() {
                if link.direction == dir && link.node.borrow().value == xmas[current_char_index + 1]
                {
                    count += count_all_from(&link.node, xmas[current_char_index + 1], dir);
                }
            }
            count
        }
    }
}

fn count_xmas(data: &Graph) -> usize {
    let all_starting_positions = data.iter().filter(|(_k, v)| v.borrow().value == 'X');

    all_starting_positions
        .map(|(_k, v)| count_all_from(v, 'X', Direction::Any))
        .sum()
}

pub fn run_part1(input: &[String]) -> usize {
    let binding = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let coords = build_graph(&binding);
    count_xmas(&coords)
}

fn build_char_map(input: &[Vec<char>]) -> HashMap<(isize, isize), char> {
    let mut map = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            map.insert((i as isize, j as isize), input[i][j]);
        }
    }
    map
}

pub fn get_neighbour_coords((i, j): (isize, isize), direction: Direction) -> (isize, isize) {
    match direction {
        Direction::Up => (i - 1, j),
        Direction::Down => (i + 1, j),
        Direction::Left => (i, j - 1),
        Direction::Right => (i, j + 1),
        Direction::UpLeft => (i - 1, j - 1),
        Direction::UpRight => (i - 1, j + 1),
        Direction::DownLeft => (i + 1, j - 1),
        Direction::DownRight => (i + 1, j + 1),
        Direction::Any => (i, j),
    }
}

pub fn run_part1_alt(input: &[String]) -> usize {
    let binding = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let map = build_char_map(&binding);

    let x_positions = map.iter().filter(|(_k, v)| **v == 'X');

    let mut count = 0;

    for (x, _v) in x_positions {
        'dir_loop: for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ] {
            let mut temp = *x;
            for tc in &['M', 'A', 'S'] {
                temp = get_neighbour_coords(temp, *direction);
                match map.get(&temp) {
                    Some(c) => {
                        if c != tc {
                            continue 'dir_loop;
                        };
                    }
                    None => {
                        continue 'dir_loop;
                    }
                }
            }
            count += 1;
        }
    }

    count
}

pub fn run_part1_alt2(input: &[String]) -> usize {
    let binding = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;
    for i in 0..binding.len() {
        for j in 0..binding[0].len() {
            if binding[i][j] != 'X' {
                continue;
            }
            let x = (i as isize, j as isize);
            'dir_loop: for direction in &[
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
                Direction::UpLeft,
                Direction::UpRight,
                Direction::DownLeft,
                Direction::DownRight,
            ] {
                let mut temp: (isize, isize) = x;
                for tc in &['M', 'A', 'S'] {
                    temp = get_neighbour_coords(temp, *direction);
                    if temp.0 < 0
                        || temp.1 < 0
                        || temp.0 >= binding.len() as isize
                        || temp.1 >= binding[0].len() as isize
                    {
                        continue 'dir_loop;
                    }
                    let c = binding[temp.0 as usize][temp.1 as usize];
                    if c != *tc {
                        continue 'dir_loop;
                    };
                }
                count += 1;
            }
        }
    }

    count
}

fn is_diag_x_mas(node: &Rc<RefCell<GraphNode>>) -> bool {
    let diag_nodes_left_right = node
        .borrow()
        .links
        .iter()
        .filter(|l| l.direction == Direction::DownRight || l.direction == Direction::UpLeft)
        .map(|l| l.node.borrow().value)
        .collect::<Vec<char>>();
    let diag_nodes_right_left = node
        .borrow()
        .links
        .iter()
        .filter(|l| l.direction == Direction::DownLeft || l.direction == Direction::UpRight)
        .map(|l| l.node.borrow().value)
        .collect::<Vec<char>>();
    if diag_nodes_left_right.len() < 2 {
        return false;
    }
    if diag_nodes_right_left.len() < 2 {
        return false;
    }

    let mut left_right_ok = false;

    if diag_nodes_left_right[0] == 'M' && diag_nodes_left_right[1] == 'S' {
        left_right_ok = true;
    }
    if diag_nodes_left_right[0] == 'S' && diag_nodes_left_right[1] == 'M' {
        left_right_ok = true;
    }

    let mut right_left_ok = false;
    if diag_nodes_right_left[0] == 'M' && diag_nodes_right_left[1] == 'S' {
        right_left_ok = true;
    }
    if diag_nodes_right_left[0] == 'S' && diag_nodes_right_left[1] == 'M' {
        right_left_ok = true;
    }

    left_right_ok && right_left_ok
}

pub fn run_part2(input: &[String]) -> usize {
    let binding = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let graph = build_graph(&binding);
    let all_as = graph.iter().filter(|(_k, v)| v.borrow().value == 'A');
    all_as.filter(|(_k, a)| is_diag_x_mas(a)).count()
}

pub fn run_part2_alt(input: &[String]) -> usize {
    let binding = input
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut count = 0;
    for i in 0..binding.len() {
        for j in 0..binding[0].len() {
            if binding[i][j] != 'A' {
                continue;
            }
            let x = (i as isize, j as isize);

            // Check out of bounds risk
            if i < 1 || j < 1 || i >= binding.len() - 1 || j >= binding[0].len() - 1 {
                continue;
            }

            let left_to_right_ok = {
                let top_left_coords = get_neighbour_coords(x, Direction::UpLeft);
                let top_left = binding[top_left_coords.0 as usize][top_left_coords.1 as usize];
                let bottom_right_coords = get_neighbour_coords(x, Direction::DownRight);
                let bottom_right =
                    binding[bottom_right_coords.0 as usize][bottom_right_coords.1 as usize];
                top_left == 'M' && bottom_right == 'S' || top_left == 'S' && bottom_right == 'M'
            };

            let right_to_left_ok = {
                let top_right_coords = get_neighbour_coords(x, Direction::UpRight);
                let top_right = binding[top_right_coords.0 as usize][top_right_coords.1 as usize];
                let bottom_left_coords = get_neighbour_coords(x, Direction::DownLeft);
                let bottom_left =
                    binding[bottom_left_coords.0 as usize][bottom_left_coords.1 as usize];
                top_right == 'M' && bottom_left == 'S' || top_right == 'S' && bottom_left == 'M'
            };

            if left_to_right_ok && right_to_left_ok {
                count += 1;
            }
        }
    }
    count
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 4");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    let p1 = run_part1_alt(&lines);
    println!("Part1 alt: {:?}", p1);

    let p1 = run_part1_alt2(&lines);
    println!("Part1 alt 2: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    let p2 = run_part2_alt(&lines);
    println!("Part2 alt: {:?}", p2);

    Ok(())
}
