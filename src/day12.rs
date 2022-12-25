use std::collections::VecDeque;

use crate::solution::*;
use simple_error::SimpleError;

pub struct Day12 {}

#[derive(Debug)]
struct Terrain {
    pub rows: usize,
    pub cols: usize,
    pub heightmap: Vec<u8>,
    pub starting_idx: usize,
    pub target_idx: usize,
}

impl Terrain {
    pub fn new(input: &str) -> Option<Terrain> {
        let lines: Vec<&str> = input.lines().collect();
        let num_rows = lines.len();
        let num_cols = lines.first()?.len();
        let mut heightmap: Vec<u8> = vec![0; num_rows * num_cols];

        let mut starting_idx: Option<usize> = None;
        let mut target_idx: Option<usize> = None;

        let start_marker = 0x53u8; // 'S'
        let target_marker = 0x45u8; // 'E'

        for (row, line) in lines.iter().enumerate() {
            for (col, character) in line.bytes().enumerate() {
                let idx = row * num_cols + col;
                if character == start_marker {
                    starting_idx = Some(idx);
                    heightmap[idx] = 0;
                } else if character == target_marker {
                    target_idx = Some(idx);
                    heightmap[idx] = 0x7au8 - 0x61u8;
                } else {
                    heightmap[idx] = character - 0x61u8;
                }
            }
        }

        Some(Terrain {
            rows: num_rows,
            cols: num_cols,
            heightmap: heightmap,
            starting_idx: starting_idx?,
            target_idx: target_idx?,
        })
    }

    pub fn coordinates(&self, idx: usize) -> (usize, usize) {
        (idx % self.cols, idx / self.cols)
    }
    pub fn index(&self, (col, row): (usize, usize)) -> usize {
        row * self.cols + col
    }

    fn reachable(&self, target: (usize, usize), pos_idx: usize) -> bool {
        let target_idx = self.index(target);
        self.heightmap[target_idx] <= (self.heightmap[pos_idx] + 1)
    }

    pub fn shortest_path(&self, starting_index: usize) -> usize {
        let mut steps_to: Vec<usize> = vec![std::usize::MAX; self.rows * self.cols];
        let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
        queue.push_back((self.coordinates(starting_index), 0));
        while let Some(((cx, cy), current_steps)) = queue.pop_front() {
            let current_idx = self.index((cx, cy));
            if current_steps < steps_to[current_idx] {
                steps_to[current_idx] = current_steps;
                if cx > 0 && self.reachable((cx - 1, cy), current_idx) {
                    queue.push_back(((cx - 1, cy), current_steps + 1));
                }
                if cx < self.cols - 1 && self.reachable((cx + 1, cy), current_idx) {
                    queue.push_back(((cx + 1, cy), current_steps + 1));
                }
                if cy > 0 && self.reachable((cx, cy - 1), current_idx) {
                    queue.push_back(((cx, cy - 1), current_steps + 1));
                }
                if cy < self.rows - 1 && self.reachable((cx, cy + 1), current_idx) {
                    queue.push_back(((cx, cy + 1), current_steps + 1));
                }
            }
        }
        steps_to[self.target_idx]
    }
}

impl Solution for Day12 {
    fn part1(&self) -> Result<()> {
        println!("Day 12 - Part 1");
        let input = std::fs::read_to_string("data/day12.txt")?;
        let terrain = Terrain::new(&input).unwrap();
        let steps = terrain.shortest_path(terrain.starting_idx);
        println!("Shortest path: {}", steps);
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 12 - Part 2");
        let input = std::fs::read_to_string("data/day12.txt")?;
        let terrain = Terrain::new(&input).unwrap();
        let potential_starts = terrain
            .heightmap
            .iter()
            .enumerate()
            .filter(|(_, height)| **height == 0)
            .map(|(idx, _)| idx);
        let shortest_path = potential_starts
            .map(|x| terrain.shortest_path(x))
            .min()
            .unwrap();
        println!("Overall shortest path: {}", shortest_path);
        Ok(())
    }
}
