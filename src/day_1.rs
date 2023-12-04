use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/day_1.txt")?);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        let first_digit = find_first_digit(&line);
        let last_digit = find_last_digit(&line);
        let number = first_digit * 10 + last_digit;
        println!("{} -> {}", line, number);
        sum += number;
    }

    println!("Sum: {}", sum);
    println!("Finished");
    Ok(())
}

pub fn find_first_digit(str: &str) -> u32 {
    for i in 0..str.len() {
        match find_digit(str, i) {
            Some(d) => return d,
            None => continue
        }
    }
    0
}

pub fn find_last_digit(str: &str) -> u32 {
    for i in (0..str.len()).rev() {
        match find_digit(str, i) {
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