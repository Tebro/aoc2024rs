#![allow(unused)]
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

pub fn read_file_to_vec<A>(filename: &str) -> Vec<A>
where
    A: FromStr,
    <A as FromStr>::Err: fmt::Debug,
{
    let file = File::open(filename).expect("Could not open");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|x| match x {
            Ok(d) => d.parse::<A>().unwrap(),
            Err(e) => panic!("{:?}", e),
        })
        .collect::<Vec<A>>()
}
pub fn read_file_to_vecs(filename: &str) -> Vec<Vec<u32>> {
    let lines: Vec<String> = read_file_to_vec(filename);

    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

pub fn read_line_to_vec<A>(filename: &str) -> Vec<A>
where
    A: FromStr,
    <A as FromStr>::Err: fmt::Debug,
{
    let mut file = File::open(filename).expect("Could not open");
    let mut s = String::new();
    file.read_to_string(&mut s)
        .expect("Could not read to string");

    s.split(',')
        .map(|x| x.parse::<A>().unwrap())
        .collect::<Vec<A>>()
}

pub fn mean(numbers: &[i32]) -> f32 {
    let sum: i32 = numbers.iter().sum();

    sum as f32 / numbers.len() as f32
}

pub fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();

    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&[numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }
}

pub fn mode(numbers: &[i32]) -> Vec<i32> {
    let mut map = HashMap::new();
    for integer in numbers {
        let count = map.entry(integer).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}
