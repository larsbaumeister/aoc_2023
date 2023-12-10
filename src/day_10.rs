use std::error::Error;

use crate::Solution;


pub struct Day10 {}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {}
    }

    fn solve() {
        
    }
}

impl Solution for Day10 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => {
                Day10::solve()
            }
            2 => {
                Day10::solve()
            }
            _ => println!("Invalid part specified")
        }
        Ok(())
    }
}
