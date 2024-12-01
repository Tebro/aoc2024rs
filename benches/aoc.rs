use criterion::{criterion_group, criterion_main};

mod day1;
mod day2;
// Add days here

criterion_group!(
    day1,
    day1::bench_day1_p1,
    day1::bench_day1_p2,
    day1::bench_day1_p2_alt
);
criterion_group!(
    day2,
    day2::bench_day2_p1,
    day2::bench_day2_p2,
    day2::bench_day2_p2_alt
);
// Add day group here

// Do not remove from last line
criterion_main!(day1, day2);
