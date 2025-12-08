use std::{collections::HashSet};

use aoclib::{ Runner, DIRS};

#[derive(Default)]
pub struct Day4 {
    rolls: HashSet<(isize, isize)>,
    rows: isize,
    cols: isize,
}

impl Day4 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day4 {
    fn name(&self) -> (usize, usize) {
        (2025, 4)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("aoc2025/input/day_4.txt");
        self.rows = lines.len() as isize;
        self.cols = lines[0].len() as isize;

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '@' {
                    self.rolls.insert((row as isize, col as isize));
                }
            }
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut accessible = 0;

        for row in 0..self.rows {
            for col in 0..self.cols {

            if !self.rolls.contains(&(row, col)) {
                continue;
            }

            let mut count: usize = 0;

                for delta in DIRS {
                    let (dr, dc) = delta.unit();
                    count += self.rolls.contains(&(row + dr, col + dc)) as usize;
                }

                if count < 4 {
                    accessible += 1
                }
            }
        }
        aoclib::output(accessible)
    }

    fn part_two(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

