use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

struct Range {
    first: usize,
    last: usize,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut split = value.split("-");
        let first = split
            .next()
            .expect("Invalid range")
            .parse()
            .expect("Invalid int");
        let last = split
            .next()
            .expect("Invalid range")
            .parse()
            .expect("Invalid int");
        Self { first, last }
    }
}

fn parse_input(input: &str) -> Vec<Range> {
    input.trim().split(",").map(Range::from).collect()
}

impl DayImpl for Day {
    fn part_one(&self) -> String {
        fn is_valid(id: String) -> bool {
            if id.chars().next() == Some('0') {
                return false;
            }

            if id.len() % 2 != 0 {
                return true;
            }

            let midpoint = id.len() / 2;
            let (first_half, last_half) = id.split_at(midpoint);
            return first_half != last_half;
        }

        let input = parse_input(self.get_input());

        let sum: usize = input
            .par_iter()
            .map(|range| {
                (range.first..=range.last)
                    .into_iter()
                    .filter(|id| !is_valid(format!("{}", id)))
                    .sum::<usize>()
            })
            .sum();

        format!("{}", sum)
    }

    fn part_two(&self) -> String {
        fn segments_of_len(id: &str, segment_len: usize) -> Vec<String> {
            assert_eq!(id.len() % segment_len, 0);
            let n = id.len() / segment_len;
            (0..n)
                .map(|i| id[(segment_len * i)..(segment_len * (i + 1))].into())
                .collect()
        }

        fn is_valid(id: String) -> bool {
            if id.chars().next() == Some('0') {
                return false;
            }

            let len = id.len();
            let len_divisors: Vec<usize> = (1..=(len / 2))
                .filter(|divisor| len % divisor == 0)
                .collect();
            len_divisors
                .iter()
                .filter(|segment_len| {
                    let segments = segments_of_len(&id, **segment_len);
                    segments.iter().all(|segment| segment == &segments[0])
                })
                .count()
                == 0
        }

        let input = parse_input(self.get_input());

        let sum: usize = input
            .par_iter()
            .map(|range| {
                (range.first..=range.last)
                    .into_iter()
                    .filter(|id| !is_valid(format!("{}", id)))
                    .sum::<usize>()
            })
            .sum();
        format!("{}", sum)
    }
}
