use std::{error::Error, io::BufRead, io::BufReader, fs::File, cmp::{Ordering, min, max}};

#[derive(Debug, Clone, Copy)]
struct Point {
    i: usize,
    j: usize
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.i == other.i && self.j == other.j
    }
}

impl Eq for Point {}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.i.cmp(&other.i).then_with(|| self.j.cmp(&other.j))
    }
}

pub fn solve() -> Result<(), Box<dyn Error>> {
    let matrix = create_matrix()?;

    let mut number_positions: Vec<Point> = vec![];
    let mut sum_of_gear_rations = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            // this is for part 1
            if is_symbol(matrix[i][j]) {
                let res = find_adjesent_numbers(&matrix, Point { i: i, j: j });
                number_positions.extend(res);
            }

            // this is for part 2
            if matrix[i][j] == b'*' {
                let mut res = find_adjesent_numbers(&matrix, Point { i: i, j: j });
                res.sort();
                res.dedup();
                if res.len() == 2 {
                    let num1 = read_number(&matrix, res[0]);
                    let num2 = read_number(&matrix, res[1]);
                    sum_of_gear_rations += num1 * num2;
                }
            }
        }
    }

    number_positions.sort();
    number_positions.dedup();


    let res: u32 = number_positions.into_iter()
        .map(|p| read_number(&matrix, p))
        .sum();

    println!("Sum of part numbers: {:?}", res);
    println!("Sum of gear ratios: {:?}", sum_of_gear_rations);
    Ok(())
}

/// Finds all the numbers adjesent to the given point.
/// @param matrix The matrix to search in.
/// @param point The point to search around.
/// @return A vector of the starting points of the surrounding numbers.
fn find_adjesent_numbers(matrix: &Vec<Vec<u8>>, point: Point) -> Vec<Point> {
    let mut starting_points: Vec<Point> = vec![];

    if is_digit(matrix[max(point.i - 1, 0)][point.j]) {
        starting_points.push(find_starting_point_of_number(matrix, Point { i: point.i -1, j: point.j }));
    }

    if is_digit(matrix[min(point.i + 1, matrix.len())][point.j]) {
        starting_points.push(find_starting_point_of_number(matrix, Point { i: point.i +1, j: point.j }));
    }

    if is_digit(matrix[point.i][max(point.j - 1, 0)]) {
        starting_points.push(find_starting_point_of_number(matrix, Point { i: point.i, j: point.j -1 }));
    }
    
    if is_digit(matrix[point.i][min(point.j + 1, matrix[point.i].len())]) {
        starting_points.push(find_starting_point_of_number(matrix, Point { i: point.i, j: point.j +1 }));
    }

    if is_digit(matrix[max(point.i - 1, 0)][max(point.j - 1, 0)]) {
        starting_points.push(find_starting_point_of_number(matrix, Point { i: point.i -1, j: point.j -1 }));
    }

    if is_digit(matrix[max(point.i - 1, 0)][min(point.j + 1, matrix[point.i].len())]) {
        starting_points.push(find_starting_point_of_number(matrix, Point { i: point.i -1, j: point.j +1 }));
    }

    if is_digit(matrix[min(point.i + 1, matrix.len())][max(point.j - 1, 0)]) {
        starting_points.push(find_starting_point_of_number(matrix, Point { i: point.i +1, j: point.j -1 }));
    }

    if is_digit(matrix[min(point.i + 1, matrix.len())][min(point.j + 1, matrix[point.i].len())]) {
        starting_points.push(find_starting_point_of_number(matrix, Point { i: point.i +1, j: point.j +1 }));
    }

    starting_points
}

fn find_starting_point_of_number(matrix: &Vec<Vec<u8>>, point: Point) -> Point {
    let mut starting_point = point;
    while starting_point.j > 0 && is_digit(matrix[starting_point.i][starting_point.j -1]) {
        starting_point.j -= 1;
    }
    starting_point
}

fn is_symbol(char: u8) -> bool {
    char == b'#' || char == b'@' || char == b'~' || char == b'&' || char == b'%' || char == b'*' || char == b'=' || char == b'+' || char == b'-' || char == b'_' || char == b'/' || char == b'\\' || char == b'(' || char == b')' || char == b'[' || char == b']' || char == b'{' || char == b'}' || char == b'<' || char == b'>' || char == b'$'
}

fn is_digit(char: u8) -> bool {
    char >= b'0' && char <= b'9'
}

/// Reads a number from the matrix starting at the given point.
/// @param matrix The matrix to read from.
/// @param point The point to start reading from.
/// @return The number read.
fn read_number(matrix: &Vec<Vec<u8>>, point: Point) -> u32 {
    let mut number = 0;
    let i = point.i;
    let mut j = point.j;
    while j < matrix[i].len() && is_digit(matrix[i][j]) {
        number *= 10;
        // In Rust, b'0' is a byte literal that represents the ASCII value of the character '0',
        // which is 48. When you subtract b'0' from another byte that represents a digit 
        // character, you get the numerical value of that digit.
        number += (matrix[i][j] - b'0') as u32;
        j += 1;
    }
    number
}

/// Reads the whole input file into a matrix of bytes.
/// @return A matrix of bytes.
fn create_matrix() -> Result<Vec<Vec<u8>>, Box<dyn Error>>{
    let mut matrix: Vec<Vec<u8>> = vec![];
    let reader = BufReader::new(File::open("input/day_3.txt")?);
    for line in reader.lines() {
        let line = line?;
        matrix.push(line.as_bytes().to_vec());
    }
    Ok(matrix)
}