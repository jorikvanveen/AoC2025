use std::str::FromStr;

use clap::{Parser, Subcommand};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    day: Day,

    #[arg(short, long)]
    part: Part
}

#[derive(Subcommand, Debug)]
enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
}

#[derive(Debug, Clone, Copy)]
enum Part {
    One,
    Two
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" | "one" | "One" => Ok(Part::One),
            "2" | "two" | "Two" => Ok(Part::Two),
            _ => Err(format!("Invalid part: {}", s))
        }
    }
}

pub(crate) trait DayImpl {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}

fn main() {
    let args = Args::parse();

    let day_impl: Box<dyn DayImpl> = match args.day {
        Day::Day1 => Box::new(day1::Day::new()),
        Day::Day2 => Box::new(day2::Day::new()),
        Day::Day3 => Box::new(day3::Day::new()),
        Day::Day4 => Box::new(day4::Day::new()),
        Day::Day5 => Box::new(day5::Day::new()),
        Day::Day6 => Box::new(day6::Day::new()),
        Day::Day7 => Box::new(day7::Day::new()),
        Day::Day8 => Box::new(day8::Day::new()),
        Day::Day9 => Box::new(day9::Day::new()),
        Day::Day10 => Box::new(day10::Day::new()),
        Day::Day11 => Box::new(day11::Day::new()),
        Day::Day12 => Box::new(day12::Day::new()),
    };

    match args.part {
        Part::One => println!("Part 1: {}", day_impl.part_one()),
        Part::Two => eprintln!("Part 2: {}", day_impl.part_two()),
    }
}
