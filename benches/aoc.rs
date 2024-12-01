use criterion::{criterion_group, criterion_main};

mod day1;
// Add days here

criterion_group!(day1, day1::bench_day1_p1, day1::bench_day1_p2);
// Add day group here

// Do not remove from last line
criterion_main!(day1);
