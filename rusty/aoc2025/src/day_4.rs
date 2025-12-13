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

    fn find_accessible(&self) -> Vec<(isize, isize)> {
        self.rolls
            .iter()
            .filter(|roll| {
                DIRS
                    .iter()
                    .map(|dir| {
                        let (dr, dc) = dir.unit();
                        (roll.0 + dr, roll.1 + dc)
                    })
                    .filter(|pos| self.rolls.contains(pos))
                    .count()
                    < 4
            })
            .copied()
            .collect()
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
        aoclib::output(self.find_accessible().len())
    }

    fn part_two(&mut self) -> Vec<String> {

        let mut total_removed = 0;

        loop {
            let acc = self.find_accessible();

            if acc.is_empty() {
                break;
            }

            total_removed += acc.len();

            for roll in &acc {
                self.rolls.remove(roll);
            }
        }

        aoclib::output(total_removed)
    }
}

