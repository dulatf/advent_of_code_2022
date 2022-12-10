use std::collections::HashSet;

use crate::solution::*;

pub struct Day06 {}

fn check_distinct(chars: &[char]) -> bool {
    let mut ctr: HashSet<char> = HashSet::new();
    for c in chars.iter() {
        if ctr.contains(c) {
            return false;
        } else {
            ctr.insert(*c);
        }
    }
    true
}

impl Solution for Day06 {
    fn part1(&self) -> Result<()> {
        println!("Day 06 - Part 1");
        let input = std::fs::read_to_string("data/day06.txt")?;
        let characters: Vec<char> = input.chars().collect();
        for i in 3..characters.len() {
            if check_distinct(&characters[i - 3..i + 1]) {
                println!("Start of packet marker found at: {}", i + 1);
                break;
            }
        }
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 06 - Part 2");
        let input = std::fs::read_to_string("data/day06.txt")?;
        let characters: Vec<char> = input.chars().collect();
        for i in 13..characters.len() {
            if check_distinct(&characters[i - 13..i + 1]) {
                println!("Start of message marker found at: {}", i + 1);
                break;
            }
        }
        Ok(())
    }
}
