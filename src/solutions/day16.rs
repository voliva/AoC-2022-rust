use itertools::Itertools;
use regex::Regex;

use super::Solver;
use core::panic;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

pub struct Valve {
    name: String,
    rate: usize,
    edges: Vec<String>,
}

impl FromStr for Valve {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref LINE_RGX: Regex = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z ,]+)").unwrap();
        }

        match LINE_RGX.captures(s) {
            None => panic!("wrong line {s}"),
            Some(captures) => {
                let name = captures[1].to_owned();
                let rate: usize = captures[2].parse().unwrap();
                let edges = captures[3].split(", ").map(|x| x.to_owned()).collect_vec();

                Ok(Valve { name, rate, edges })
            }
        }
    }
}

#[derive(Clone)]
struct SearchState {
    position: String,
    opened: HashSet<String>,
    release: usize,
}

impl SearchState {
    fn new(position: &str) -> Self {
        Self {
            position: position.to_owned(),
            opened: HashSet::new(),
            release: 0,
        }
    }
}

impl Solver for Problem {
    type Input = HashMap<String, Valve>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .map(|x: Valve| (x.name.to_owned(), x))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut states: Vec<SearchState> = vec![SearchState::new("AA")];

        for m in 1..31 {
            println!("{m}");
            let mut new_states: Vec<SearchState> = vec![];

            for s in states {
                let current_node = input.get(&s.position).unwrap();

                if s.opened.len() == input.len() {
                    new_states.push(s);
                    continue;
                }

                if !s.opened.contains(&s.position) {
                    let mut new_state = s.clone();
                    new_state.opened.insert(s.position.clone());
                    new_state.release += (30 - m) * current_node.rate;
                    new_states.push(new_state);
                }

                for e in &current_node.edges {
                    let mut new_state = s.clone();
                    new_state.position = e.clone();
                    new_states.push(new_state);
                }
            }

            states = new_states;
        }

        todo!()
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}
