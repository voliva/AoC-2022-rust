use itertools::Itertools;
use ndarray::Array2;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Array2<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.bytes().map(|c| c - b'0').collect_vec())
            .collect_vec();

        ndarray::Array2::from_shape_fn((lines.len(), lines[0].len()), |(r, c)| lines[r][c])
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let shape = input.shape();
        let mut visible_set: HashSet<(usize, usize)> = HashSet::new();

        for x in 1..(shape[0] - 1) {
            let mut max = input[[x, 0]];
            for y in 1..(shape[1] - 1) {
                if max == 9 {
                    break;
                }
                let v = input[[x, y]];
                if v > max {
                    visible_set.insert((x, y));
                    max = v;
                }
            }

            max = input[[x, shape[1] - 1]];
            // print!("{max}");
            for y in (1..(shape[1] - 1)).rev() {
                if max == 9 {
                    break;
                }
                let v = input[[x, y]];
                if v > max {
                    visible_set.insert((x, y));
                    max = v;
                }
            }
        }

        for y in 1..(shape[1] - 1) {
            let mut max = input[[0, y]];
            for x in 1..(shape[0] - 1) {
                if max == 9 {
                    break;
                }
                let v = input[[x, y]];
                if v > max {
                    visible_set.insert((x, y));
                    max = v;
                }
            }

            max = input[[shape[0] - 1, y]];
            for x in (1..(shape[0] - 1)).rev() {
                if max == 9 {
                    break;
                }
                let v = input[[x, y]];
                if v > max {
                    visible_set.insert((x, y));
                    max = v;
                }
            }
        }

        // println!(
        //     "{:?}, {:?}",
        //     visible_set
        //         .iter()
        //         .map(|(x, y)| (x, y, input[[*x, *y]]))
        //         .collect_vec(),
        //     shape
        // );

        // 1808 1814
        Ok(visible_set.len() + 2 * (shape[0] - 2) + 2 * (shape[1] - 2) + 4)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let shape = input.shape();

        let r = input
            .indexed_iter()
            .map(|((x, y), v)| {
                let mut up = 0;
                for i in (0..y).rev() {
                    if input[[x, i]] <= *v {
                        up = up + 1;
                    }
                    if input[[x, i]] >= *v {
                        break;
                    }
                }

                let mut down = 0;
                for i in (y + 1)..shape[1] {
                    if input[[x, i]] <= *v {
                        down = down + 1;
                    }
                    if input[[x, i]] >= *v {
                        break;
                    }
                }

                let mut left = 0;
                for i in (0..x).rev() {
                    if input[[i, y]] <= *v {
                        left = left + 1;
                    }
                    if input[[i, y]] >= *v {
                        break;
                    }
                }

                let mut right = 0;
                for i in (x + 1)..shape[0] {
                    if input[[i, y]] <= *v {
                        right = right + 1;
                    }
                    if input[[i, y]] >= *v {
                        break;
                    }
                }

                up * down * left * right
            })
            .max()
            .unwrap();

        Ok(r)
    }
}
