use core::num;
use std::{error::Error, fs::File, io::{BufReader, BufRead}};

use crate::Solution;


pub struct Day12 {}

impl Day12 {
    pub fn new() -> Day12 {
        Day12 {}
    }

    pub fn solve_part_1() -> Result<(), Box<dyn Error>> {
        let sum: u32 = Day12::parse_input()?.iter()
            .map(|i| Day12::possible_solutions(i))
            .sum::<u32>();

        println!("Sum: {}", sum);
        Ok(())
    }

    pub fn solve_part_2() -> Result<(), Box<dyn Error>> {
     
        Ok(())
    }

    fn parse_input() -> Result<Vec<(Vec<char>, Vec<u32>)>, Box<dyn Error>> {
        let reader = BufReader::new(File::open("input/day_12.txt")?);
        let mut lines: Vec<(Vec<char>, Vec<u32>)> = vec![];
        for line in reader.lines() {
            let line = line?;
            let res = line.split(" ").collect::<Vec<&str>>();
            let input = res[0].chars().collect::<Vec<char>>();
            let numbers = res[1].split(",").map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();
            lines.push((input, numbers));
        }
        Ok(lines)
    }

    fn possible_solutions(input: &(Vec<char>, Vec<u32>)) -> u32 {
        let mut sum = 0;
        let number_of_questoinmakrs = input.0.iter().filter(|c| **c == '?').count() as u32;
        // for every possible combination
        for i in 0..((2 as u64).pow(number_of_questoinmakrs)) {
            // build an input string
            let mut posible_solution = input.0.clone();
            let mut idx = 0;
            for char in posible_solution.iter_mut() {
                if *char == '?' {
                    *char = if (i >> idx) & 1 == 1 { '#' } else { '.' };
                    idx += 1;
                }
            }
            if Day12::is_valid_solution(&posible_solution, &input.1) {
                println!("{:?}", posible_solution);
                sum += 1;
            }
        }
        sum
    }

    fn is_valid_solution(input: &Vec<char>, numbers: &Vec<u32>) -> bool {
        let mut numbers = numbers.clone();
        let mut size_of_group = 0;
        for char in input.iter().rev() {
            match char {
                '#' => {
                    size_of_group += 1;
                },
                '.' => {
                    if size_of_group > 0 {
                        if numbers.is_empty() {
                            return false;
                        }
                        if size_of_group != numbers.pop().unwrap() {
                            return false;
                        }
                    }
                    size_of_group = 0;
                },
                _ => {}
            }
        
        }
        if size_of_group > 0 {
            if numbers.is_empty() {
                return false;
            }
            if size_of_group != numbers.pop().unwrap() {
                return false;
            }
        }
        numbers.is_empty()
    }
}

impl Solution for Day12 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day12::solve_part_1()?,
            2 => Day12::solve_part_2()?,
            _ => println!("Invalid part specified")
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_solution() {
        //let str = "?#.???#???????##?.?";
        let str = "##..#.#######.###.#";
        let vec = str.chars().collect::<Vec<char>>();
        let numbers = vec![2,1,7,3,1];

        assert_eq!(Day12::is_valid_solution(&vec, &numbers), true);
    }

    #[test]
    fn test_number_of_possible_solutions() {
        let str = "?###????????";
        let vec = str.chars().collect::<Vec<char>>();
        let numbers = vec![3,2,1];

        assert_eq!(Day12::possible_solutions(&(vec, numbers)), 10);
    }

    #[test]
    fn test_number_of_possible_solutions_1() {
        let str = "????.######..#####.";
        let vec = str.chars().collect::<Vec<char>>();
        let numbers = vec![1,6,5];

        assert_eq!(Day12::possible_solutions(&(vec, numbers)), 4);
    }
}