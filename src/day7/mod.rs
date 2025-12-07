use std::{collections::HashMap, fmt::Display};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Splitter,
    Beam,
    Start,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Empty,
            '^' => Cell::Splitter,
            '|' => Cell::Beam,
            'S' => Cell::Start,
            c => panic!("Invalid cell: {c}"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cell::Empty => write!(f, "."),
            Cell::Splitter => write!(f, "^"),
            Cell::Beam => write!(f, "|"),
            Cell::Start => write!(f, "S"),
        }
    }
}

#[derive(Debug, Clone)]
struct TachyonManifold {
    map: Vec<Vec<Cell>>,
    start_position: (usize, usize),
}

impl Display for TachyonManifold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.map.iter() {
            for cell in row {
                write!(f, "{cell}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> TachyonManifold {
    let map: Vec<Vec<Cell>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(From::from).collect())
        .collect();
    let start_position = map
        .iter()
        .enumerate()
        .filter_map(|(row_idx, row)| {
            row.iter()
                .position(|cell| cell == &Cell::Start)
                .and_then(|col_idx| Some((row_idx, col_idx)))
        })
        .next()
        .unwrap();
    TachyonManifold {
        map,
        start_position,
    }
}

impl DayImpl for Day {
    fn part_one(&self) -> String {
        let mut m = parse_input(&self.get_input());
        m.map[m.start_position.0 + 1][m.start_position.1] = Cell::Beam;

        let mut split_count = 0;
        for row_idx in m.start_position.0 + 1..m.map.len() - 1 {
            for col_idx in 0..m.map[row_idx].len() {
                if m.map[row_idx][col_idx] == Cell::Beam {
                    let cell_below = &m.map[row_idx + 1][col_idx];
                    match cell_below {
                        Cell::Empty => m.map[row_idx + 1][col_idx] = Cell::Beam,
                        Cell::Splitter => {
                            m.map[row_idx + 1][col_idx - 1] = Cell::Beam;
                            m.map[row_idx + 1][col_idx + 1] = Cell::Beam;
                            split_count += 1;
                        }
                        _ => continue,
                    }
                }
            }
            println!("{m}");
        }

        format!("{split_count}")
    }

    fn part_two(&self) -> String {
        let mut m = parse_input(&self.get_input());
        m.map[m.start_position.0 + 1][m.start_position.1] = Cell::Beam;

        fn count_timelines(
            map: &Vec<Vec<Cell>>,
            cache: &mut HashMap<(usize, usize), usize>,
            beam_head: (usize, usize),
            d: usize,
        ) -> usize {
            if let Some(timelines) = cache.get(&beam_head) {
                return *timelines;
            }
            let below_pos = (beam_head.0 + 1, beam_head.1);
            let below_cell = match map.get(below_pos.0).and_then(|row| row.get(below_pos.1)) {
                Some(c) => c,
                None => return 1,
            };
            let result = if below_cell == &Cell::Splitter {
                count_timelines(map, cache, (beam_head.0 + 1, beam_head.1 - 1), d + 1)
                    + count_timelines(map, cache, (beam_head.0 + 1, beam_head.1 + 1), d + 1)
            } else {
                count_timelines(map, cache, below_pos, d + 1)
            };
            cache.insert(beam_head, result);
            result
        }

        let mut cache = HashMap::new();
        let timelines = count_timelines(
            &m.map,
            &mut cache,
            (m.start_position.0 + 1, m.start_position.1),
            0,
        );

        format!("{timelines}")
    }
}
