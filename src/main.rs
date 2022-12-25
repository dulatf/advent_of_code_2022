use clap::Parser;
use solution::Solution;

mod solution;
mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    day: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let registry: Vec<Box<dyn Solution>> = vec![
        Box::new(day01::Day01 {}),
        Box::new(day02::Day02 {}),
        Box::new(day03::Day03 {}),
        Box::new(day04::Day04 {}),
        Box::new(day05::Day05 {}),
        Box::new(day06::Day06 {}),
        Box::new(day07::Day07 {}),
        Box::new(day08::Day08 {}),
        Box::new(day09::Day09 {}),
        Box::new(day10::Day10 {}),
        Box::new(day11::Day11 {}),
        Box::new(day12::Day12 {}),
    ];

    if let Some(selected_day) = args.day {
        if selected_day < 1 || selected_day > registry.len() {
            panic!(
                "Invalid day ({}), supported values: 1..{}",
                selected_day,
                registry.len()
            );
        }
        registry[selected_day - 1].part1().unwrap();
        registry[selected_day - 1].part2().unwrap();
    } else {
        for day in registry {
            day.part1().unwrap();
            day.part2().unwrap();
        }
    }
}
