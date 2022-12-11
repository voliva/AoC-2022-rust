use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

pub struct Problem;

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    positive: usize,
    negative: usize,
}

fn parse_items(items_line: &String) -> Vec<usize> {
    let first_split = items_line.split(": ").collect_vec();
    first_split[1]
        .split(", ")
        .map(|x| x.parse().unwrap())
        .collect_vec()
}
fn parse_operation(operation_line: &String) -> Operation {
    let first_split = operation_line.split(" = ").collect_vec();
    let second_split = first_split[1].split(" ").collect_vec();

    match (second_split[1], second_split[2]) {
        ("*", "old") => Operation::Square,
        ("*", v) => Operation::Mul(v.parse().unwrap()),
        ("+", v) => Operation::Add(v.parse().unwrap()),
        _ => panic!("unknown operation"),
    }
}
fn parse_test(test_line: &String) -> usize {
    let first_split = test_line.split(" by ").collect_vec();
    first_split[1].parse().unwrap()
}
fn parse_throw(throw_line: &String) -> usize {
    let first_split = throw_line.split(" monkey ").collect_vec();
    first_split[1].parse().unwrap()
}

fn apply_operation(item: usize, operation: Operation) -> usize {
    match operation {
        Operation::Square => item * item,
        Operation::Add(v) => item + v,
        Operation::Mul(v) => item * v,
    }
}

impl Solver for Problem {
    type Input = Vec<Monkey>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let mut monkeys = vec![];

        let mut l = 0;
        while lines.len() > l {
            let items = parse_items(&lines[l + 1]);
            let operation = parse_operation(&lines[l + 2]);
            let test = parse_test(&lines[l + 3]);
            let positive = parse_throw(&lines[l + 4]);
            let negative = parse_throw(&lines[l + 5]);

            monkeys.push(Monkey {
                items,
                operation,
                test,
                positive,
                negative,
            });

            l += 7;
        }

        monkeys
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut monkeys = input.clone();
        let mut inspections = vec![0; input.len()];

        for _ in 0..20 {
            for m in 0..monkeys.len() {
                inspections[m] += monkeys[m].items.len();

                for i in 0..monkeys[m].items.len() {
                    let item = monkeys[m].items[i];
                    let worry = apply_operation(item, monkeys[m].operation) / 3;
                    let target = if worry % monkeys[m].test == 0 {
                        monkeys[m].positive
                    } else {
                        monkeys[m].negative
                    };

                    monkeys[target].items.push(worry);
                }
                monkeys[m].items.clear();
            }

            // println!("After round {}", r + 1);
            // for m in 0..monkeys.len() {
            //     println!("Monkey {m}: {:?}", monkeys[m].items);
            // }
            // println!("");
        }

        // println!("{:?}", inspections);
        inspections.sort();

        // 102512
        Ok(inspections
            .into_iter()
            .rev()
            .take(2)
            .reduce(|a, b| a * b)
            .unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut monkeys = input.clone();
        let mut inspections = vec![0; input.len()];

        let common = monkeys.iter().map(|m| m.test).reduce(|a, b| a * b).unwrap();

        for _ in 0..10000 {
            for m in 0..monkeys.len() {
                inspections[m] += monkeys[m].items.len();

                for i in 0..monkeys[m].items.len() {
                    let item = monkeys[m].items[i];
                    let worry = apply_operation(item, monkeys[m].operation) % common;
                    let target = if worry % monkeys[m].test == 0 {
                        monkeys[m].positive
                    } else {
                        monkeys[m].negative
                    };

                    monkeys[target].items.push(worry);
                }
                monkeys[m].items.clear();
            }

            // println!("After round {}", r + 1);
            // for m in 0..monkeys.len() {
            //     println!("Monkey {m}: {:?}", monkeys[m].items);
            // }
            // println!("");
        }

        // println!("{:?}", inspections);
        inspections.sort();

        // 102512
        Ok(inspections
            .into_iter()
            .rev()
            .take(2)
            .reduce(|a, b| a * b)
            .unwrap())
    }
}
