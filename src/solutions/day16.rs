use itertools::Itertools;
use pathfinding::prelude::dijkstra_all;
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

#[derive(Debug)]
pub struct ImportantValve {
    name: String,
    rate: usize,
    edges: HashMap<String, usize>,
}

fn reduce_nodes(nodes: &HashMap<String, Valve>) -> HashMap<String, ImportantValve> {
    let important_nodes = nodes
        .values()
        .filter(|x| x.rate > 0 || x.name == "AA")
        .collect_vec();

    important_nodes
        .iter()
        .map(|origin| {
            let name = origin.name.clone();
            let rate = origin.rate;

            let res = dijkstra_all(&name, |p| {
                nodes
                    .get(p)
                    .unwrap()
                    .edges
                    .iter()
                    .map(|e| (e.to_owned(), 1))
            });

            let edges: HashMap<String, usize> = important_nodes
                .iter()
                .filter(|other| other.name != name)
                .map(|other| (other.name.clone(), res.get(&other.name).unwrap().1))
                .collect();

            (name.clone(), ImportantValve { name, rate, edges })
        })
        .collect()
}

// #[derive(Clone, PartialEq, Eq, Hash)]
// struct SearchState {
//     position: String,
//     visited: HashSet<String>,
//     release: usize,
//     time: usize,
// }

// impl SearchState {
//     fn new(position: &str) -> Self {
//         Self {
//             position: position.to_owned(),
//             visited: HashSet::new(),
//             release: 0,
//             time: 0,
//         }
//     }
// }

#[derive(Debug, PartialOrd)]
struct Possibility {
    to: String,
    release: usize,
}

impl PartialEq for Possibility {
    fn eq(&self, other: &Self) -> bool {
        return self.release.eq(&other.release);
    }
}
impl Eq for Possibility {}

impl Ord for Possibility {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.release.cmp(&other.release);
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
        let important_nodes = reduce_nodes(&input);

        let mut time = 0;
        let mut released = 0;
        let mut visited: HashSet<String> = HashSet::new();
        let mut position = "AA".to_owned();

        while time < 30 && visited.len() < important_nodes.len() - 1 {
            println!("{:?}", position);
            let node = input.get(&position).unwrap();
            visited.insert(position.clone());
            released += node.rate * (30 - time);

            let distances = &important_nodes.get(&position).unwrap().edges;
            let max = important_nodes
                .keys()
                .filter(|other| !visited.contains(other.clone()))
                .map(|other| {
                    let other_node = input.get(other).unwrap();

                    let total_time = time + distances.get(other).unwrap() + 1;

                    return Possibility {
                        to: other.to_owned(),
                        release: if total_time >= 30 {
                            0
                        } else {
                            other_node.rate * (30 - total_time)
                        },
                    };
                })
                .max()
                .unwrap();

            time += distances.get(&max.to).unwrap() + 1;
            position = max.to.clone();
        }

        // >395
        Ok(released)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}
