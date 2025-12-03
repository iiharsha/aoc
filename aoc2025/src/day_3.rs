use std::str::FromStr;

use aoclib::{LineParser, Runner};

#[derive(Default)]
pub struct Day3 {
    // something
    power_banks: Vec<PowerBank>,
}
impl Day3 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day3 {
    fn name(&self) -> (usize, usize) {
        (2025, 3)
    }

    fn parse(&mut self) {
        let lines = aoclib::read_lines("aoc2025/input/day_3.txt");
        self.power_banks = lines.parse_lines();
    }

    fn part_one(&mut self) -> Vec<String> {
        aoclib::output(
            self.power_banks
                .iter()
                .map(|bank| bank.best_number_by_digits())
                .sum::<usize>(),
        )
    }

    fn part_two(&mut self) -> Vec<String> {
        aoclib::output("unsolved")
    }
}

#[derive(Debug)]
struct PowerBank {
    bank: Vec<u8>,
}

impl PowerBank {
    fn best_number_by_digits(&self) -> usize {
        let Some((pos, &first_digit)) = self.bank[0..self.bank.len() - 1]
            .iter()
            .enumerate()
            .rev()
            .max_by(|a, b| a.1.cmp(b.1))
        else {
            panic!("no digits found")
        };
        let &second_digit = self.bank[pos + 1..].iter().max().unwrap();
        first_digit as usize * 10 + second_digit as usize
    }
}

impl FromStr for PowerBank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PowerBank {
            bank: s.chars().map(|ch| ch as u8 - b'0').collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_data = [
            ("987654321111111", 98),
            ("811111111111119", 89),
            ("234234234234278", 78),
            ("818181911112111", 92),
        ];

        for test in test_data {
            let bank: PowerBank = test.0.parse().unwrap();
            assert_eq!(test.1, bank.best_number_by_digits());
        }
    }
}
