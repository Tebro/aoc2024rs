use criterion::{criterion_group, criterion_main};

mod day1;
mod day2;
mod day3;
mod day4;
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
criterion_group!(
    day3,
    day3::bench_day3_p1,
    day3::bench_day3_p1_alt,
    day3::bench_day3_p2,
    day3::bench_day3_p2_alt,
);
criterion_group!(
    day4,
    day4::bench_day4_p1,
    day4::bench_day4_p1_alt,
    day4::bench_day4_p1_alt2,
    day4::bench_day4_p2
);
// Add day group here

// Do not remove from last line
criterion_main!(day1, day2, day3, day4);
