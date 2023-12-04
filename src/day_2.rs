use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};

use regex::Regex;

pub fn solve() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open("input/day_2.txt")?);

    let green_regex = Regex::new(r"(\d+) green").unwrap();
    let blue_regex = Regex::new(r"(\d+) blue").unwrap();
    let red_regex = Regex::new(r"(\d+) red").unwrap();
    let gameid_regex = Regex::new(r"Game (\d+)").unwrap();

    let green_threshold = 13;
    let blue_threshold = 14;
    let red_threshold = 12;

    let mut possible_game_ids: Vec<u32> = vec![];
    let mut sum_of_powers = 0;
    for line in reader.lines() {
        let line = line?;
        let max_green = green_regex.captures_iter(&line)
            .filter_map(|c| c.get(1).unwrap().as_str().parse::<u32>().ok())
            .max().unwrap_or(0);

        let max_blue = blue_regex.captures_iter(&line)
            .filter_map(|c| c.get(1).unwrap().as_str().parse::<u32>().ok())
            .max().unwrap_or(0);

        let max_red = red_regex.captures_iter(&line)
            .filter_map(|c| c.get(1).unwrap().as_str().parse::<u32>().ok())
            .max().unwrap_or(0);

        if max_green <= green_threshold && max_blue <= blue_threshold && max_red <= red_threshold {
            let gameid = gameid_regex.captures(&line).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
            possible_game_ids.push(gameid);
        }

        let power = max_green * max_blue * max_red;
        sum_of_powers += power;
    }

    let sum: u32 = possible_game_ids.iter().sum();
    println!("Sum of possible game ids: {}", sum);
    println!("Sum of powers: {}", sum_of_powers);

    Ok(())
}

