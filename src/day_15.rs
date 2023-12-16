use std::{error::Error, path::Path, fs, collections::HashMap};

use itertools::Itertools;
use regex::Regex;

use crate::Solution;


pub struct Day15 {}

impl Day15 {
    pub fn new() -> Day15 {
        Day15 {}
    }

    pub fn solve() -> Result<(), Box<dyn Error>> {
        let str = fs::read_to_string(Path::new("input/day_15.txt"))?;
        let sum: u32 = str.split(",")
            .map(Day15::hash)
            .sum();

        println!("{}", sum);
        Ok(())
    }

    pub fn solve_v2() -> Result<(), Box<dyn Error>> {

        struct Lense {
            label: String,
            focal_length: u32,
        }

        let mut lenses: HashMap<u8, Vec<Lense>> = HashMap::new();
        let str = fs::read_to_string(Path::new("input/day_15.txt"))?;
        let label_extractor = Regex::new(r"\w*")?;
        for part in str.split(",") {
            let label = label_extractor.find(part).unwrap().as_str();
            let label_hash = Day15::hash(label) as u8;
            let operation = &part[label.len()..label.len()+1];
            let mut num = None;
            if operation == "=" {
                num = Some(&part[label.len()+1..]);
            }

            match operation {
                "=" => {
                    let num = num.unwrap().parse::<u32>()?;
                    let vec = lenses.entry(label_hash).or_insert(vec![]);
                    let found_lens = vec.into_iter().find(|l| l.label == label);
                    if let Some(lens) = found_lens {
                        lens.focal_length = num;
                    } else {
                        vec.push(Lense {
                            label: label.to_string(),
                            focal_length: num,
                        });
                    }
                },
                "-" => {
                    if lenses.contains_key(&label_hash) {
                        let vec = lenses.entry(label_hash).or_insert(vec![]);
                        let x = vec.into_iter()
                            .find_position(|l| l.label == label)
                            .map(|(i, _)| i);
                        if let Some(i) = x {
                            vec.remove(i);
                        }
                    }
                },
                _ => {
                    println!("{}", operation);
                    return Err("Invalid operation".into());
                }
            }
        }

        let mut sum = 0;
        for (i, vec) in lenses {
            for (j, lense) in vec.into_iter().enumerate() {
                sum += lense.focal_length * (i as u32 + 1) * (j as u32 + 1);
            }
        }

        println!("{}", sum);

        Ok(())
    }

    fn hash(input: &str) -> u32 {
        let mut hash = 0;
        for c in input.chars() {
            hash += c as u32;
            hash *= 17;
            hash %= 256;
        }
        hash
    }
}

impl Solution for Day15 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day15::solve(),
            2 => Day15::solve_v2(),
            _ => panic!("Illegal part number"),
        }
    }
}