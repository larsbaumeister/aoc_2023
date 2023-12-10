use std::{error::Error, fs::File, io::Read};

use crate::Solution;

pub struct Day6 {}

impl Solution for Day6 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day6::solve()?,
            2 => Day6::solve()?,
            _ => println!("Invalid part number"),
        }
        Ok(())
    }
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {}
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        /* let races = parse_races()?; */
        let races = Day6::parse_race_v2()?;
        
        let mut possible_solutions: Vec<u64> = vec![];
        for race in races {
            possible_solutions.push(Day6::num_of_possible_solutions(race));
        }

        println!("Possible solutions: {:?}", possible_solutions);
        let product: u64 = possible_solutions.into_iter().product();
        println!("Product: {}", product);

        Ok(())
    }


    fn num_of_possible_solutions(race: Race) -> u64 {
        let mut wins = 0;
        for charging_time in 1..race.time {
            let distance_treveled = Day6::distance_traveld_for_charging_time(charging_time, race.time);
            if distance_treveled > race.distance {
                wins += 1;
            }
        }
        wins
    }

    fn distance_traveld_for_charging_time(charging_time: u64, race_duration: u64) -> u64 {
        let boat_speed = charging_time;
        let distance_traveled = boat_speed * (race_duration - charging_time);
        distance_traveled
    }

    fn parse_races() -> Result<Vec<Race>, Box<dyn Error>>{
        let mut file = File::open("input/day_6.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut lines = contents.lines();
        let times_line = lines.next().unwrap_or("");
        let distance_line = lines.next().unwrap_or("");

        let mut races: Vec<Race> = vec![];
        for (time, distance) in times_line.split_ascii_whitespace().zip(distance_line.split_ascii_whitespace()).skip(1) {
            races.push(Race {
                time: time.parse::<u64>()?,
                distance: distance.parse::<u64>()?,
            });
        }
        Ok(races)
    }


    fn parse_race_v2() -> Result<Vec<Race>, Box<dyn Error>> {
        let mut file = File::open("input/day_6.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut lines = contents.lines();
        let times_line = lines.next().unwrap_or("").replace(" ", "");
        let distance_line = lines.next().unwrap_or("").replace(" ", "");

        let x = times_line.split(":").collect::<Vec<&str>>()[1];
        let y = distance_line.split(":").collect::<Vec<&str>>()[1];
        Ok(vec![Race {
            time: x.parse::<u64>()?,
            distance: y.parse::<u64>()?
        }])
    }
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}
