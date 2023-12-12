use std::{error::Error, io::BufReader, fs};

use itertools::Itertools;

use crate::Solution;

pub struct Day11 {}

impl Solution for Day11 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day11::solve(2)?,
            2 => Day11::solve(1000000)?,
            _ => println!("Invalid part specified")
        }
        Ok(())
    }
}

impl Day11 {
    pub fn new() -> Day11 {
        Day11 {}
    }

    fn solve(universe_scale: u64) -> Result<(), Box<dyn Error>> {
        let file_content = fs::read_to_string("input/day_11.txt")?;
        let matrix = file_content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        fn is_row_blank(matrix: &Vec<Vec<char>>, row: usize) -> bool {
            for col in 0..matrix[row].len() {
                if matrix[row][col] == '#' {
                    return false;
                }
            }
            true
        }

        fn is_col_blank(matrix: &Vec<Vec<char>>, col: usize) -> bool {
            for row in 0..matrix.len() {
                if matrix[row][col] == '#' {
                    return false;
                }
            }
            true
        }

        fn get_number_of_blank_rows(matrix: &Vec<Vec<char>>, row: usize) -> usize {
            let mut blank_rows = 0;
            for row in 0..row {
                if is_row_blank(&matrix, row) {
                    blank_rows += 1;
                }
            }
            blank_rows
        }

        fn get_number_of_blank_cols(matrix: &Vec<Vec<char>>, col: usize) -> usize {
            let mut blank_cols = 0;
            for col in 0..col {
                if is_col_blank(&matrix, col) {
                    blank_cols += 1;
                }
            }
            blank_cols
        }

        let mut nodes_to_visit: Vec<(u64, u64)> = vec![];
        for (y, row) in matrix.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == '#' {
                    let offset_y = get_number_of_blank_rows(&matrix, y) as u64 * (universe_scale - 1);
                    let offset_x = get_number_of_blank_cols(&matrix, x) as u64 * (universe_scale - 1);
                    nodes_to_visit.push((y as u64 + offset_y, x as u64 + offset_x));
                }
            }
        }

        let sum = nodes_to_visit.into_iter()
            .combinations(2)
            .map(|pair| {
                let (y1, x1) = pair[0];
                let (y2, x2) = pair[1];
                (y1 as i64 - y2 as i64).abs() + (x1 as i64 - x2 as i64).abs()
            })
            .sum::<i64>();

        println!("Sum: {}", sum);
        Ok(())
    }

    fn solve_v2() -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}