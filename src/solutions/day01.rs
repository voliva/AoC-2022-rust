use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

struct Acc {
    current: usize,
    max: usize,
}
struct Acc2 {
    current: usize,
    values: Vec<usize>,
}

impl Solver for Problem {
    type Input = Vec<String>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader.lines().map(|x| x.unwrap()).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let res = input.iter().fold(Acc { current: 0, max: 0 }, |acc, x| {
            if x == "" {
                Acc {
                    current: 0,
                    max: acc.max.max(acc.current),
                }
            } else {
                Acc {
                    current: acc.current + x.parse::<usize>().unwrap(),
                    max: acc.max,
                }
            }
        });

        Ok(res.max.max(res.current))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut res = input.iter().fold(
            Acc2 {
                current: 0,
                values: Vec::new(),
            },
            |mut acc, x| {
                if x == "" {
                    acc.values.push(acc.current);
                    Acc2 {
                        current: 0,
                        values: acc.values,
                    }
                } else {
                    Acc2 {
                        current: acc.current + x.parse::<usize>().unwrap(),
                        values: acc.values,
                    }
                }
            },
        );

        res.values.push(res.current);
        res.values.sort();
        res.values.reverse();

        let res2 = &res.values[..3];

        Ok(res2.iter().sum())
    }
}
