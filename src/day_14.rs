use std::error::Error;

use crate::Solution;


pub struct Day14 {}

impl Day14 {
    pub fn new() -> Day14 {
        Day14 {}
    }
}

impl Solution for Day14 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => {
                println!("Day 14 Part 1");
            }
            2 => {
                println!("Day 14 Part 2");
            }
            _ => println!("Invalid part {}", part),
        }
        Ok(())
    }
}