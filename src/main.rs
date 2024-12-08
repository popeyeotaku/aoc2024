//! Advent of Code 2024 by Troy F.

use aoc2024::{
    day01::day01, day02::day02, day03::day03, day04::day04, day05::day05, day06::day06,
    day07::day07,
};
use clap::Parser;

fn main() {
    let args = Args::parse();
    match args.day {
        1 => day01(),
        2 => day02(),
        3 => day03(),
        4 => day04(),
        5 => day05(),
        6 => day06(),
        7 => day07(),
        _ => panic!("bad day"),
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    day: usize,
}
