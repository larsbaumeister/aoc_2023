#![allow(dead_code)]
use std::error::Error;

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() -> Result<(), Box<dyn Error>> {

    day_4::solve()?;
    Ok(())
}
