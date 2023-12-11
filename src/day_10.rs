use std::{error::Error, io::{BufReader, BufRead}, fs::File, char};

use crate::Solution;

pub struct Day10 {}

impl Solution for Day10 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day10::solve()?,
            2 => Day10::solve_v2()?,
            _ => println!("Invalid part specified")
        }
        Ok(())
    }
}

impl Day10 {
    pub fn new() -> Day10 {
        Day10 {}
    }

    fn solve() -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(File::open("input/day_10.txt")?);
        let matrix = Day10::parse_matrix(reader)?;
        let (x, y) = Day10::find_start_node(&matrix);
        println!("Start node: ({}, {})", x, y);
        let directions = Day10::possible_directions(&matrix, x, y);
        println!("Possible directions: {:?}", directions);

        let largest_path = Day10::find_largest_path(&matrix);
        println!("Largest path: {:?}", largest_path);
        println!("Largest path: {:?}", largest_path.len());

        Ok(())
    }

    fn solve_v2() -> Result<(), Box<dyn Error>> {
        let reader = BufReader::new(File::open("input/day_10.txt")?);
        let mut matrix = Day10::parse_matrix(reader)?;

        let start_x = 0;
        let start_y = 0;
        let old_char = '.';
        let new_char = 'I';
    
        Day10::flood_fill(&mut matrix, start_x, start_y, old_char, new_char);
    
        let tiles = Day10::count_enclosed_tiles(&matrix, old_char);
        println!("Tiles: {}", tiles);
        Ok(())
    }

    fn flood_fill(matrix: &mut Vec<Vec<char>>, x: usize, y: usize, old_char: char, new_char: char) {
        if x >= matrix.len() || y >= matrix[0].len() {
            return;
        }
    
        if matrix[x][y] != old_char {
            return;
        }
    
        matrix[x][y] = new_char;
    
        Day10::flood_fill(matrix, x + 1, y, old_char, new_char);
        Day10::flood_fill(matrix, x - 1, y, old_char, new_char);
        Day10::flood_fill(matrix, x, y + 1, old_char, new_char);
        Day10::flood_fill(matrix, x, y - 1, old_char, new_char);
    }
    
    fn count_enclosed_tiles(matrix: &Vec<Vec<char>>, enclosed_char: char) -> usize {
        matrix.iter().flatten().filter(|&&c| c == enclosed_char).count()
    }

    fn find_largest_path(matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
        let start_node = Day10::find_start_node(matrix);
        let mut largest_path: Vec<(usize, usize)> = Vec::new();
        let mut current_path: Vec<(usize, usize)> = Vec::new();
        Day10::find_path(matrix, &mut largest_path, &mut current_path, start_node, start_node);
        largest_path
    }

    fn find_path(
        matrix: &Vec<Vec<char>>,
        largest_path: &mut Vec<(usize, usize)>,
        current_path: &mut Vec<(usize, usize)>,
        current_node: (usize, usize),
        start_node: (usize, usize),
    ) {
        current_path.push(current_node);

        let directions = Day10::possible_directions(matrix, current_node.0, current_node.1);
        for direction in directions {
            if direction == start_node && current_path.len() > largest_path.len() {
                *largest_path = current_path.clone();
            } else if !current_path.contains(&direction) {
                Day10::find_path(matrix, largest_path, current_path, direction, start_node);
            }
        }

        current_path.pop();
    }

    fn parse_matrix(reader: BufReader<File>) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
        let mut matrix: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let vec: Vec<char> = line.chars().collect();
            matrix.push(vec);
        }
        Ok(matrix)
    }

    fn find_start_node(matrix: &Vec<Vec<char>>) -> (usize, usize) {
        for (x, row) in matrix.iter().enumerate() {
            for (y, col) in row.iter().enumerate() {
                if Day10::is_start_node(matrix, x, y) {
                    return (x, y);
                }
            }
        }
        panic!("No start node found")
    }

    fn is_start_node(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        matrix[x][y] == 'S'
    }

    fn possible_directions(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut directions: Vec<(usize, usize)> = Vec::new();
        if Day10::can_go_down(matrix, x, y) {
            directions.push((x + 1, y));
        }
        if Day10::can_go_up(matrix, x, y) {
            directions.push((x - 1, y));
        }
        if Day10::can_go_left(matrix, x, y) {
            directions.push((x, y - 1));
        }
        if Day10::can_go_right(matrix, x, y) {
            directions.push((x, y + 1));
        }
        directions
    }

    fn can_go_right(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if y > matrix[x].len() - 2 {
            return false;
        }

        let c = matrix[x][y];
        let nc = matrix[x][y + 1];
        (c == 'S' || c == '-' || c == 'L' || c == 'F') 
            && (nc == 'S' || nc == '-' || nc == 'J' || nc == '7')
    }

    fn can_go_left(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if y == 0 {
            return false;
        }
        let c = matrix[x][y];
        let nc = matrix[x][y - 1];
        (c == 'S' || c == '-' || c == 'J' || c == '7') 
            && (nc == 'S' || nc == '-' || nc == 'L' || nc == 'F')
    }

    fn can_go_up(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }
        let c = matrix[x][y];
        let nc = matrix[x - 1][y];
        (c == 'S' || c == '|' || c == 'L' || c == 'J') 
            && (nc == 'S' || nc == '|' || nc == 'F' || nc == '7')
    }

    fn can_go_down(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        if x > matrix.len() - 2 {
            return false;
        }
        let c = matrix[x][y];
        let nc = matrix[x + 1][y];
        (c == 'S' || c == '|' || c == '7' || c == 'F') 
            && (nc == 'S' || nc == '|' || nc == 'L' || nc == 'J')
    }
}