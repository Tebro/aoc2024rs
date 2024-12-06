use criterion::{black_box, Criterion};

use aoc2024::*;

pub fn bench_day6_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day6.txt");
    c.bench_function("Day6 Part 1", |b| {
        b.iter(|| day6::run_part1(black_box(&lines)))
    });
}
pub fn bench_day6_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day6.txt");
    c.bench_function("Day6 Part 2", |b| {
        b.iter(|| day6::run_part2(black_box(&lines)))
    });
}
