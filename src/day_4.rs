use std::{error::Error, fs::File, io::BufReader, io::BufRead, cmp::min};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/day_4.txt")?);
    let mut total_win = 0;

    let mut lines: Vec<String> = vec![];
    for line in reader.lines() {
        let line = line?;
        let (_, points) = calc_line(&line);
        total_win += points;
        lines.push(line);
    }

    let (wins, _) = version_2(&lines, 0, lines.len());

    println!("Total win: {}", total_win);
    println!("Total wins: {}", wins + lines.len() as u32);
    Ok(())
}

/// Calculates the number of wins and the points given line.
/// @param line The line to calculate.
/// @return A tuple containing the number of wins and the points.
fn calc_line(line: &str) -> (u32, u32) {
    let winning_numbers_str = line.chars().skip(10).take(29).collect::<String>();
    let actual_numbers_str = line.chars().skip(42).take(74).collect::<String>();

    let winning_numbers = winning_numbers_str.split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let actual_numbers = actual_numbers_str.split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let number_of_wins = actual_numbers.into_iter()
        .filter(|n| winning_numbers.contains(n))
        .count();

    if number_of_wins > 0 {
        let base: u32 = 2;
        return (number_of_wins as u32, base.pow((number_of_wins - 1) as u32));
    }
    (0, 0)
}


fn version_2(lines: &Vec<String>, start: usize, end: usize) -> (u32, u32) {
    let mut total_wins = 0;
    let mut total_points = 0;
    for i in start..end {
        let (number_of_wins, points) = calc_line(&lines[i]);
        total_wins += number_of_wins;
        total_points += points;

        let (number_of_wins, _) = version_2(&lines, i + 1, min(i + 1 + number_of_wins as usize, lines.len()));
        total_wins += number_of_wins;
    }
    (total_wins, total_points)

}