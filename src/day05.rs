use std::collections::HashMap;

use crate::solution::*;
use simple_error::SimpleError;

pub struct Day05 {}

fn parse_stacks(input: &str) -> HashMap<i32, Vec<char>> {
    let lines: Vec<&str> = input.split("\n").collect();
    let numbers: Vec<&str> = lines[lines.len() - 1].split_whitespace().collect();
    let stack_positions: HashMap<i32, usize> = numbers
        .into_iter()
        .map(|i| {
            (
                i.parse::<i32>().unwrap(),
                lines[lines.len() - 1].find(i).unwrap(),
            )
        })
        .collect();
    let mut stacks = HashMap::<i32, Vec<char>>::new();
    for (label, _) in stack_positions.iter() {
        stacks.insert(*label, Vec::new());
    }
    for i in (0..(lines.len() - 1)).rev() {
        for (label, idx) in stack_positions.iter() {
            if *idx < lines[i].len() {
                let crate_char = lines[i].chars().skip(*idx).next().unwrap();
                if crate_char != ' ' {
                    stacks.get_mut(label).unwrap().push(crate_char);
                }
            }
        }
    }
    return stacks;
}

fn process_line(stacks: &mut HashMap<i32, Vec<char>>, line: &str) -> Result<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 6 {
        return Err(Box::new(SimpleError::new(format!(
            "Invalid line: {}",
            line
        ))));
    }
    let (num_crates, source, dest) = (
        parts[1].parse::<i32>().unwrap(),
        parts[3].parse::<i32>().unwrap(),
        parts[5].parse::<i32>().unwrap(),
    );
    for i in 0..num_crates {
        if let Some(c) = stacks.get_mut(&source).and_then(|x| x.pop()) {
            stacks.get_mut(&dest).unwrap().push(c);
        } else {
            return Err(Box::new(SimpleError::new("invalid instructions")));
        }
    }
    Ok(())
}

fn process_line_part2(stacks: &mut HashMap<i32, Vec<char>>, line: &str) -> Result<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 6 {
        return Err(Box::new(SimpleError::new(format!(
            "Invalid line: {}",
            line
        ))));
    }
    let (num_crates, source, dest) = (
        parts[1].parse::<i32>().unwrap(),
        parts[3].parse::<i32>().unwrap(),
        parts[5].parse::<i32>().unwrap(),
    );
    let mut temp_stack: Vec<char> = Vec::new();
    for i in 0..num_crates {
        if let Some(c) = stacks.get_mut(&source).and_then(|x| x.pop()) {
            temp_stack.push(c);
        } else {
            return Err(Box::new(SimpleError::new("invalid instructions")));
        }
    }
    for i in 0..num_crates {
        if let Some(c) = temp_stack.pop() {
            stacks.get_mut(&dest).unwrap().push(c);
        } else {
            return Err(Box::new(SimpleError::new("invalid instructions")));
        }
    }
    Ok(())
}

impl Solution for Day05 {
    fn part1(&self) -> Result<()> {
        println!("Day 05 - Part 1");
        let input = std::fs::read_to_string("data/day05.txt")?;
        let parts: Vec<&str> = input.split("\n\n").collect();
        if parts.len() != 2 {
            return Err(Box::new(SimpleError::new("Invalid input")));
        }
        let mut stacks = parse_stacks(parts[0]);
        for line in parts[1].lines() {
            process_line(&mut stacks, line)?;
        }
        println!("Top of the stacks:");
        for i in 0..stacks.len() {
            let top = stacks[&((i + 1) as i32)].last().unwrap();
            print!("{}", top);
        }
        println!();
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 05 - Part 2");
        let input = std::fs::read_to_string("data/day05.txt")?;
        let parts: Vec<&str> = input.split("\n\n").collect();
        if parts.len() != 2 {
            return Err(Box::new(SimpleError::new("Invalid input")));
        }
        let mut stacks = parse_stacks(parts[0]);
        for line in parts[1].lines() {
            process_line_part2(&mut stacks, line)?;
        }
        println!("Top of the stacks:");
        for i in 0..stacks.len() {
            let top = stacks[&((i + 1) as i32)].last().unwrap();
            print!("{}", top);
        }
        println!();
        Ok(())
    }
}
