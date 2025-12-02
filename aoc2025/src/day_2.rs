use std::str::FromStr;

use aoclib::Runner;

#[derive(Default)]
pub struct Day2 {
    ranges: Vec<Range>,
}

impl Day2 {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

impl Runner for Day2 {
    fn name(&self) -> (usize, usize) {
        (2025, 2)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("aoc2025/input/day_2.txt");
        self.ranges = lines[0]
            .split(",")
            .map(|range| range.parse().unwrap())
            .collect();
    }

    fn part_one(&mut self) -> Vec<String> {
        let out = self
            .ranges
            .iter()
            .map(Range::sum_invalid_ids)
            .sum::<usize>();
        aoclib::output(out)
    }

    fn part_two(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

impl Range {
    fn sum_invalid_ids(&self) -> usize {
        (self.start..=self.end)
            .filter(|num| {
                let half_digit = num.ilog10().div_ceil(2);
                let mod_value = 10usize.pow(half_digit);
                let upper_half = num / mod_value;
                let lower_half = num % mod_value;
                lower_half == upper_half
            })
            .sum()
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once('-').unwrap();
        Ok(Range {
            start: left.parse().unwrap(),
            end: right.parse().unwrap(),
        })
    }
}
