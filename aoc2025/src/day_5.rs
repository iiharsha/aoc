
use std::{ ops::RangeInclusive};

use aoclib::{ Runner};

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Day5 {
    ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl Day5 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day5 {
    fn name(&self) -> (usize, usize) {
        (2025, 5)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("aoc2025/input/day_5.txt");
        for line in lines {
            if line.contains('-') {
                let (left, right) = line.split_once('-').unwrap();
                let start: usize = left.parse().unwrap();
                let end: usize = right.parse().unwrap();
                self.ranges.push(start..=end);
            } else {
                self.ingredients.push(line.parse().unwrap());
            }
        }
    }

    fn part_one(&mut self) -> Vec<String> {
        let res = self.ingredients
            .iter()
            .filter(|ingredient| self.ranges.iter().any(|range| range.contains(ingredient)))
            .count();

        aoclib::output(res)
    }

    fn part_two(&mut self) -> Vec<String> {
        let mut intervals: Vec<(usize, usize)> = self.ranges.iter().map(|range| (*range.start(), *range.end())).collect();
        intervals.sort(); // sorts by start

        let mut merged : Vec<(usize, usize)>= Vec::new();
        for (s, e) in intervals {
            if let Some((_last_s, last_e)) = merged.last_mut() {
                if s <= *last_e + 1 {
                    *last_e = (*last_e).max(e);
                } else {
                    merged.push((s, e));
                }
            } else {
                merged.push((s, e));
            }
        }

        let mut num = 0;
        for (s, e) in &merged {
            num += e - s + 1;
        }

        aoclib::output(num)
    }
}
