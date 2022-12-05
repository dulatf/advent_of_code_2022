use crate::solution::*;
use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
enum ProcessError {
    InvalidItem(char),
    NoItemInBoth,
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for ProcessError {}

pub struct Day03 {}

fn priority(c: char) -> Result<i8> {
    if c.is_ascii_lowercase() {
        Ok((c as i8) - ('a' as i8) + 1)
    } else if c.is_ascii_uppercase() {
        Ok((c as i8) - ('A' as i8) + 27)
    } else {
        Err(Box::new(ProcessError::InvalidItem(c)))
    }
}

fn common_item(lists: &[&[char]]) -> Result<char> {
    let mut sets: Vec<HashSet<char>> = Vec::new();
    for list in lists {
        let mut set = HashSet::new();
        set.extend(list.iter());
        sets.push(set);
    }

    'outer: for c in sets.first().unwrap().iter() {
        for set in sets.iter() {
            if !set.contains(c) {
                continue 'outer;
            }
        }
        return Ok(*c);
    }
    Err(Box::new(ProcessError::NoItemInBoth))
}

fn process_line(line: &str) -> Result<i64> {
    let elements: Vec<char> = line.chars().collect();
    let first_compartment = &elements[0..elements.len() / 2];
    let second_compartment = &elements[elements.len() / 2..];
    let common = common_item(&[first_compartment, second_compartment])?;
    priority(common).map(|x| x as i64)
}

impl Solution for Day03 {
    fn part1(&self) -> Result<()> {
        println!("Day 03 - Part 1");
        let input = std::fs::read_to_string("data/day03.txt")?;
        let processed: Result<Vec<i64>> = input.lines().map(process_line).collect();
        let priority_sum: i64 = processed?.iter().sum();
        println!("Priority sum: {}", priority_sum);

        Ok(())
    }
    fn part2(&self) -> Result<()> {
        println!("Day 03 - Part 2");
        let input = std::fs::read_to_string("data/day03.txt")?;
        let lines: Vec<&str> = input.lines().collect();
        let mut priority_sum: i64 = 0;
        for i in 0..lines.len() / 3 {
            let l0: Vec<char> = lines[3 * i + 0].chars().collect();
            let l1: Vec<char> = lines[3 * i + 1].chars().collect();
            let l2: Vec<char> = lines[3 * i + 2].chars().collect();
            let common = common_item(&[&l0, &l1, &l2])?;
            priority_sum += priority(common)? as i64;
        }
        println!("Priority sum: {}", priority_sum);
        Ok(())
    }
}
