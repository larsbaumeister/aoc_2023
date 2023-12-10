use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

use crate::Solution;

pub struct Day1 {}

impl Solution for Day1 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => self._solve()?,
            2 => self._solve()?,
            _ => println!("Invalid part number"),
        }
        Ok(())
    }
}

impl Day1 {

    pub fn new() -> Day1 {
        Day1 {}
    }

    fn _solve(&self) -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(File::open("input/day_1.txt")?);
        
        let mut sum = 0;
        for line in reader.lines() {
            let line = line?;
            let first_digit = Day1::find_first_digit(&line);
            let last_digit = Day1::find_last_digit(&line);
            let number = first_digit * 10 + last_digit;
            println!("{} -> {}", line, number);
            sum += number;
        }
    
        println!("Sum: {}", sum);
        println!("Finished");
        Ok(())
    }

    fn find_first_digit(str: &str) -> u32 {
        for i in 0..str.len() {
            match Day1::find_digit(str, i) {
                Some(d) => return d,
                None => continue
            }
        }
        0
    }

    fn find_last_digit(str: &str) -> u32 {
        for i in (0..str.len()).rev() {
            match Day1::find_digit(str, i) {
                Some(d) => return d,
                None => continue
            }
        }
        0
    }

    fn find_digit(s: &str, i: usize) -> Option<u32> {
        let c = s.chars().nth(i).unwrap();
        if c.is_digit(10) {
            return Some(c.to_digit(10).unwrap_or(0));
        } else if s[i..s.len()].starts_with("one") {
            return Some(1);
        } else if s[i..s.len()].starts_with("two") {
            return Some(2);
        } else if s[i..s.len()].starts_with("three") {
            return Some(3);
        } else if s[i..s.len()].starts_with("four") {
            return Some(4);
        } else if s[i..s.len()].starts_with("five") {
            return Some(5);
        } else if s[i..s.len()].starts_with("six") {
            return Some(6);
        } else if s[i..s.len()].starts_with("seven") {
            return Some(7);
        } else if s[i..s.len()].starts_with("eight") {
            return Some(8);
        } else if s[i..s.len()].starts_with("nine") {
            return Some(9);
        }
        None
    }
}