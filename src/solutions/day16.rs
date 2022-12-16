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

            (name.clone(), ImportantValve { rate, edges })
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

fn find_max(
    nodes: &HashMap<String, ImportantValve>,
    visited: &HashSet<String>,
    position: String,
    time_left: usize,
) -> usize {
    let node = nodes.get(&position).unwrap();
    let mut new_visited = visited.clone();
    new_visited.insert(position.clone());

    node.edges
        .iter()
        .filter(|(_, distance)| *distance + 1 < time_left)
        .filter(|(dest, _)| !visited.contains(*dest))
        .map(|(x, distance)| {
            let sub_time = time_left - distance - 1;
            let other_node = nodes.get(x).unwrap();
            let released = other_node.rate * sub_time;
            return find_max(nodes, &new_visited, x.clone(), sub_time) + released;
        })
        .max()
        .unwrap_or(0)
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

        Ok(find_max(
            &important_nodes,
            &HashSet::new(),
            "AA".to_owned(),
            30,
        ))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let important_nodes = reduce_nodes(&input);

        Ok(find_max_2(
            &important_nodes,
            &HashSet::new(),
            "AA".to_owned(),
            0,
            "AA".to_owned(),
            0,
            26,
        ))
    }
}

fn find_max_2(
    nodes: &HashMap<String, ImportantValve>,
    visited: &HashSet<String>,
    position_a: String,
    timeout_a: usize,
    position_b: String,
    timeout_b: usize,
    time_left: usize,
) -> usize {
    if timeout_a == 0 {
        let node = nodes.get(&position_a).unwrap();
        let all_edges = node
            .edges
            .iter()
            .filter(|(_, distance)| *distance + 1 < time_left)
            .filter(|(dest, _)| !visited.contains(*dest))
            .collect_vec();

        let take = if timeout_b == 0 {
            (all_edges.len() + 1) / 2
        } else {
            all_edges.len()
        };

        all_edges
            .into_iter()
            .take(take)
            .map(|(x, distance)| {
                if time_left == 26 {
                    println!("{x} {take}");
                }

                let timeout_a = distance + 1;
                let sub_time = time_left - timeout_a;
                let other_node = nodes.get(x).unwrap();
                let released = other_node.rate * sub_time;

                let next_step = timeout_a.min(timeout_b);

                let mut new_visited = visited.clone();
                new_visited.insert(x.clone());

                return find_max_2(
                    nodes,
                    &new_visited,
                    x.clone(),
                    timeout_a - next_step,
                    position_b.clone(),
                    timeout_b - next_step,
                    time_left - next_step,
                ) + released;
            })
            .max()
            .unwrap_or(0)
    } else if timeout_b == 0 {
        let node = nodes.get(&position_b).unwrap();

        node.edges
            .iter()
            .filter(|(_, distance)| *distance + 1 < time_left)
            .filter(|(dest, _)| !visited.contains(*dest))
            .map(|(x, distance)| {
                let timeout_b = distance + 1;
                let sub_time = time_left - timeout_b;
                let other_node = nodes.get(x).unwrap();
                let released = other_node.rate * sub_time;

                let mut new_visited = visited.clone();
                new_visited.insert(x.clone());

                let next_step = timeout_a.min(timeout_b);

                return find_max_2(
                    nodes,
                    &new_visited,
                    position_a.clone(),
                    timeout_a - next_step,
                    x.clone(),
                    timeout_b - next_step,
                    time_left - next_step,
                ) + released;
            })
            .max()
            .unwrap_or(0)
    } else {
        panic!("No timeout = 0?")
    }
}
