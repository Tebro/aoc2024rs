#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::wildcard_imports)]
#[allow(dead_code, unused_imports)]
use std::io;

use aoc2024::*;

fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    match day.parse::<i32>().unwrap() {
        1 => day1::run()?,
        2 => day2::run()?,
        3 => day3::run()?,
        4 => day4::run()?,
        5 => day5::run()?,
        // Day invocations
        _ => {}
    };

    Ok(())
}
