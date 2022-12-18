use std::collections::HashSet;

use crate::solution::*;
use simple_error::SimpleError;

pub struct Day09 {}

#[derive(Clone, Debug)]
struct LongState {
    pub knots: Vec<(i32, i32)>,
}

impl LongState {
    pub fn new(num_knots: usize) -> Self {
        Self {
            knots: vec![(0, 0); num_knots + 1],
        }
    }
    pub fn knot_distance(&self, knot_idx: usize) -> Option<(i32, i32)> {
        if knot_idx == 0 || knot_idx > self.knots.len() {
            return None;
        }
        let head = self.knots[knot_idx - 1];
        let tail = self.knots[knot_idx];
        Some((head.0 - tail.0, head.1 - tail.1))
    }

    fn move_head(&mut self, direction: &Direction, distance: i32) {
        let (x, y) = self.knots[0];
        self.knots[0] = match direction {
            Direction::Left => (x - distance, y),
            Direction::Right => (x + distance, y),
            Direction::Up => (x, y + distance),
            Direction::Down => (x, y - distance),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn update_long_positions(state: &mut LongState, direction: &Direction) -> LongState {
    state.move_head(direction, 1);
    for i in 1..state.knots.len() {
        let (dx, dy) = state.knot_distance(i).unwrap();
        let (tx, ty) = state.knots[i];
        if dx.abs() >= 2 || dy.abs() >= 2 {
            state.knots[i] = (tx + dx.signum(), ty + dy.signum());
        }
    }
    state.clone()
}

impl Solution for Day09 {
    fn part1(&self) -> Result<()> {
        println!("Day 09 - Part 1");
        let input = std::fs::read_to_string("data/day09.txt")?;
        let lines = input
            .lines()
            .map(|line| parse_line(line))
            .collect::<std::result::Result<Vec<(Direction, i32)>, SimpleError>>()?;
        let commands: Vec<Direction> = lines
            .into_iter()
            .map(|(dir, count)| vec![dir; count as usize])
            .flatten()
            .collect();
        let states: Vec<LongState> = commands
            .iter()
            .scan(LongState::new(1), |state, dir| {
                Some(update_long_positions(state, dir))
            })
            .collect();
        let tail_positions: HashSet<(i32, i32)> = states
            .into_iter()
            .map(|state| *state.knots.last().unwrap())
            .collect();
        println!("Number of unique tail positions: {}", tail_positions.len());
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 09 - Part 2");
        let input = std::fs::read_to_string("data/day09.txt")?;
        let lines = input
            .lines()
            .map(|line| parse_line(line))
            .collect::<std::result::Result<Vec<(Direction, i32)>, SimpleError>>()?;
        let commands: Vec<Direction> = lines
            .into_iter()
            .map(|(dir, count)| vec![dir; count as usize])
            .flatten()
            .collect();
        let states: Vec<LongState> = commands
            .iter()
            .scan(LongState::new(9), |state, dir| {
                Some(update_long_positions(state, dir))
            })
            .collect();
        let tail_positions: HashSet<(i32, i32)> = states
            .into_iter()
            .map(|state| *state.knots.last().unwrap())
            .collect();
        println!("Number of unique tail positions: {}", tail_positions.len());
        Ok(())
    }
}

fn parse_line(line: &str) -> std::result::Result<(Direction, i32), SimpleError> {
    let parts: Vec<&str> = line.split(" ").collect();
    if parts.len() != 2 {
        return Err(SimpleError::new("Invalid input line"));
    }
    let direction = match parts[0] {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => return Err(SimpleError::new(format!("Invalid direction in {}", line))),
    };
    if let Ok(steps) = parts[1].parse::<i32>() {
        Ok((direction, steps))
    } else {
        Err(SimpleError::new(format!("Couldn't parse steps: {}", line)))
    }
}
