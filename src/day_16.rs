use std::{error::Error, fs, path::Path, collections::HashSet};

use crate::Solution;

#[derive(PartialEq, Clone, Copy, Debug, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Day16 {}

impl Day16 {
    pub fn new () -> Day16 {
        Day16 {}
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let matrix: Vec<Vec<char>> = fs::read_to_string(Path::new("input/day_16.txt"))?
            .lines()
            .map(|l| l.chars().collect())
            .collect();

        let mut visited_nodes: HashSet<(i64, i64)> = HashSet::with_capacity(matrix.len() * matrix[0].len());
        let mut visited_nodes_with_dir: HashSet<(i64, i64, Direction)> = HashSet::with_capacity(matrix.len() * matrix[0].len() * 4);
        Day16::trace_ray(&matrix, Direction::Right, (0, 0), &mut visited_nodes, &mut visited_nodes_with_dir);
        println!("Visited nodes: {}", visited_nodes.len());
        Ok(())
    }

    pub fn solve_v2() -> Result<(), Box<dyn Error>> {
        let matrix: Vec<Vec<char>> = fs::read_to_string(Path::new("input/day_16.txt"))?
            .lines()
            .map(|l| l.chars().collect())
            .collect();

        let mut start_positions: Vec<(i64, i64, Direction)> = vec![];
        for i in 0..matrix.len() {
            start_positions.push((i as i64, 0, Direction::Right));
            start_positions.push((i as i64, (matrix[0].len() - 1) as i64, Direction::Left));
        }

        for i in 0..matrix[0].len() {
            start_positions.push((0, i as i64, Direction::Down));
            start_positions.push(((matrix.len() - 1) as i64, i as i64, Direction::Up));
        }

        let mut max_visited_nodes = 0;
        for (start_x, start_y, dir) in start_positions {
            let mut visited_nodes: HashSet<(i64, i64)> = HashSet::with_capacity(matrix.len() * matrix[0].len());
            let mut visited_nodes_with_dir: HashSet<(i64, i64, Direction)> = HashSet::with_capacity(matrix.len() * matrix[0].len() * 4);
            Day16::trace_ray(&matrix, dir, (start_x, start_y), &mut visited_nodes, &mut visited_nodes_with_dir);
            println!("Start pos: {:?}, visited nodes: {}", (start_x, start_y), visited_nodes.len());
            if visited_nodes.len() > max_visited_nodes {
                max_visited_nodes = visited_nodes.len();
            }
        }
        println!("Max visited nodes: {}", max_visited_nodes);

        Ok(())
    }

    fn transform_pos(pos: (i64, i64), dir: Direction) -> (i64, i64) {
        match dir {
            Direction::Up => (pos.0 - 1, pos.1),
            Direction::Down => (pos.0 + 1, pos.1),
            Direction::Left => (pos.0, pos.1 - 1),
            Direction::Right => (pos.0, pos.1 + 1),
        }
    }

    fn pos_is_in_matrix(pos: (i64, i64), matrix: &Vec<Vec<char>>) -> bool {
           pos.0 < matrix.len() as i64 
        && pos.0 >= 0
        && pos.1 < matrix[0].len() as i64
        && pos.1 >= 0
    }

    fn trace_ray(matrix: &Vec<Vec<char>>, direction: Direction, pos: (i64, i64), visited_nodes: &mut HashSet<(i64, i64)>, visited_nodes_with_dir: &mut HashSet<(i64, i64, Direction)>) {
        let mut pos = pos;
        let mut direction = direction;
        loop {
            visited_nodes.insert(pos);
            if !visited_nodes_with_dir.insert((pos.0, pos.1, direction)) {
                // path already visited
                break;
            }
            let char = matrix[pos.0 as usize][pos.1 as usize];
            let next_directions: Vec<Direction> = match char {
                '\\' => match direction {
                    Direction::Up => vec![Direction::Left],
                    Direction::Down => vec![Direction::Right],
                    Direction::Left => vec![Direction::Up],
                    Direction::Right => vec![Direction::Down],
                },                
                '/' => match direction {
                    Direction::Up => vec![Direction::Right],
                    Direction::Down => vec![Direction::Left],
                    Direction::Left => vec![Direction::Down],
                    Direction::Right => vec![Direction::Up],
                },
                '.' => vec![direction],
                '|' => if direction == Direction::Left || direction == Direction::Right {
                    vec![Direction::Up, Direction::Down]
                } else {
                    vec![direction]
                },
                '-' => if direction == Direction::Up || direction == Direction::Down {
                    vec![Direction::Left, Direction::Right]
                } else {
                    vec![direction]
                },
                _ => panic!("Invalid char"),
            };
            match next_directions.len() {
                1 => {
                    let next_pos = Day16::transform_pos(pos, next_directions[0]);
                    if !Day16::pos_is_in_matrix(next_pos, matrix) {
                        break;
                    }
                    pos = next_pos;
                    direction = next_directions[0];
                },
                2 => {
                    for dir in next_directions {
                        let new_pos = Day16::transform_pos(pos, dir);
                        if !Day16::pos_is_in_matrix(new_pos, matrix) {
                            continue;
                        }
                        Day16::trace_ray(matrix, dir, new_pos, visited_nodes, visited_nodes_with_dir);
                    }
                    break;
                },
                _ => panic!("Invalid number of directions"),
            }
        }
    }
}

impl Solution for Day16 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day16::solve(),
            2 => Day16::solve_v2(),
            _ => panic!("Invalid part number"),
        }
    }
}