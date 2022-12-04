use crate::solution::*;
pub struct Day02 {}

#[derive(Clone, Copy)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Move::Rock,
            'B' => Move::Paper,
            'C' => Move::Scissors,
            'X' => Move::Rock,
            'Y' => Move::Paper,
            'Z' => Move::Scissors,
            _ => panic!("Invalid move spec: '{}'", c),
        }
    }
    fn score(&self) -> i64 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}
impl Outcome {
    fn from_char(c: char) -> Self {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Invalid outcome spec: '{}'", c),
        }
    }
}

fn choose_move(opponent: Move, outcome: Outcome) -> Move {
    match opponent {
        Move::Rock => match outcome {
            Outcome::Loss => Move::Scissors,
            Outcome::Draw => Move::Rock,
            Outcome::Win => Move::Paper,
        },
        Move::Paper => match outcome {
            Outcome::Loss => Move::Rock,
            Outcome::Draw => Move::Paper,
            Outcome::Win => Move::Scissors,
        },
        Move::Scissors => match outcome {
            Outcome::Loss => Move::Paper,
            Outcome::Draw => Move::Scissors,
            Outcome::Win => Move::Rock,
        },
    }
}

fn score(opponent: Move, player: Move) -> i64 {
    let choice_score = player.score();
    let win_score = match (opponent, player) {
        (Move::Rock, Move::Paper) => 6,
        (Move::Rock, Move::Scissors) => 0,
        (Move::Paper, Move::Rock) => 0,
        (Move::Paper, Move::Scissors) => 6,
        (Move::Scissors, Move::Rock) => 6,
        (Move::Scissors, Move::Paper) => 0,
        _ => 3,
    };
    win_score + choice_score
}

impl Solution for Day02 {
    fn part1(&self) -> Result<()> {
        println!("Day 02 - Part 1");
        let input = std::fs::read_to_string("data/day02.txt")?;
        let moves = input.split("\n").map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() != 2 || parts[0].len() != 1 || parts[1].len() != 1 {
                panic!("Invalid line in strategy guide: '{}'", line);
            }
            (
                Move::from_char(parts[0].chars().next().unwrap()),
                Move::from_char(parts[1].chars().next().unwrap()),
            )
        });
        let total_score: i64 = moves
            .map(|(opponent, player)| score(opponent, player))
            .sum();
        println!("Total score: {}", total_score);
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 02 - Part 1");
        let input = std::fs::read_to_string("data/day02.txt")?;
        let rounds = input.split("\n").map(|line| {
            let parts: Vec<&str> = line.split(" ").collect();
            if parts.len() != 2 || parts[0].len() != 1 || parts[1].len() != 1 {
                panic!("Invalid line in strategy guide: '{}'", line);
            }
            (
                Move::from_char(parts[0].chars().next().unwrap()),
                Outcome::from_char(parts[1].chars().next().unwrap()),
            )
        });
        let total_score: i64 = rounds
            .map(|(opponent, outcome)| {
                let player_move = choose_move(opponent, outcome);
                score(opponent, player_move)
            })
            .sum();
        println!("Total score: {}", total_score);
        Ok(())
    }
}
