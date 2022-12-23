use crate::solution::*;
use simple_error::SimpleError;

pub struct Day10 {}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(i32),
}

fn parse_line(line: &str) -> std::result::Result<Instruction, SimpleError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() == 1 {
        if parts[0] == "noop" {
            return Ok(Instruction::Noop);
        }
    } else if parts.len() == 2 {
        if parts[0] == "addx" {
            if let Ok(value) = parts[1].parse::<i32>() {
                return Ok(Instruction::AddX(value));
            }
        }
    }
    Err(SimpleError::new(format!("Invalid line: '{}'", line)))
}
#[derive(Debug, Clone)]
struct State {
    cycle_counter: usize,
    register: i32,
}
impl Default for State {
    fn default() -> Self {
        Self {
            cycle_counter: 0,
            register: 1,
        }
    }
}

impl State {
    pub fn update(&self, instruction: Instruction) -> State {
        match instruction {
            Instruction::Noop => State {
                cycle_counter: self.cycle_counter + 1,
                register: self.register,
            },
            Instruction::AddX(value) => State {
                cycle_counter: self.cycle_counter + 2,
                register: self.register + value,
            },
        }
    }
}

fn register_value_at_cycle(states: &Vec<State>, cycle_number: usize) -> Option<i32> {
    if states.is_empty() {
        return None;
    }
    if cycle_number > states.last().unwrap().cycle_counter {
        return None;
    }
    let mut left: usize = 0;
    let mut right: usize = states.len();
    while left < right {
        let mid = (left + right) / 2;
        let mid_cycle = states[mid].cycle_counter;
        if right - left == 1 && mid_cycle < cycle_number {
            return Some(states[mid].register);
        }
        if mid_cycle == cycle_number {
            return Some(states[mid].register);
        } else if mid_cycle < cycle_number {
            left = mid;
        } else {
            right = mid;
        }
    }
    None
}

impl Solution for Day10 {
    fn part1(&self) -> Result<()> {
        println!("Day 10 - Part 1");
        let input = std::fs::read_to_string("data/day10.txt")?;
        let instructions = input
            .lines()
            .map(parse_line)
            .collect::<std::result::Result<Vec<Instruction>, SimpleError>>()?;
        let states: Vec<State> = instructions
            .into_iter()
            .scan(State::default(), |state, instruction| {
                let new_state = state.update(instruction);
                let retval = new_state.clone();
                *state = new_state;
                Some(retval)
            })
            .collect();
        let mut cycle = 20;
        let mut cycle_values: Vec<(usize, i64)> = Vec::new();
        loop {
            if let Some(cycle_val) = register_value_at_cycle(&states, cycle - 1) {
                cycle_values.push((cycle, cycle_val as i64));
                cycle += 40;
            } else {
                break;
            }
        }
        let signal_strength: i64 = cycle_values.iter().map(|(x, y)| (*x as i64) * (*y)).sum();
        println!("Signal strength: {:#?}", signal_strength);
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 10 - Part 2");
        let input = std::fs::read_to_string("data/day10.txt")?;
        let instructions = input
            .lines()
            .map(parse_line)
            .collect::<std::result::Result<Vec<Instruction>, SimpleError>>()?;
        let states: Vec<State> = instructions
            .into_iter()
            .scan(State::default(), |state, instruction| {
                let new_state = state.update(instruction);
                let retval = new_state.clone();
                *state = new_state;
                Some(retval)
            })
            .collect();
        for cycle in 0..states.last().unwrap().cycle_counter {
            let xpos = (cycle) % 40;
            let register_value = register_value_at_cycle(&states, cycle).unwrap_or(1);
            if ((xpos as i32) - register_value).abs() <= 1{
                print!("#")
            }else {
                print!(".")
            }
            if xpos == 39 {
                println!();
            }
        }
        Ok(())
    }
}
