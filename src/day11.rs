use crate::solution::*;
use simple_error::SimpleError;

pub struct Day11 {}

#[derive(Debug)]
enum Operand {
    Variable,
    Number(i64),
}

#[derive(Debug)]
enum Operation {
    Sum(Operand, Operand),
    Product(Operand, Operand),
}

fn operand_value(operand: &Operand, old_value: i64) -> i64 {
    match operand {
        Operand::Variable => old_value,
        Operand::Number(num) => *num,
    }
}

fn evaluate(operation: &Operation, old_value: i64) -> i64 {
    match operation {
        Operation::Sum(op1, op2) => operand_value(op1, old_value) + operand_value(op2, old_value),
        Operation::Product(op1, op2) => {
            operand_value(op1, old_value) * operand_value(op2, old_value)
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    divisibility_test: i64,
    true_target: usize,
    false_target: usize,
}

fn parse_operand(input: &str) -> Option<Operand> {
    if input == "old" {
        Some(Operand::Variable)
    } else {
        let num = input.parse::<i64>().ok()?;
        Some(Operand::Number(num))
    }
}

fn parse_monkey(input: &str) -> Option<Monkey> {
    let lines = input.lines();
    let mut items: Option<Vec<i64>> = None;
    let mut operation: Option<Operation> = None;
    let mut div_test: Option<i64> = None;
    let mut true_target: Option<usize> = None;
    let mut false_target: Option<usize> = None;

    for line in lines.skip(1) {
        let payload = line.split(":").last()?;
        if line.contains("Starting items:") {
            let parts: Vec<Option<i64>> = payload
                .split(",")
                .map(|x| x.trim().parse::<i64>().ok())
                .collect();
            if parts.contains(&None) {
                println!("Invalid line: `{}`", line);
                return None;
            }
            items = Some(parts.into_iter().map(|x| x.unwrap()).collect::<Vec<i64>>());
        } else if line.contains("Operation:") {
            let payload = line.split("=").last()?;
            if payload.contains("+") {
                let parts: Vec<&str> = payload.split("+").collect();
                if parts.len() != 2 {
                    println!("Invalid operand count in: `{}`", line);
                    return None;
                }
                let op0 = parse_operand(parts[0].trim())?;
                let op1 = parse_operand(parts[1].trim())?;
                operation = Some(Operation::Sum(op0, op1));
            } else if payload.contains("*") {
                let parts: Vec<&str> = payload.split("*").collect();
                if parts.len() != 2 {
                    println!("Invalid operand count in: `{}`", line);
                    return None;
                }
                let op0 = parse_operand(parts[0].trim())?;
                let op1 = parse_operand(parts[1].trim())?;
                operation = Some(Operation::Product(op0, op1));
            } else {
                println!("Invalid Operation definition: `{}`", line);
                return None;
            }
        } else if line.contains("Test:") {
            let parts = line.trim().split_whitespace();
            div_test = parts.last()?.parse::<i64>().ok();
        } else if line.contains("If true:") {
            let parts = line.trim().split_whitespace();
            true_target = parts.last()?.parse::<usize>().ok();
        } else if line.contains("If false:") {
            let parts = line.trim().split_whitespace();
            false_target = parts.last()?.parse::<usize>().ok();
        } else {
            println!("Ignoring line: `{}`", line);
        }
    }
    Some(Monkey {
        items: items?,
        operation: operation?,
        divisibility_test: div_test?,
        true_target: true_target?,
        false_target: false_target?,
    })
}

fn run_monkey(monkeys: &mut [Monkey], idx: usize, divide: bool) -> usize {
    let mut items: Vec<i64> = monkeys[idx].items.iter().rev().map(|x| *x).collect();
    let inspections = items.len();
    let modulus: i64 = monkeys.iter().map(|x| x.divisibility_test).product();
    while let Some(level) = items.pop() {
        let next_level = evaluate(&monkeys[idx].operation, level);
        let reduced = if divide {
            next_level / 3
        } else {
            next_level % modulus
        };
        let target = if reduced % monkeys[idx].divisibility_test == 0 {
            monkeys[idx].true_target
        } else {
            monkeys[idx].false_target
        };
        monkeys[target].items.push(reduced);
    }
    monkeys[idx].items.clear();
    inspections
}

impl Solution for Day11 {
    fn part1(&self) -> Result<()> {
        println!("Day 11 - Part 1");
        let input = std::fs::read_to_string("data/day11.txt")?;
        let definitions = input.split("\n\n");
        let monkeys: Option<Vec<Monkey>> = definitions.map(parse_monkey).collect();
        if let Some(mut monkeys) = monkeys {
            let mut inspect_counters = vec![0; monkeys.len()];
            for _ in 0..20 {
                for i in 0..monkeys.len() {
                    inspect_counters[i] += run_monkey(&mut monkeys, i, true);
                }
            }
            println!("{:?}", inspect_counters);
            inspect_counters.sort();
            println!("{:?}", inspect_counters);
            let monkey_business = inspect_counters[inspect_counters.len() - 1]
                * inspect_counters[inspect_counters.len() - 2];

            println!("Level of monkey business: {}", monkey_business);
            Ok(())
        } else {
            Err(Box::new(SimpleError::new("Invalid monkeys")))
        }
    }

    fn part2(&self) -> Result<()> {
        println!("Day 11 - Part 2");
        let input = std::fs::read_to_string("data/day11.txt")?;
        let definitions = input.split("\n\n");
        let monkeys: Option<Vec<Monkey>> = definitions.map(parse_monkey).collect();
        if let Some(mut monkeys) = monkeys {
            let mut inspect_counters = vec![0; monkeys.len()];
            for _ in 0..10000 {
                for i in 0..monkeys.len() {
                    inspect_counters[i] += run_monkey(&mut monkeys, i, false);
                }
            }
            println!("{:?}", inspect_counters);
            inspect_counters.sort();
            println!("{:?}", inspect_counters);
            let monkey_business = inspect_counters[inspect_counters.len() - 1]
                * inspect_counters[inspect_counters.len() - 2];

            println!("Level of monkey business: {}", monkey_business);
            Ok(())
        } else {
            Err(Box::new(SimpleError::new("Invalid monkeys")))
        }
    }
}
