use crate::solution::*;

pub struct Day08 {}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    trees: Vec<u8>,
}

struct Buffers {
    pub highest_left: Vec<u8>,
    pub highest_right: Vec<u8>,
    pub highest_up: Vec<u8>,
    pub highest_down: Vec<u8>,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut trees: Vec<u8> = Vec::new();
        for line in lines {
            for i in 0..width {
                trees.push(line[i..i + 1].parse::<u8>().unwrap());
            }
        }
        Grid {
            width: width,
            height: height,
            trees: trees,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn prepare_buffers(&self) -> Buffers {
        let mut highest_left: Vec<u8> = vec![0; self.trees.len()];
        let mut highest_right: Vec<u8> = vec![0; self.trees.len()];
        let mut highest_up: Vec<u8> = vec![0; self.trees.len()];
        let mut highest_down: Vec<u8> = vec![0; self.trees.len()];

        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                let idx = self.index(x, y);
                let left = self.index(x - 1, y);
                let up = self.index(x, y - 1);
                if self.trees[left] > highest_left[left] {
                    highest_left[idx] = self.trees[left];
                } else {
                    highest_left[idx] = highest_left[left];
                }
                if self.trees[up] > highest_up[up] {
                    highest_up[idx] = self.trees[up];
                } else {
                    highest_up[idx] = highest_up[up];
                }
            }
        }
        for y in (1..(self.height - 1)).rev() {
            for x in (1..(self.width - 1)).rev() {
                let idx = self.index(x, y);
                let right = self.index(x + 1, y);
                let down = self.index(x, y + 1);
                if self.trees[right] > highest_right[right] {
                    highest_right[idx] = self.trees[right];
                } else {
                    highest_right[idx] = highest_right[right];
                }
                if self.trees[down] > highest_down[down] {
                    highest_down[idx] = self.trees[down];
                } else {
                    highest_down[idx] = highest_down[down];
                }
            }
        }
        Buffers {
            highest_left,
            highest_right,
            highest_up,
            highest_down,
        }
    }

    fn compute_visibility(&self, buffers: &Buffers) -> usize {
        let mut visible: Vec<bool> = vec![true; self.trees.len()];
        for y in 1..(self.height - 1) {
            for x in 1..(self.width - 1) {
                let idx = self.index(x, y);
                if self.trees[idx] > buffers.highest_left[idx]
                    || self.trees[idx] > buffers.highest_right[idx]
                    || self.trees[idx] > buffers.highest_up[idx]
                    || self.trees[idx] > buffers.highest_down[idx]
                {
                    visible[idx] = true;
                } else {
                    visible[idx] = false;
                }
            }
        }
        let num_visible = visible.into_iter().filter(|x| *x).count();
        num_visible
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let idx = self.index(x, y);
        let tree_height = self.trees[idx];
        if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
            return 0;
        }
        let mut left_viz = 0;
        for cx in (0..x).rev() {
            left_viz += 1;
            if self.trees[self.index(cx, y)] >= tree_height {
                break;
            }
        }
        let mut right_viz = 0;
        for cx in x + 1..self.width {
            right_viz += 1;
            if self.trees[self.index(cx, y)] >= tree_height {
                break;
            }
        }
        let mut up_viz = 0;
        for cy in (0..y).rev() {
            up_viz += 1;
            if self.trees[self.index(x, cy)] >= tree_height {
                break;
            }
        }
        let mut down_viz = 0;
        for cy in y + 1..self.height {
            down_viz += 1;
            if self.trees[self.index(x, cy)] >= tree_height {
                break;
            }
        }
        let scenic_score = left_viz * right_viz * up_viz * down_viz;
        scenic_score
    }
}

impl Solution for Day08 {
    fn part1(&self) -> Result<()> {
        println!("Day 08 - Part 1");
        let input = std::fs::read_to_string("data/day08.txt")?;
        let grid = Grid::new(&input);
        let buffers = grid.prepare_buffers();
        let num_visible = grid.compute_visibility(&buffers);
        println!("Number of visible tress: {}", num_visible);
        Ok(())
    }

    fn part2(&self) -> Result<()> {
        println!("Day 08 - Part 1");
        let input = std::fs::read_to_string("data/day08.txt")?;
        let grid = Grid::new(&input);
        let best_scenic_score = (0..grid.trees.len())
            .map(|idx| (idx % grid.width, idx / grid.width))
            .map(|(x, y)| grid.scenic_score(x, y))
            .max()
            .unwrap();
        println!("Best scenic score: {}", best_scenic_score);
        Ok(())
    }
}
