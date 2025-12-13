use std::{convert::Infallible, str::FromStr};

use aoclib::Runner;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Day6 {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Day6 {
    pub fn new() -> Self {
        Self::default()
    }

    /**
     * 123
     * 456
     * 789
     * we get => ["147", "258", "369"]
     */
    fn columns_as_strings(&self) -> Vec<String> {
        let width = self.grid.iter().map(|row| row.len()).max().unwrap();
        let mut rotated = vec![String::new(); width];

        for row in &self.grid {
            for (col, ch) in row.iter().enumerate() {
                rotated[col].push(*ch);
            }
        }

        rotated
    }
}

impl Runner for Day6 {
    fn name(&self) -> (usize, usize) {
        (2025, 6)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("aoc2025/input/day_6.txt");
        self.rows = lines.len();
        self.cols = lines[0].len();
        for line in lines.iter() {
            self.grid.push(line.chars().collect());
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let mut rows: Vec<Vec<usize>> = Vec::new();
        let mut lines = Vec::new();
        for i in 0..self.rows {
            lines.push(self.grid[i].iter().collect::<String>());
        }

        for line in &lines[0..lines.len() - 1] {
            rows.push(
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            );
        }
        let ops = lines[lines.len() - 1]
            .split_whitespace()
            .map(|op| {
                if op == "*" {
                    Operation::Multiply
                } else {
                    Operation::Add
                }
            })
            .collect::<Vec<Operation>>();

        let mut total = 0;
        for i in 0..ops.len() {
            match ops[i] {
                Operation::Multiply => {
                    let mut ans = 1;
                    for entry in &rows {
                        ans *= entry[i]
                    }
                    total += ans;
                }
                Operation::Add => {
                    let mut ans = 0;
                    for entry in &rows {
                        ans += entry[i]
                    }
                    total += ans;
                }
            }
        }

        aoclib::output(total)
    }

    fn part_two(&mut self) -> Vec<String> {
        let rotated = self.columns_as_strings();

        let (total, acc, _) = rotated
            .iter()
            .map(|line| line.parse::<Line>().unwrap())
            .chain([Line::Blank])
            .fold((0,0, Operation::Add), |(total, acc, op), line| {
                match (line, &op) {
                    (Line::Product(p), _) => (total, p, Operation::Multiply),
                    (Line::Sum(s), _) => (total, s, Operation::Add),
                    (Line::Number(n), Operation::Add) => (total, acc + n, op),
                    (Line::Number(n), Operation::Multiply) => (total, acc * n, op),
                    (Line::Blank, _) => (total + acc, 0, op),
                }
            });

        assert_eq!(acc, 0); // to check if we have flushed the accumulator to 0
        aoclib::output(total)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Multiply,
    Add,
}

#[derive(Debug)]
enum Line {
    Product(usize),
    Sum(usize),
    Number(usize),
    Blank,
}

impl FromStr for Line {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('*') {
            Ok(Self::Product(s[0..s.len() - 1].trim().parse().unwrap()))
        } else if s.contains('+') {
            Ok(Self::Sum(s[0..s.len() - 1].trim().parse().unwrap()))
        } else if s.trim().is_empty() {
            Ok(Self::Blank)
        } else {
            Ok(Self::Number(s.trim().parse().unwrap()))
        }
    }
}
