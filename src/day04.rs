use crate::solution::*;
use simple_error::SimpleError;

pub struct Day04 {}

fn parse_range(input: &str) -> Result<(i64, i64)> {
    let parts: Vec<&str> = input.split("-").collect();
    if parts.len() != 2 {
        return Err(Box::new(SimpleError::new("Invalid range")));
    }
    Ok((parts[0].parse()?, parts[1].parse()?))
}

fn parse_line(input: &str) -> Result<((i64, i64), (i64, i64))> {
    let parts: Vec<&str> = input.split(",").collect();
    if parts.len() != 2 {
        return Err(Box::new(SimpleError::new("Invalid line")));
    }
    let range1 = parse_range(parts[0])?;
    let range2 = parse_range(parts[1])?;
    Ok((range1, range2))
}

fn ranges_fully_overlap(range1: &(i64, i64), range2: &(i64, i64)) -> bool {
    if (range1.0 <= range2.0 && range1.1 >= range2.1)
        || (range2.0 <= range1.0 && range2.1 >= range1.1)
    {
        true
    } else {
        false
    }
}

fn ranges_overlap(range1: &(i64, i64), range2: &(i64, i64)) -> bool {
    (range1.0 <= range2.0 && range1.1 >= range2.0)
        || (range1.0 <= range2.1 && range1.1 >= range2.1)
        || (range2.0 <= range1.0 && range2.1 >= range1.0)
        || (range2.0 <= range1.1 && range2.1 >= range1.1)
}

impl Solution for Day04 {
    fn part1(&self) -> Result<()> {
        println!("Day 04 - Part 1");
        let input = std::fs::read_to_string("data/day04.txt")?;
        let ranges: Result<Vec<((i64, i64), (i64, i64))>> = input.lines().map(parse_line).collect();
        let fully_overlapping = ranges?
            .into_iter()
            .filter(|(r1, r2)| ranges_fully_overlap(r1, r2))
            .count();
        println!("Number of pairs that fully overlap: {}", fully_overlapping);
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 04 - Part 2");
        let input = std::fs::read_to_string("data/day04.txt")?;
        let ranges: Result<Vec<((i64, i64), (i64, i64))>> = input.lines().map(parse_line).collect();
        let overlapping = ranges?
            .into_iter()
            .filter(|(r1, r2)| ranges_overlap(r1, r2))
            .count();
        println!("Number of pairs that overlap at all: {}", overlapping);
        Ok(())
    }
}
