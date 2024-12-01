//! Advent of Code 2024 by Troy F.

use aoc2024::{day01::day01, day02};
use clap::Parser;

fn main() {
    let args = Args::parse();
    match args.day {
        1 => day01(),
        2 => day02(),
        _ => panic!("bad day"),
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    day: usize,
}
