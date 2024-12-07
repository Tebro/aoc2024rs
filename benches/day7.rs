use criterion::{black_box, Criterion};

use aoc2024::*;

pub fn bench_day7_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day7.txt");
    c.bench_function("Day7 Part 1", |b| {
        b.iter(|| day7::run_part1(black_box(&lines)))
    });
}
pub fn bench_day7_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day7.txt");
    c.bench_function("Day7 Part 2", |b| {
        b.iter(|| day7::run_part2(black_box(&lines)))
    });
}
