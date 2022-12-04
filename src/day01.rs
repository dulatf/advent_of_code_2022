use crate::solution::*;

pub struct Day01 {}

fn compute_line_sum(lines: &str) -> Result<i64> {
    let values: std::result::Result<Vec<_>, _> =
        lines.split("\n").map(|line| line.parse::<i64>()).collect();
    Ok(values?.iter().sum())
}

fn get_calories_per_elf(input: &str) -> Result<Vec<i64>> {
    let lines_per_elf = input.split("\n\n");
    let calories_per_elf: std::result::Result<Vec<_>, _> =
        lines_per_elf.map(compute_line_sum).collect();
    calories_per_elf
}

pub fn get_top_n_sum(slice: &[i64], n: usize) -> Option<i64> {
    let mut heap = std::collections::BinaryHeap::<i64>::new();
    heap.extend(slice.iter());
    let mut result = 0;
    for i in 0..n {
        result += heap.peek()?;
        heap.pop();
    }
    return Some(result);
}

impl Solution for Day01 {
    fn part1(&self) -> Result<()> {
        println!("Day 01 - Part 1");
        let input = std::fs::read_to_string("data/day01.txt")?;
        let calories_per_elf = get_calories_per_elf(&input);
        let max_calories = calories_per_elf?.into_iter().max().unwrap_or(0);
        println!("Maximum calories carried: {}", max_calories);
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 01 - Part 2");
        let input = std::fs::read_to_string("data/day01.txt")?;
        let calories_per_elf = get_calories_per_elf(&input);
        let top_3_sum = get_top_n_sum(&calories_per_elf?, 3);
        println!(
            "Calories carried by the top 3 elves: {}",
            top_3_sum.unwrap()
        );
        Ok(())
    }
}
