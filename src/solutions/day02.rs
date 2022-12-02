use itertools::Itertools;
use lazy_static::__Deref;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn getResult(op: RPS, pl: RPS) -> usize {
    match (op, pl) {
        (RPS::Rock, RPS::Paper) => 6,
        (RPS::Rock, RPS::Scissors) => 0,
        (RPS::Paper, RPS::Rock) => 0,
        (RPS::Paper, RPS::Scissors) => 6,
        (RPS::Scissors, RPS::Paper) => 0,
        (RPS::Scissors, RPS::Rock) => 6,
        _ => 3,
    }
}
fn getPlayScore(rps: RPS) -> usize {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}
fn getResultMatch(rps: Vec<(RPS, RPS)>) -> usize {
    rps.into_iter()
        .map(|(op, pl)| getResult(op, pl) + getPlayScore(pl))
        .sum()
}

impl Solver for Problem {
    type Input = Vec<(char, char)>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let mut iter = line.split_whitespace().map(|x| x.chars().next().unwrap());
                return (iter.next().unwrap(), iter.next().unwrap());
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let opponentRPS =
            HashMap::from([('A', RPS::Rock), ('B', RPS::Paper), ('C', RPS::Scissors)]);
        let playerRPS = HashMap::from([('X', RPS::Rock), ('Y', RPS::Paper), ('Z', RPS::Scissors)]);

        let rps: Vec<(RPS, RPS)> = input
            .into_iter()
            .map(|(op, pl)| {
                (
                    opponentRPS.get(op).unwrap().to_owned(),
                    playerRPS.get(pl).unwrap().to_owned(),
                )
            })
            .collect();

        // let res: Vec<(usize, usize)> = rps
        //     .into_iter()
        //     .map(|(op, pl)| (getResult(op, pl), getPlayScore(pl)))
        //     .collect();
        // println!("{:?}", res);

        Ok(getResultMatch(rps))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let opponentRPS =
            HashMap::from([('A', RPS::Rock), ('B', RPS::Paper), ('C', RPS::Scissors)]);

        let rps = input
            .into_iter()
            .map(|(op, res)| {
                let opRPS = opponentRPS.get(op).unwrap().to_owned();

                let plRPS = match (opRPS, res) {
                    (RPS::Rock, 'X') => RPS::Scissors,
                    (RPS::Rock, 'Z') => RPS::Paper,
                    (RPS::Paper, 'X') => RPS::Rock,
                    (RPS::Paper, 'Z') => RPS::Scissors,
                    (RPS::Scissors, 'X') => RPS::Paper,
                    (RPS::Scissors, 'Z') => RPS::Rock,
                    _ => opRPS,
                };

                (opRPS, plRPS)
            })
            .collect_vec();

        Ok(getResultMatch(rps))
    }
}
