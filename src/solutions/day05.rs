use itertools::Itertools;
use regex::Regex;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub struct Problem;

impl Solver for Problem {
    type Input = (Crates, Vec<Operation>);
    type Output1 = String;
    type Output2 = String;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut lines = file_reader.lines().map(|x| x.unwrap());

        let stack_lines = lines.by_ref().take_while(|x| x.contains("[")).collect_vec();
        let reversed_lines = stack_lines.into_iter().rev().collect_vec();
        let mut crates = Crates::new(&reversed_lines[0]);
        reversed_lines
            .into_iter()
            .skip(1)
            .for_each(|line| crates.push_line(&line));

        let operations: Vec<Operation> = lines
            .skip_while(|x| !x.contains("move"))
            .map(|x| x.parse().unwrap())
            .collect_vec();

        (crates, operations)
    }

    fn solve_first(&self, (crates, operations): &Self::Input) -> Result<Self::Output1, String> {
        let mut crates_clone = crates.clone();

        operations.iter().for_each(|operation| {
            crates_clone.apply(operation);
        });

        Ok(crates_clone.get_top())
    }

    fn solve_second(&self, (crates, operations): &Self::Input) -> Result<Self::Output2, String> {
        let mut crates_clone = crates.clone();

        operations.iter().for_each(|operation| {
            crates_clone.apply_batch(operation);
        });

        Ok(crates_clone.get_top())
    }
}

#[derive(Debug, Clone)]
pub struct Crates {
    stacks: Vec<Vec<String>>,
}

impl Crates {
    pub fn new(base: &str) -> Self {
        let crates = base.split(" ").map(|x| x[1..2].to_owned()).collect_vec();
        let stacks = crates.into_iter().map(|x| Vec::from([x])).collect_vec();
        Crates { stacks: stacks }
    }
    pub fn push_line(&mut self, line: &str) {
        for i in 0..self.stacks.len() {
            let crate_name = &line[(1 + i * 4)..(1 + i * 4 + 1)];
            if crate_name != " " {
                self.stacks[i].push(crate_name.to_owned())
            }
        }
    }
    pub fn apply(&mut self, operation: &Operation) {
        // println!("{:?} {:?}", self, operation);
        for _ in 0..operation.quantity {
            let top_crate = self.stacks[operation.from].pop().unwrap();
            self.stacks[operation.to].push(top_crate);
        }
    }
    pub fn apply_batch(&mut self, operation: &Operation) {
        let from_crate = &self.stacks[operation.from];
        let base = from_crate.len() - operation.quantity;

        for i in base..from_crate.len() {
            let str = self.stacks[operation.from][i].to_owned();
            self.stacks[operation.to].push(str);
        }
        self.stacks[operation.from].truncate(base);
    }
    pub fn get_top(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack[stack.len() - 1].to_owned())
            .join("")
    }
}

#[derive(Debug)]
pub struct Operation {
    quantity: usize,
    from: usize,
    to: usize,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref LINE_RGX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        match LINE_RGX.captures(s) {
            None => Err(format!("Error parsing line '{}'", s)),
            Some(captures) => {
                let from: usize = captures[2].parse().unwrap();
                let to: usize = captures[3].parse().unwrap();

                Ok(Operation {
                    quantity: captures[1].parse().unwrap(),
                    from: from - 1,
                    to: to - 1,
                })
            }
        }
    }
}
