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

#[derive(Clone, Copy, Debug)]
enum Operation {
    Mul,
    Add,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        match value {
            "*" => Operation::Mul,
            "+" => Operation::Add,
            _ => panic!("Invalid operation"),
        }
    }
}

#[derive(Debug, Clone)]
struct Worksheet(Vec<(Operation, Vec<usize>)>);
impl Worksheet {
    pub fn evaluate(&self) -> usize {
        self.0
            .iter()
            .map(|(operation, numbers)| {
                numbers.iter().fold(
                    match operation {
                        Operation::Add => 0,
                        Operation::Mul => 1,
                    },
                    |total, num| match operation {
                        Operation::Mul => total * num,
                        Operation::Add => total + num,
                    },
                )
            })
            .sum()
    }
}

impl DayImpl for Day {
    fn part_one(&self) -> String {
        fn parse_input(input: &str) -> Worksheet {
            let lines: Vec<Vec<String>> = input
                .lines()
                .map(|line| {
                    line.split(" ")
                        .filter(|segment| segment != &"")
                        .map(From::from)
                        .collect()
                })
                .collect();

            let mut numbers: Vec<Vec<usize>> = vec![];
            let mut operations: Vec<Operation> = vec![];
            for (i, line) in lines.iter().enumerate() {
                if i == lines.len() - 1 {
                    operations = line.iter().map(String::as_str).map(From::from).collect();
                    break;
                }

                numbers.push(line.iter().map(|num| num.parse().unwrap()).collect());
            }

            let mut exercises: Vec<(Operation, Vec<usize>)> = vec![];

            for col in 0..numbers[0].len() {
                let mut ex_numbers = vec![];
                for row in 0..numbers.len() {
                    ex_numbers.push(numbers[row][col])
                }
                exercises.push((operations[col], ex_numbers));
            }
            Worksheet(exercises)
        }
        let sheet = parse_input(&self.get_input());
        let result: usize = sheet.evaluate();
        format!("{result}")
    }

    fn part_two(&self) -> String {
        fn parse_input(input: &str) -> Worksheet {
            let last_line = input.lines().last().unwrap();
            let col_widths = {
                let mut col_widths = vec![];
                let mut last_op_idx = 0;
                for (i, c) in last_line.chars().enumerate() {
                    match c {
                        '\n' | '*' | '+' => {
                            let width = i - last_op_idx;
                            last_op_idx = i;
                            if width == 0 {
                                continue;
                            }
                            col_widths.push(width - 1);
                        }
                        _ => continue,
                    }
                }
                col_widths.push(last_line.len() - last_op_idx);
                col_widths
            };

            let operations: Vec<Operation> = last_line
                .trim()
                .split(" ")
                .filter(|s| s != &"")
                .map(From::from)
                .collect();

            let number_lines: Vec<Vec<char>> = {
                let mut l = input.lines().rev();
                let _ = l.next();
                l.rev().map(|line| line.chars().collect()).collect()
            };

            let mut exercises: Vec<(Operation, Vec<usize>)> = vec![];
            for (col, col_width) in col_widths.iter().enumerate() {
                let mut col_numbers = vec![];
                let row_start_idx = col_widths[..col].iter().sum::<usize>() + col;
                for i in 0..*col_width {
                    let mut num_str = String::new();
                    for row in number_lines.iter() {
                        match row[row_start_idx + i] {
                            ' ' => continue,
                            c => num_str.push(c),
                        };
                    }
                    col_numbers.push(num_str.parse().unwrap())
                }
                exercises.push((operations[col], col_numbers));
            }
            Worksheet(exercises)
        }

        let sheet = parse_input(&self.get_input());
        let result = sheet.evaluate();
        format!("{result}")
    }
}
