use std::{cmp::max, collections::HashSet};

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

    pub fn get_input(&self) -> String {
        match self.input_type {
            InputType::Actual => INPUT.into(),
            InputType::Example => EX_INPUT.into(),
        }
    }
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut split = input.trim().split("\n\n");
    let ranges = split.next().unwrap();
    let ids = split.next().unwrap();

    let ranges = ranges
        .trim()
        .lines()
        .map(|line| {
            let mut split = line.trim().split("-");
            let lower = split.next().unwrap();
            let higher = split.next().unwrap();
            (lower.parse().unwrap(), higher.parse().unwrap())
        })
        .collect();
    let ids = ids
        .trim()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    (ranges, ids)
}

impl DayImpl for Day {
    fn part_one(&self) -> String {
        let (ranges, ids) = parse_input(&self.get_input());
        let result = ids
            .iter()
            .filter(|id| ranges.iter().any(|(low, high)| id >= &low && id <= &high))
            .count();
        format!("{result}")
    }

    fn part_two(&self) -> String {
        fn merge_ranges(a: &(usize, usize), b: &(usize, usize)) -> Option<(usize, usize)> {
            // Make sure that a.0 <= b.0
            let (a, b) = match a.0 <= b.0 {
                true => (a, b),
                false => (b, a),
            };

            // Ranges overlap if a.1 >= b.0
            if a.1 >= b.0 {
                Some((a.0, max(a.1, b.1)))
            } else {
                None
            }
        }

        let (ranges, _) = parse_input(&self.get_input());
        let mut ranges: HashSet<(usize, usize)> = ranges.into_iter().collect();

        loop {
            let mut did_change = false;
            let mut new_ranges = ranges.clone();
            'outer: for range_a in ranges.iter().cloned() {
                for range_b in ranges.iter().cloned() {
                    if range_a == range_b {
                        continue;
                    }
                    if let Some(new_range) = merge_ranges(&range_a, &range_b) {
                        new_ranges.remove(&range_a);
                        new_ranges.remove(&range_b);
                        new_ranges.insert(new_range);
                        did_change = true;
                        break 'outer;
                    }
                }
            }
            ranges = new_ranges;
            if !did_change {
                break;
            }
        }

        let id_count: usize = ranges.iter().map(|(low, high)| high - low + 1).sum();
        format!("{id_count}")
    }
}
