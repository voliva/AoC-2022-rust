use itertools::Itertools;
use regex::Regex;

use super::Solver;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

// enum Operation {
//     Add,
//     Sub,
//     Mul,
//     Div,
// }

pub enum Yell {
    Number(isize),
    Op(String, String, String), // op, left, right
}

impl FromStr for Yell {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref OP_REG: Regex = Regex::new(r"(\w+) ([\+\-\*/]) (\w+)").unwrap();
        }

        let regex_result = OP_REG.captures(s);

        match regex_result {
            None => Ok(Yell::Number(s.parse()?)),
            Some(captures) => Ok(Yell::Op(
                captures[2].to_owned(),
                captures[1].to_owned(),
                captures[3].to_owned(),
            )),
        }
    }
}

impl Solver for Problem {
    type Input = HashMap<String, Yell>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let split = line.split(": ").collect_vec();
                let name = split[0].to_owned();
                let yell: Yell = split[1].parse().unwrap();

                (name, yell)
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(calculate(input, "root", &mut HashMap::new()))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut cache = HashMap::new();
        cache.insert("humn".to_owned(), 1);

        let mut res = calculate2(input, "root", &mut cache);
        let initial_res = res;
        let mut min_val = 1;
        let mut current_val = 1;
        while res == initial_res {
            min_val = current_val;
            current_val = current_val * 2 + 10;
            let mut cache = HashMap::new();
            cache.insert("humn".to_owned(), current_val);
            res = calculate2(input, "root", &mut cache);
        }
        if res == 0 {
            return Ok(current_val);
        }

        let mut max_val = current_val;

        loop {
            if max_val - min_val < 10 {
                for current_val in min_val..=max_val {
                    let mut cache = HashMap::new();
                    cache.insert("humn".to_owned(), current_val);
                    let res = calculate2(input, "root", &mut cache);
                    if res == 0 {
                        return Ok(current_val);
                    }
                }
                panic!("not found?!");
            }

            current_val = (max_val + min_val) / 2;
            let mut cache = HashMap::new();
            cache.insert("humn".to_owned(), current_val);
            let res = calculate2(input, "root", &mut cache);
            if res == 0 {
                return Ok(current_val);
            } else if res == initial_res {
                min_val = current_val;
            } else {
                max_val = current_val;
            }
        }
    }
}

fn calculate(
    monkeys: &HashMap<String, Yell>,
    name: &str,
    cache: &mut HashMap<String, isize>,
) -> isize {
    if let Some(r) = cache.get(name) {
        return *r;
    }

    let yell = monkeys.get(name).unwrap();
    match yell {
        Yell::Number(r) => *r,
        Yell::Op(op, left_name, right_name) => {
            let left = calculate(monkeys, left_name, cache);
            let right = calculate(monkeys, right_name, cache);

            let res = apply_op(op, left, right);
            cache.insert(name.to_owned(), res);
            res
        }
    }
}

fn calculate2(
    monkeys: &HashMap<String, Yell>,
    name: &str,
    cache: &mut HashMap<String, isize>,
) -> isize {
    if let Some(r) = cache.get(name) {
        return *r;
    }

    let yell = monkeys.get(name).unwrap();
    match yell {
        Yell::Number(r) => *r,
        Yell::Op(op, left_name, right_name) => {
            let left = calculate(monkeys, left_name, cache);
            let right = calculate(monkeys, right_name, cache);

            if name == "root" {
                // println!("{left} =?= {right}");
                return if left < right {
                    -1
                } else if left > right {
                    1
                } else {
                    0
                };
            }
            let res = apply_op(op, left, right);
            cache.insert(name.to_owned(), res);
            res
        }
    }
}

fn apply_op(op: &str, left: isize, right: isize) -> isize {
    match op {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => panic!("unknown op"),
    }
}
