use std::{ops::SubAssign, str::FromStr};

use crate::{DayImpl, InputType};

pub struct Day {
    input_type: InputType,
}

const INPUT: &str = include_str!("input");
const EX_INPUT: &str = include_str!("example_input");

impl Day {
    pub fn new(input_type: InputType) -> Self {
        Self { input_type }
    }

    fn get_input(&self) -> &str {
        match self.input_type {
            InputType::Actual => INPUT,
            InputType::Example => EX_INPUT,
        }
    }
}

struct Battery(Vec<u8>);

impl From<&str> for Battery {
    fn from(value: &str) -> Self {
        Battery(
            value
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        )
    }
}

fn parse_input(input: &str) -> Vec<Battery> {
    input.lines().map(From::from).collect()
}

impl DayImpl for Day {
    fn part_one(&self) -> String {
        let input = parse_input(self.get_input());

        let result: usize = input
            .iter()
            .map(|Battery(digits)| {
                let mut max = *digits.iter().max().unwrap();
                let first_max_pos = loop {
                    // Get first occurrence of max. If it has no digit to its right then decrease max and try again
                    match digits.iter().position(|digit| digit == &max) {
                        Some(pos) if pos == digits.len() - 1 => {
                            max -= 1;
                            continue;
                        }
                        Some(m) => break m,
                        None => {
                            max -= 1; // ex. We decreased from 9 to 8, but 8 isnt in the battery, try 7
                        }
                    };
                };

                let remaining = &digits[(first_max_pos + 1)..];
                let remaining_max = remaining.iter().max().unwrap();
                (max as usize) * 10 + (*remaining_max as usize)
            })
            .sum();
        format!("{}", result)
    }

    fn part_two(&self) -> String {
        fn solve_segment(digits: &[u8], n: usize) -> usize {
            if n == 0 {
                return 0;
            }
            let mut max = *digits.iter().max().unwrap();
            let len = digits.len();

            let first_max_pos = loop {
                match digits.iter().position(|digit| digit == &max) {
                    None => max -= 1,
                    Some(pos) if len - pos < n => max -= 1,
                    Some(m) => break m,
                };
            };

            10_usize.pow((n - 1) as u32) * (max as usize) + solve_segment(&digits[first_max_pos + 1..], n - 1)
        }

        let input = parse_input(self.get_input());
        let result: usize = input
            .iter()
            .map(|Battery(digits)| solve_segment(digits, 12))
            .sum();
        format!("{result}")
    }
}
