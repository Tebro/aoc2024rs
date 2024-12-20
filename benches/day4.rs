use criterion::{black_box, Criterion};

use aoc2024::*;

pub fn bench_day4_p1(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");
    c.bench_function("Day4 Part 1", |b| {
        b.iter(|| day4::run_part1(black_box(&lines)))
    });
}
pub fn bench_day4_p1_alt(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");
    c.bench_function("Day4 Part 1 alt", |b| {
        b.iter(|| day4::run_part1_alt(black_box(&lines)))
    });
}
pub fn bench_day4_p1_alt2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");
    c.bench_function("Day4 Part 1 alt 2", |b| {
        b.iter(|| day4::run_part1_alt2(black_box(&lines)))
    });
}
pub fn bench_day4_p2(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");
    c.bench_function("Day4 Part 2", |b| {
        b.iter(|| day4::run_part2(black_box(&lines)))
    });
}

pub fn bench_day4_p2_alt(c: &mut Criterion) {
    let lines = helpers::read_file_to_vec::<String>("inputs/day4.txt");
    c.bench_function("Day4 Part 2 alt", |b| {
        b.iter(|| day4::run_part2_alt(black_box(&lines)))
    });
}
