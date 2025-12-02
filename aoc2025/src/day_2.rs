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
        let out = self
            .ranges
            .iter()
            .map(Range::sum_multi_invalid)
            .sum::<usize>();
        aoclib::output(out)
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

    fn sum_multi_invalid(&self) -> usize {
        let mut invalid_sum = 0;

        for num in self.start..=self.end {
            let half_digit = num.ilog10().div_ceil(2);
            for digit_count in 1..=half_digit {
                let mod_value = 10usize.pow(digit_count);
                let last_n_digits = num % mod_value;
                let mut test_num = num / mod_value;
                if last_n_digits == 0 || last_n_digits.ilog10() + 1 != digit_count {
                    continue;
                }
                let mut found = true;
                while test_num > 0 {
                    found = test_num % mod_value == last_n_digits;
                    if !found {
                        break;
                    }
                    test_num /= mod_value;
                }

                if found {
                    invalid_sum += num;
                    break;
                }
            }
        }
        invalid_sum
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_multi_invalids1() {
        let range = Range {
            start: 824824821,
            end: 824824827,
        };
        assert_eq!(824824824, range.sum_multi_invalid());
    }

    #[test]
    fn test_multi_invalids2() {
        let range = Range {
            start: 70701,
            end: 70710,
        };
        assert_eq!(0, range.sum_multi_invalid());
    }
}
