use crate::{
    day4::{get_neighbour_coords, Direction},
    helpers,
};
use std::{collections::HashSet, io};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day6_test.txt");

        assert_eq!(run_part1(&lines), 41);
    }
    #[test]
    fn test_part2() {
        let lines = helpers::read_file_to_vec::<String>("inputs/day6_test.txt");

        assert_eq!(run_part2(&lines), 6);
    }
    #[test]
    fn test_turn_right() {
        assert_eq!(turn_right(Direction::Up), Direction::Right);
        assert_eq!(turn_right(Direction::Right), Direction::Down);
        assert_eq!(turn_right(Direction::Down), Direction::Left);
        assert_eq!(turn_right(Direction::Left), Direction::Up);
    }
    #[test]
    fn test_looping() {
        let map = vec![
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '#', '#'],
            vec!['#', '.', '.', '.', '#', '.', '.', '.', '.', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#', '#', '.', '#'],
            vec!['#', '.', '#', '.', '#', '.', '#', '#', '.', '#'],
            vec!['#', '#', '#', '#', '#', '#', '#', '#', '.', '#'],
        ];

        let start = ((1, 1), Direction::Down);
        assert_eq!(is_looping(&map, start), true);

        let start = ((1, 1), Direction::Right);
        assert_eq!(is_looping(&map, start), true);

        let start = ((1, 5), Direction::Down);
        assert_eq!(is_looping(&map, start), false);

        let map = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['#', '.', '.', '.'],
            vec!['#', '.', '#', '.'],
        ];
        let start = ((1, 1), Direction::Down);
        assert_eq!(is_looping(&map, start), false);
        let start = ((1, 1), Direction::Up);
        assert_eq!(is_looping(&map, start), true);
    }

    #[test]
    fn test_add_obstacle_ahead() {
        let map = vec![
            vec!['.', '#', '.', '.'],
            vec!['.', '.', '.', '#'],
            vec!['#', '.', '.', '.'],
            vec!['#', '.', '#', '.'],
        ];
        let start = ((1, 1), Direction::Down);
        let new_map = add_obstacle_ahead(&map, start).unwrap();
        assert_eq!(new_map[2][1], '#');

        let start = ((1, 1), Direction::Up);
        let new_map = add_obstacle_ahead(&map, start);
        assert_eq!(new_map, None);

        let start = ((0, 2), Direction::Up);
        let new_map = add_obstacle_ahead(&map, start);
        assert_eq!(new_map, None);
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
    let mut current_position = start;
    let mut current_direction = Direction::Up;
    loop {
        visited_positions.insert(current_position);
        if is_going_over_edge((current_position, current_direction), &map) {
            break;
        }

        let next_position = get_neighbour_coords(current_position, current_direction);
        if map[next_position.0 as usize][next_position.1 as usize] == '#' {
            current_direction = turn_right(current_direction);
        } else {
            current_position = next_position;
        }
    }

    visited_positions.len()
}

type PosAndDir = ((isize, isize), Direction);

fn is_going_over_edge(pos: PosAndDir, map: &[Vec<char>]) -> bool {
    match pos.1 {
        Direction::Up => pos.0 .0 == 0,
        Direction::Down => pos.0 .0 == map.len() as isize - 1,
        Direction::Left => pos.0 .1 == 0,
        Direction::Right => pos.0 .1 == map[0].len() as isize - 1,
        _ => false,
    }
}

fn get_original_path(map: &[Vec<char>], start: PosAndDir) -> HashSet<PosAndDir> {
    let mut visited_positions = HashSet::new();
    let mut current_position = start.0;
    let mut current_direction = start.1;
    loop {
        visited_positions.insert((current_position, current_direction));
        // If edge of map, break
        if is_going_over_edge((current_position, current_direction), map) {
            break;
        }

        let next_position = get_neighbour_coords(current_position, current_direction);
        if map[next_position.0 as usize][next_position.1 as usize] == '#' {
            current_direction = turn_right(current_direction);
        } else {
            current_position = next_position;
        }
    }

    visited_positions
}

fn add_obstacle_ahead(map: &[Vec<char>], start: PosAndDir) -> Option<Vec<Vec<char>>> {
    if !is_going_over_edge(start, map) {
        let next_pos = get_neighbour_coords(start.0, start.1);
        if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
            return None;
        }

        let mut new_map = map.to_owned();
        let next_pos = get_neighbour_coords(start.0, start.1);
        new_map[next_pos.0 as usize][next_pos.1 as usize] = '#';
        return Some(new_map);
    }
    None
}

fn is_looping(map: &[Vec<char>], start: PosAndDir) -> bool {
    let mut visited_positions = HashSet::new();
    let mut current_position_and_dir = start;
    loop {
        if is_going_over_edge(current_position_and_dir, map) {
            return false;
        }
        if visited_positions.contains(&current_position_and_dir) {
            return true;
        }
        visited_positions.insert(current_position_and_dir);
        let next_position =
            get_neighbour_coords(current_position_and_dir.0, current_position_and_dir.1);
        if map[next_position.0 as usize][next_position.1 as usize] == '#' {
            current_position_and_dir.1 = turn_right(current_position_and_dir.1);
        } else {
            current_position_and_dir.0 = next_position;
        }
    }
}

pub fn run_part2(input: &[String]) -> usize {
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

    let original_path = get_original_path(&map, (start, Direction::Up));

    let mut obstacle_positions = HashSet::new();

    for pos in original_path.iter() {
        let next_pos = get_neighbour_coords(pos.0, pos.1);
        // Skip the guards starting possition, not allowed!
        if next_pos.0 == start.0 && next_pos.1 == start.1 {
            continue;
        }

        if obstacle_positions.contains(&next_pos) {
            continue;
        }

        if let Some(new_map) = add_obstacle_ahead(&map, *pos) {
            if is_looping(&new_map, (start, Direction::Up)) {
                obstacle_positions.insert(next_pos);
            }
        }
    }

    obstacle_positions.len()
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
