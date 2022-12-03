use super::Solver;
use itertools::Itertools;
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

fn get_result(op: RPS, pl: RPS) -> usize {
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
fn get_play_score(rps: RPS) -> usize {
    match rps {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}
fn get_result_match(rps: Vec<(RPS, RPS)>) -> usize {
    rps.into_iter()
        .map(|(op, pl)| get_result(op, pl) + get_play_score(pl))
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
        let opponent_rps =
            HashMap::from([('A', RPS::Rock), ('B', RPS::Paper), ('C', RPS::Scissors)]);
        let player_rps = HashMap::from([('X', RPS::Rock), ('Y', RPS::Paper), ('Z', RPS::Scissors)]);

        let rps: Vec<(RPS, RPS)> = input
            .into_iter()
            .map(|(op, pl)| {
                (
                    opponent_rps.get(op).unwrap().to_owned(),
                    player_rps.get(pl).unwrap().to_owned(),
                )
            })
            .collect();

        // let res: Vec<(usize, usize)> = rps
        //     .into_iter()
        //     .map(|(op, pl)| (getResult(op, pl), getPlayScore(pl)))
        //     .collect();
        // println!("{:?}", res);

        Ok(get_result_match(rps))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let opponent_rps =
            HashMap::from([('A', RPS::Rock), ('B', RPS::Paper), ('C', RPS::Scissors)]);

        let rps = input
            .into_iter()
            .map(|(op, res)| {
                let op_rps = opponent_rps.get(op).unwrap().to_owned();

                let pl_rps = match (op_rps, res) {
                    (RPS::Rock, 'X') => RPS::Scissors,
                    (RPS::Rock, 'Z') => RPS::Paper,
                    (RPS::Paper, 'X') => RPS::Rock,
                    (RPS::Paper, 'Z') => RPS::Scissors,
                    (RPS::Scissors, 'X') => RPS::Paper,
                    (RPS::Scissors, 'Z') => RPS::Rock,
                    _ => op_rps,
                };

                (op_rps, pl_rps)
            })
            .collect_vec();

        Ok(get_result_match(rps))
    }
}
