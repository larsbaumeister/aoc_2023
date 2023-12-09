use std::{error::Error, io::{BufRead, BufReader}, fs::File};


pub fn solve() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/day_9.txt")?);
    let mut next_value_sum = 0;
    let mut prev_value_sum = 0;
    for line in reader.lines() {
        let line = line?;
        let nums = line.as_str().split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let nextvalue = next_value(nums.clone());
        let prevvalue = prev_value(nums.clone());
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
    while !is_all_zeros(&diffs.last().unwrap()) {
        let diff = differences(&diffs.last().unwrap());
        diffs.push(diff);
    }
    let mut idx = diffs.len() - 1;
    diffs.get_mut(idx).unwrap().push(0);
    
    idx -= 1;
    loop {
        let last_item_in_this;
        {
            let this = diffs.get_mut(idx).unwrap();
            last_item_in_this = *this.last().unwrap();
        }
        let last_item_in_prev;        
        {
            let prev = diffs.get(idx + 1).unwrap();
            last_item_in_prev = *prev.last().unwrap();
        }
        let new_val = last_item_in_prev + last_item_in_this;
        {
            let this = diffs.get_mut(idx).unwrap();
            this.push(new_val);
        }
        if idx == 0 {
            break;
        }
        idx -= 1;
    }

    diffs.get(0).unwrap().last().unwrap().clone()
}

fn prev_value(mut nums: Vec<i64>) -> i64 {
    nums.reverse();
    next_value(nums)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_value() {
        let nums = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(next_value(nums), 18);

        let nums = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(next_value(nums), 68);
    }

    #[test]
    fn test_differences() {
        let nums = vec![1, 3, 6, 10, 15, 21, 28];
        assert_eq!(differences(&nums), vec![2, 3, 4, 5, 6, 7]);
    }
}