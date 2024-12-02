use criterion::{black_box, Criterion};

use aoc2024::*;

pub fn bench_day2_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day2.txt");
    c.bench_function("Day2 Part 1", |b| {
        b.iter(|| day2::run_part1(black_box(&lines)))
    });
}
pub fn bench_day2_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day2.txt");
    c.bench_function("Day2 Part 2", |b| {
        b.iter(|| day2::run_part2(black_box(&lines)))
    });
}

pub fn bench_day2_p2_alt(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day2.txt");
    c.bench_function("Day2 Part 2 alt", |b| {
        b.iter(|| day2::run_part2_alt(black_box(&lines)))
    });
}
