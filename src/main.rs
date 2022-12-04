use clap::Parser;
use solution::Solution;

mod day01;
mod day02;
mod solution;
mod util;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    day: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let registry: Vec<Box<dyn Solution>> =
        vec![Box::new(day01::Day01 {}), Box::new(day02::Day02 {})];

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
