use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

fn has_duplicates(chr: &[char]) -> bool {
    let chr_set: HashSet<char> = HashSet::from_iter(chr.iter().cloned());
    // println!("{:?} {:?}, {}", chr, chr_set, chr_set.len() != chr.len());

    chr_set.len() != chr.len()
}

impl Solver for Problem {
    type Input = Vec<char>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .flat_map(|line| line.chars().collect_vec())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let input_clone = input.clone();

        let res = input
            .iter()
            .enumerate()
            .skip(4)
            .map(|(i, _)| has_duplicates(&input_clone[(i - 4)..i]))
            .tuple_windows::<(_, _, _, _)>()
            .find_position(|(a, b, c, d)| [a, b, c, d].iter().all(|x| !*x));

        match res {
            None => Err("Not found".to_owned()),
            Some((pos, _)) => Ok(pos + 4),
        }
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let input_clone = input.clone();

        let res = input
            .iter()
            .enumerate()
            .skip(14)
            .map(|(i, _)| has_duplicates(&input_clone[(i - 14)..i]))
            .tuple_windows::<(_, _, _, _)>()
            .find_position(|(a, b, c, d)| [a, b, c, d].iter().all(|x| !*x));

        match res {
            None => Err("Not found".to_owned()),
            Some((pos, _)) => Ok(pos + 14),
        }
    }
}
