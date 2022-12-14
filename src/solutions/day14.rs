use itertools::Itertools;
use ndarray::{Array, Array2};

use super::Solver;
use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

pub struct RockPath {
    path: Vec<(usize, usize)>,
}

impl FromStr for RockPath {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let path = s
            .split(" -> ")
            .map(|substr| {
                let coords: (usize, usize) = substr
                    .split(",")
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                coords
            })
            .collect_vec();

        Ok(RockPath { path })
    }
}

#[derive(Clone, PartialEq)]
enum CaveElement {
    Air,
    Rock,
    Sand,
}

fn put_rocks(
    dest: &mut Array2<CaveElement>,
    from: (usize, usize),
    to: (usize, usize),
    x_offset: usize,
) {
    if from.0 == to.0 {
        let min = from.1.min(to.1);
        let max = from.1.max(to.1);
        let x = from.0 + x_offset;
        for y in min..(max + 1) {
            dest[[y, x]] = CaveElement::Rock
        }
    } else if from.1 == to.1 {
        let min = from.0.min(to.0) + x_offset;
        let max = from.0.max(to.0) + x_offset;
        let y = from.1;
        for x in min..(max + 1) {
            dest[[y, x]] = CaveElement::Rock
        }
    } else {
        panic!("diagonal line?! {:?} {:?}", from, to);
    }
}

fn array_from_path(rocks: &Vec<RockPath>) -> (Array2<CaveElement>, usize) {
    let max_y = rocks
        .iter()
        .map(|x| x.path.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap();
    let min_x = rocks
        .iter()
        .map(|r| r.path.iter().map(|(x, _)| x).min().unwrap())
        .min()
        .unwrap();
    let max_x = rocks
        .iter()
        .map(|r| r.path.iter().map(|(x, _)| x).max().unwrap())
        .max()
        .unwrap();
    let extra_x = 500;
    let start_point = 500 - min_x + extra_x;

    let mut res: Array2<CaveElement> = Array2::from_elem(
        (max_y + 1 + 2, max_x - min_x + extra_x * 2),
        CaveElement::Air,
    );

    for r in rocks {
        for (from, to) in (0..r.path.len()).tuple_windows() {
            put_rocks(&mut res, r.path[from], r.path[to], extra_x - min_x);
        }
    }

    (res, start_point)
}

fn put_sand(dest: &mut Array2<CaveElement>, start: usize) -> bool {
    let shape = dest.shape().to_owned();
    let mut x = start;
    let mut y = 0;
    if dest[[y, x]] != CaveElement::Air {
        return false;
    }

    while y < shape[0] - 2 {
        match dest[[y + 1, x]] {
            CaveElement::Air => {
                y += 1;
            }
            _ => {
                if dest[[y + 1, x - 1]] == CaveElement::Air {
                    y += 1;
                    x -= 1;
                } else if dest[[y + 1, x + 1]] == CaveElement::Air {
                    y += 1;
                    x += 1;
                } else {
                    dest[[y, x]] = CaveElement::Sand;
                    return true;
                }
            }
        }
    }
    dest[[y, x]] = CaveElement::Sand;

    return true;
}

impl Solver for Problem {
    type Input = Vec<RockPath>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let (mut cave, start) = array_from_path(input);

        let mut units = 0;
        while put_sand(&mut cave, start) {
            units += 1;
        }

        // 22807
        Ok(units)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}
