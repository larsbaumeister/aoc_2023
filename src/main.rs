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

#[derive(clap::ValueEnum, Debug, Clone)]
enum Challenge {
    DAY_1,
    DAY_2,
    DAY_3,
    DAY_4,
    DAY_5,
    DAY_6,
    DAY_7,
    DAY_8,
    DAY_9,
    DAY_10,
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
/* 
    let args = Args::parse();

    let solution: Box<dyn Solution> = match args.challenge {
        Challenge::DAY_1 => Box::new(day_1::Day1::new()),
        Challenge::DAY_2 => Box::new(day_2::Day2::new()),
        Challenge::DAY_3 => Box::new(day_3::Day3::new()),
        Challenge::DAY_4 => Box::new(day_4::Day4::new()),
        Challenge::DAY_5 => Box::new(day_5::Day5::new()),
        Challenge::DAY_6 => Box::new(day_6::Day6::new()),
        Challenge::DAY_7 => Box::new(day_7::Day7::new()),
        Challenge::DAY_8 => Box::new(day_8::Day8::new()),
        Challenge::DAY_9 => Box::new(day_9::Day9::new()),
        Challenge::DAY_10 => Box::new(day_10::Day10::new()),
    };
    solution.solve(args.part)?; */

    day_10::Day10::new().solve(2)?;
    Ok(())
}
 