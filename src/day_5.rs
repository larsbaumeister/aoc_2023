use std::{error::Error, io::{BufReader, BufRead}, fs::File};

use crate::Solution;

pub struct Day5 {}

impl Solution for Day5 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day5::solve()?,
            2 => Day5::solve()?,
            _ => println!("Invalid part number"),
        }
        Ok(())
    }
}

impl Day5 {
    
    pub fn new() -> Day5 {
        Day5 {}
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let (
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map) = Day5::read_input_file()?;


        let mut min_location = u64::MAX;
        // this is for part 1
    /*     for seed in seeds {
            let soil = perform_mapping(seed, &seed_to_soil_map);
            let fertilizer = perform_mapping(soil, &soil_to_fertilizer_map);
            let water = perform_mapping(fertilizer, &fertilizer_to_water_map);
            let light = perform_mapping(water, &water_to_light_map);
            let temperature = perform_mapping(light, &light_to_temperature_map);
            let humidity = perform_mapping(temperature, &temperature_to_humidity_map);
            let location = perform_mapping(humidity, &humidity_to_location_map);

            if location < min_location {
                min_location = location;
            }
            println!("Seed: {}, Location: {}", seed, location);
        } */


        // this is for part 2
        for i in (0..seeds.len()).step_by(2) {
            let start = seeds[i];
            let size = seeds[i + 1];
            for seed in start..start + size {
                let soil = Day5::perform_mapping(seed, &seed_to_soil_map);
                let fertilizer = Day5::perform_mapping(soil, &soil_to_fertilizer_map);
                let water = Day5::perform_mapping(fertilizer, &fertilizer_to_water_map);
                let light = Day5::perform_mapping(water, &water_to_light_map);
                let temperature = Day5::perform_mapping(light, &light_to_temperature_map);
                let humidity = Day5::perform_mapping(temperature, &temperature_to_humidity_map);
                let location = Day5::perform_mapping(humidity, &humidity_to_location_map);

                if location < min_location {
                    min_location = location;
                }
            }
        }

        println!("Min location: {}", min_location);

        Ok(())
    }

    fn perform_mapping(value: u64, map: &Vec<MapEntry>) -> u64 {
        for map_entry in map {
            if map_entry.source_start <= value && value < map_entry.source_start + map_entry.length {
                return map_entry.dest_start + (value - map_entry.source_start);
            }
        }
        value
    }

    fn read_input_file() -> Result<(Vec<u64>, Vec<MapEntry>, Vec<MapEntry>, Vec<MapEntry>, Vec<MapEntry>, Vec<MapEntry>, Vec<MapEntry>, Vec<MapEntry>), Box<dyn Error>> {
        let reader = BufReader::new(File::open("input/day_5.txt")?);
        let mut is_in_seed_to_soil_section = false;
        let mut is_in_soil_to_feritilizer_section = false;
        let mut is_in_fertilizer_to_water_section = false;
        let mut is_in_water_to_light_section = false;
        let mut is_in_light_to_temperature_section = false;
        let mut is_in_temperature_to_humidity_section = false;
        let mut is_in_humidity_to_location_section = false;

        let mut seeds: Vec<u64> = vec![];
        let mut seed_to_soil_map: Vec<MapEntry> = vec![];
        let mut soil_to_fertilizer_map: Vec<MapEntry> = vec![];
        let mut fertilizer_to_water_map: Vec<MapEntry> = vec![];
        let mut water_to_light_map: Vec<MapEntry> = vec![];
        let mut light_to_temperature_map: Vec<MapEntry> = vec![];
        let mut temperature_to_humidity_map: Vec<MapEntry> = vec![];
        let mut humidity_to_location_map: Vec<MapEntry> = vec![];

        for line in reader.lines() {
            let line = line?;

            if line.starts_with("seeds:") {
                let parsed_seeds = Day5::parse_seeds(&line);
                seeds.extend(parsed_seeds);
                continue;
            }
            if line.starts_with("seed-to-soil map:") {
                is_in_seed_to_soil_section = true;
                continue;
            }
            if line.starts_with("soil-to-fertilizer map:") {
                is_in_soil_to_feritilizer_section = true;
                continue;
            }
            if line.starts_with("fertilizer-to-water map:") {
                is_in_fertilizer_to_water_section = true;
                continue;
            }
            if line.starts_with("water-to-light map:") {
                is_in_water_to_light_section = true;
                continue;
            }
            if line.starts_with("light-to-temperature map:") {
                is_in_light_to_temperature_section = true;
                continue;
            }
            if line.starts_with("temperature-to-humidity map:") {
                is_in_temperature_to_humidity_section = true;
                continue;
            }

            if line.starts_with("humidity-to-location map:") {
                is_in_humidity_to_location_section = true;
                continue;
            }

            if line.is_empty() {
                is_in_seed_to_soil_section = false;
                is_in_soil_to_feritilizer_section = false;
                is_in_fertilizer_to_water_section = false;
                is_in_water_to_light_section = false;
                is_in_light_to_temperature_section = false;
                is_in_temperature_to_humidity_section = false;
                is_in_humidity_to_location_section = false;
                continue;
            }

            if is_in_seed_to_soil_section {
                let map_entry = Day5::parse_map_entry(&line);
                seed_to_soil_map.push(map_entry);
                continue;
            }

            if is_in_soil_to_feritilizer_section {
                let map_entry = Day5::parse_map_entry(&line);
                soil_to_fertilizer_map.push(map_entry);
                continue;
            }

            if is_in_fertilizer_to_water_section {
                let map_entry = Day5::parse_map_entry(&line);
                fertilizer_to_water_map.push(map_entry);
                continue;
            }

            if is_in_water_to_light_section {
                let map_entry = Day5::parse_map_entry(&line);
                water_to_light_map.push(map_entry);
                continue;
            }

            if is_in_light_to_temperature_section {
                let map_entry = Day5::parse_map_entry(&line);
                light_to_temperature_map.push(map_entry);
                continue;
            }

            if is_in_temperature_to_humidity_section {
                let map_entry = Day5::parse_map_entry(&line);
                temperature_to_humidity_map.push(map_entry);
                continue;
            }

            if is_in_humidity_to_location_section {
                let map_entry = Day5::parse_map_entry(&line);
                humidity_to_location_map.push(map_entry);
                continue;
            }

        }


        Ok((
            seeds,
            seed_to_soil_map,
            soil_to_fertilizer_map,
            fertilizer_to_water_map,
            water_to_light_map,
            light_to_temperature_map,
            temperature_to_humidity_map,
            humidity_to_location_map))
    }

    fn parse_seeds(line: &str) -> Vec<u64> {
        let mut parts = line[7..line.len()].split_ascii_whitespace();
        let mut seeds = vec![];
        while let Some(part) = parts.next() {
            seeds.push(part.parse::<u64>().unwrap());
        }
        seeds
    }

    fn parse_map_entry(line: &str) -> MapEntry {
        let mut parts = line.split_ascii_whitespace();
        let dest_start = parts.next().unwrap().parse::<u64>().unwrap();
        let source_start = parts.next().unwrap().parse::<u64>().unwrap();
        let length = parts.next().unwrap().parse::<u64>().unwrap();
        MapEntry {
            source_start,
            dest_start,
            length
        }
    }
}

#[derive(Debug)]
struct MapEntry {
    source_start: u64,
    dest_start: u64,
    length: u64
}
