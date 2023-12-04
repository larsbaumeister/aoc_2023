use std::{error::Error, fs::File, io::BufReader, io::BufRead};

pub fn solve() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/day_4.txt")?);
    let mut total_win = 0;
    for line in reader.lines() {
        let line = line?;

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
            let res = base.pow((number_of_wins - 1) as u32);
            total_win += res;
        }
    }
    println!("Total win: {}", total_win);
    Ok(())
}