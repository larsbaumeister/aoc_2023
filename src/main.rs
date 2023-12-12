#![allow(dead_code)]
use std::error::Error;
use clap::Parser;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;

#[derive(clap::ValueEnum, Debug, Clone)]
enum Challenge {
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
    Day12
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The challenge to run
    #[arg(required = true)]
    challenge: Challenge,

    /// The part of the challenge to run
    #[arg(required = true)]
    part: u8,
}

pub trait Solution {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>>;
}

fn main() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();

    let solution: Box<dyn Solution> = match args.challenge {
        Challenge::Day1 => Box::new(day_1::Day1::new()),
        Challenge::Day2 => Box::new(day_2::Day2::new()),
        Challenge::Day3 => Box::new(day_3::Day3::new()),
        Challenge::Day4 => Box::new(day_4::Day4::new()),
        Challenge::Day5 => Box::new(day_5::Day5::new()),
        Challenge::Day6 => Box::new(day_6::Day6::new()),
        Challenge::Day7 => Box::new(day_7::Day7::new()),
        Challenge::Day8 => Box::new(day_8::Day8::new()),
        Challenge::Day9 => Box::new(day_9::Day9::new()),
        Challenge::Day10 => Box::new(day_10::Day10::new()),
        Challenge::Day11 => Box::new(day_11::Day11::new()),
        Challenge::Day12 => Box::new(day_12::Day12::new()),
    };
    solution.solve(args.part)?;
    Ok(())
}
 