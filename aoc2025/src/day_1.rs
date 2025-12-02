use std::str::FromStr;

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Day1 {
    turns: Vec<Turn>,
}

impl Day1 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day1 {
    fn name(&self) -> (usize, usize) {
        (2025, 1)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("aoc2025/input/day_1.txt");
        self.turns = lines.parse_lines();
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut setting = 50;
        let mut counter = 0;
        for turn in &self.turns {
            match turn {
                Turn::Left(amount) => setting = (setting - amount).rem_euclid(100),
                Turn::Right(amount) => setting = (setting + amount) % 100,
            }
            if setting == 0 {
                counter += 1;
            }
        }
        aoclib::output(counter)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut setting = 50isize;
        let mut counter = 0;
        for turn in &self.turns {
            let (dir, mut amount) = match turn {
                Turn::Left(amnt) => (-1, *amnt),
                Turn::Right(amnt) => (1, *amnt),
            };

            while amount > 0 {
                setting = (setting + dir).rem_euclid(100);
                if setting == 0 {
                    counter += 1;
                }
                amount -= 1;
            }
        }

        aoclib::output(counter)
    }
}

#[derive(Debug)]
enum Turn {
    Left(isize),
    Right(isize),
}

impl FromStr for Turn {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let is_left = &s[0..1] == "L";
        let amount = s[1..].parse::<isize>().unwrap();
        Ok(if is_left {
            Turn::Left(amount)
        } else {
            Turn::Right(amount)
        })
    }
}
