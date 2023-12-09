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

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    match args.challenge {
        Challenge::DAY_1 => day_1::solve()?,
        Challenge::DAY_2 => day_2::solve()?,
        Challenge::DAY_3 => day_3::solve()?,
        Challenge::DAY_4 => day_4::solve()?,
        Challenge::DAY_5 => day_5::solve()?,
        Challenge::DAY_6 => day_6::solve()?,
        Challenge::DAY_7 => day_7::solve()?,
        Challenge::DAY_8 => day_8::solve()?,
        Challenge::DAY_9 => day_9::solve()?,   
    }
    Ok(())
}
 