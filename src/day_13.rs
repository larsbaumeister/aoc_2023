use std::{error::Error, fs, collections::HashSet, path::Path};

use itertools::Itertools;

use crate::{Solution};

#[derive(Debug, PartialEq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

struct Map<'a> {
    string: &'a str,
    cols: usize,
    rows: usize,
}

impl<'a> Map<'a> {
    fn new(map_string: &'a str) -> Option<Self> {
        let height = map_string.lines().count();
        let width = map_string.lines().try_fold(None, |a: Option<usize>, l| {
            let this_char_count = l.chars().count();
            match a {
                Some(c) => {
                    if c != this_char_count {
                        None
                    } else {
                        Some(Some(c))
                    }
                }
                None => Some(Some(this_char_count)),
            }
        })??;
        Some(Self {
            string: map_string,
            cols: width,
            rows: height,
        })
    }

    fn coord(&self, row: usize, column: usize) -> Option<char> {
        if (row < self.rows) & (column < self.cols) {
            let index = row * (self.cols + 1) + column;
            self.string[index..index + 1].chars().next()
        } else {
            None
        }
    }

    fn find_reflection(&self, changed_amount: usize) -> Option<Reflection> {
        fn find_smudge_inner(
            changed_amount: usize,
            outer_limit: usize,
            inner_limit: usize,
            coord_fun: impl Fn(usize, usize) -> char,
        ) -> Option<usize> {
            (1..outer_limit).find_map(|outer| {
                if Map::pairs_iter(outer, outer_limit).try_fold(0, |acc, (o1, o2)| {
                    let new_acc = (0..inner_limit)
                        .filter(|inner| coord_fun(o1, *inner) != coord_fun(o2, *inner))
                        .count()
                        + acc;
                    if new_acc > changed_amount {
                        None
                    } else {
                        Some(new_acc)
                    }
                }) == Some(changed_amount)
                {
                    Some(outer)
                } else {
                    None
                }
            })
        }
        find_smudge_inner(changed_amount, self.rows, self.cols, |r, c| {
            self.coord(r, c).unwrap()
        })
        .map(|x| Reflection::Horizontal(x))
        .or_else(|| {
            find_smudge_inner(changed_amount, self.cols, self.rows, |c, r| {
                self.coord(r, c).unwrap()
            })
            .map(|x| Reflection::Vertical(x))
        })
    }

    /// Create an iterator over all pairs of lines starting at `inbetween`
    fn pairs_iter(inbetween: usize, limit: usize) -> impl Iterator<Item = (usize, usize)> {
        let max_offset = (limit - inbetween).min(inbetween);
        (0..max_offset).map(move |o| (inbetween - 1 - o, inbetween + o))
    }
}


pub struct Day13 {}

impl Day13 {
    pub fn new() -> Day13 {
        Day13 {}
    }

    fn evaluate_input(smudges: usize) -> Result<(), Box<dyn Error>> {
       let res: usize = fs::read_to_string(Path::new("input/day_13.txt"))?
            .split("\n\n")
            .map(|m| match Map::new(m).unwrap().find_reflection(smudges) {
                Some(Reflection::Vertical(col)) => col,
                Some(Reflection::Horizontal(row)) => 100 * row,
                None => unreachable!(),
            })
            .sum();
        println!("{}", res);
        Ok(())
    }

}

impl Solution for Day13 {
    fn solve(&self, part: u8) -> Result<(), Box<dyn Error>> {
        match part {
            1 => Day13::evaluate_input(0),
            2 => Day13::evaluate_input(1),
            _ => panic!("Unknown part: {}", part),
        }
    }
}
