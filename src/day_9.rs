use std::{error::Error, io::{BufRead, BufReader}, fs::File};

use crate::Solution;

pub struct Day9 {}

impl Solution for Day9 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day9::solve()?,
            2 => Day9::solve()?,
            _ => println!("Invalid part number"),
        }
        Ok(())
    }
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {}
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(File::open("input/day_9.txt")?);
        let mut next_value_sum = 0;
        let mut prev_value_sum = 0;
        for line in reader.lines() {
            let line = line?;
            let nums = line.as_str().split_ascii_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let nextvalue = Day9::next_value(nums.clone());
            let prevvalue = Day9::prev_value(nums.clone());
            println!("{} <- {} -> {}", prevvalue, line, nextvalue);

            next_value_sum += nextvalue;
            prev_value_sum += prevvalue;
        }
        println!("Sum of next values: {}", next_value_sum);
        println!("Sum of prev values: {}", prev_value_sum);
        Ok(())
    }

    fn next_value(nums: Vec<i64>) -> i64 {
        let mut diffs = vec![nums];
        while !Day9::is_all_zeros(&diffs.last().unwrap()) {
            diffs.push(Day9::differences(&diffs.last().unwrap()));
        }

        diffs.last_mut().unwrap().push(0);
        
        for i in (0..diffs.len() - 1).rev() {
            let last_item_in_this = *diffs[i].last().unwrap();
            let last_item_in_prev = *diffs[i + 1].last().unwrap();
            let new_val = last_item_in_prev + last_item_in_this;
            diffs[i].push(new_val);
        }
        diffs.get(0).unwrap().last().unwrap().clone()
    }

    fn prev_value(mut nums: Vec<i64>) -> i64 {
        nums.reverse();
        Day9::next_value(nums)
    }

    fn is_all_zeros(nums: &Vec<i64>) -> bool {
        for num in nums {
            if *num != 0 {
                return false;
            }
        }
        true
    }

    fn differences(nums: &Vec<i64>) -> Vec<i64> {
        let mut diffs = Vec::new();
        for i in 0..nums.len() - 1 {
            diffs.push(nums[i + 1] - nums[i]);
        }
        diffs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_value() {
        let nums = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(Day9::next_value(nums), 18);

        let nums = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(Day9::next_value(nums), 68);
    }

    #[test]
    fn test_differences() {
        let nums = vec![1, 3, 6, 10, 15, 21, 28];
        assert_eq!(Day9::differences(&nums), vec![2, 3, 4, 5, 6, 7]);
    }
}