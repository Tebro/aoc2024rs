use crate::{day4::Direction, helpers};
use std::{collections::HashSet, io};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day6_test.txt");

        assert_eq!(run_part1(&lines), 100);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day6_test.txt");

        assert_eq!(run_part2(&lines), 70);
    }
}

fn turn_right(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        _ => Direction::Up,
    }
}

pub fn run_part1(input: &[String]) -> usize {
    use crate::day4::get_neighbour_coords;

    let map = input
        .iter()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut start: (isize, isize) = (0, 0);
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '^' {
                start = (i as isize, j as isize);
            }
        }
    }

    // Start position counts
    let mut visited_positions = HashSet::new();
    visited_positions.insert(start);
    let mut current_position = start;
    let mut current_direction = Direction::Up;
    loop {
        // If edge of map, break
        match current_direction {
            Direction::Up => {
                if current_position.0 == 0 {
                    break;
                }
            }
            Direction::Down => {
                if current_position.0 == input.len() as isize - 1 {
                    break;
                }
            }
            Direction::Left => {
                if current_position.1 == 0 {
                    break;
                }
            }
            Direction::Right => {
                if current_position.1 == input[0].len() as isize - 1 {
                    break;
                }
            }
            _ => {}
        }

        let next_position = get_neighbour_coords(current_position, current_direction);
        visited_positions.insert(current_position);
        if map[next_position.0 as usize][next_position.1 as usize] == '#' {
            current_direction = turn_right(current_direction);
        } else {
            current_position = next_position;
        }
    }

    visited_positions.len()
}

pub fn run_part2(input: &[String]) -> usize {
    200
}

pub fn run() -> io::Result<()> {
    println!("\n\nDay 6");

    // Input goes here
    let lines = helpers::read_file_to_vec::<String>("inputs/day6.txt");

    // Part 1 goes here
    let p1 = run_part1(&lines);
    println!("Part1: {:?}", p1);

    // Part 2 Goes here
    let p2 = run_part2(&lines);
    println!("Part2: {:?}", p2);

    Ok(())
}
