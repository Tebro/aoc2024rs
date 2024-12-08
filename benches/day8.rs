use criterion::{black_box, Criterion};

use aoc2024::*;

pub fn bench_day8_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day8.txt");
    c.bench_function("Day8 Part 1", |b| {
        b.iter(|| day8::run_part1(black_box(&lines)))
    });
}
pub fn bench_day8_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day8.txt");
    c.bench_function("Day8 Part 2", |b| {
        b.iter(|| day8::run_part2(black_box(&lines)))
    });
}
