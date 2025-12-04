use std::fmt::format;

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

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Paper,
}

impl From<char> for Cell {
    fn from(value: char) -> Self {
        match value {
            '.' => Cell::Empty,
            '@' => Cell::Paper,
            _ => panic!("Invalid cell value: {value}"),
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(From::from).collect())
        .collect()
}

impl DayImpl for Day {
    fn part_one(&self) -> String {
        let grid = parse_input(&self.get_input());
        let count: usize = (0..grid.len())
            .map(|y| {
                (0..grid[0].len())
                    .filter(|x| {
                        if let Some(Cell::Empty) = grid.get(y).and_then(|row| row.get(*x)) {
                            return false;
                        };
                        let adj_paper_count = [
                            (0, -1),
                            (0, 1),
                            (1, 0),
                            (-1, 0),
                            (-1, -1),
                            (-1, 1),
                            (1, -1),
                            (1, 1),
                        ]
                        .iter()
                        .filter(|(x_offset, y_offset)| {
                            let adj_x: usize = match (*x as isize + x_offset).try_into() {
                                Ok(x) => x,
                                Err(_) => return false,
                            };
                            let adj_y: usize = match (y as isize + y_offset).try_into() {
                                Ok(y) => y,
                                Err(_) => return false,
                            };
                            match grid.get(adj_y).and_then(|row| row.get(adj_x)) {
                                Some(Cell::Paper) => true,
                                _ => false,
                            }
                        })
                        .count();
                        adj_paper_count < 4
                    })
                    .count()
            })
            .sum();

        format!("{count}")
    }

    fn part_two(&self) -> String {
        let mut grid = parse_input(&self.get_input());
        let mut removed_count = 0;

        loop {
            let remove_positions: Vec<(usize, usize)> = (0..grid.len())
                .map(|y| {
                    (0..grid[0].len()).filter_map({
                        let y = y.clone();
                        let grid = grid.clone();
                        move |x| {
                            if let Some(Cell::Empty) = grid.get(y).and_then(|row| row.get(x)) {
                                return None;
                            };
                            let adj_paper_count = [
                                (0, -1),
                                (0, 1),
                                (1, 0),
                                (-1, 0),
                                (-1, -1),
                                (-1, 1),
                                (1, -1),
                                (1, 1),
                            ]
                            .iter()
                            .filter(|(x_offset, y_offset)| {
                                let adj_x: usize = match (x as isize + x_offset).try_into() {
                                    Ok(x) => x,
                                    Err(_) => return false,
                                };
                                let adj_y: usize = match (y as isize + y_offset).try_into() {
                                    Ok(y) => y,
                                    Err(_) => return false,
                                };
                                match grid.get(adj_y).and_then(|row| row.get(adj_x)) {
                                    Some(Cell::Paper) => true,
                                    _ => false,
                                }
                            })
                            .count();
                            if adj_paper_count < 4 {
                                Some((x.clone(), y))
                            } else {
                                None
                            }
                        }
                    })
                })
                .flatten()
                .collect();
            if remove_positions.len() == 0 {
                break
            }
            removed_count += remove_positions.len();
            for (x, y) in remove_positions {
                grid[y][x] = Cell::Empty
            }
        }
        format!("{removed_count}")
    }
}
