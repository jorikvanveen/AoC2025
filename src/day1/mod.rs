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

#[derive(Debug, Clone)]
struct Rotation {
    direction: Direction,
    degrees: i16,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(|line| {
            let (dir_char, deg_str) = line.split_at(1);
            let direction = match dir_char {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction character: {}", dir_char),
            };
            let degrees: i16 = deg_str.parse().expect("Invalid degrees");
            Rotation { direction, degrees }
        })
        .collect()
}

impl DayImpl for Day {
    fn part_one(&self) -> String {
        let input = parse_input(self.get_input());

        let mut pointing_at = 50;
        let mut zero_count = 0;

        for Rotation { degrees, direction } in input {
            pointing_at = match direction {
                Direction::Left => pointing_at - degrees,
                Direction::Right => pointing_at + degrees,
            } % 100;

            if pointing_at == 0 {
                zero_count += 1;
            }
        }

        format!("{}", zero_count)
    }

    fn part_two(&self) -> String {
        let input = parse_input(self.get_input());
        let mut pointing_at = 50;
        let mut zero_count = 0;

        for Rotation { degrees, direction } in input {
            let full_rotations = degrees / 100;
            zero_count += full_rotations;

            let degrees = degrees.rem_euclid(100);
            let old_pointing_at = pointing_at.clone();
            pointing_at = match direction {
                Direction::Left => pointing_at - degrees,
                Direction::Right => pointing_at + degrees,
            };
            if (pointing_at < 0 && old_pointing_at != 0) || pointing_at > 100 {
                zero_count += 1;
            }
            pointing_at = pointing_at.rem_euclid(100);

            if pointing_at == 0 {
                zero_count += 1;
            }
        }

        format!("{}", zero_count)
    }
}
