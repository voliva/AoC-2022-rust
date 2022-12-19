use regex::Regex;

use super::Solver;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::{Add, Sub};
use std::str::FromStr;

pub struct Problem;

pub struct Blueprint {
    id: usize,
    ore: FourTuple,
    clay: FourTuple,
    obsidian: FourTuple,
    geode: FourTuple,
}

impl FromStr for Blueprint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static::lazy_static! {
            static ref LINE_RGX: Regex = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
        }

        match LINE_RGX.captures(s) {
            None => panic!("wrong line {s}"),
            Some(captures) => Ok(Blueprint {
                id: captures[1].parse()?,
                ore: FourTuple(captures[2].parse()?, 0, 0, 0),
                clay: FourTuple(captures[3].parse()?, 0, 0, 0),
                obsidian: FourTuple(captures[4].parse()?, captures[5].parse()?, 0, 0),
                geode: FourTuple(captures[6].parse()?, 0, captures[7].parse()?, 0),
            }),
        }
    }
}

impl Blueprint {
    fn get_quality(&self) -> usize {
        self.id * self.get_max_geode()
    }

    fn get_max_geode(&self) -> usize {
        let mut to_visit: Vec<Node> = vec![];
        to_visit.push(Node {
            robots: FourTuple(1, 0, 0, 0),
            resources: FourTuple(0, 0, 0, 0),
        });

        for i in 0..24 {
            let max_robots = 5;

            println!("{}: {i} {}", self.id, to_visit.len());
            let mut new_to_visit: Vec<Node> = vec![];
            for node in to_visit.iter() {
                if node.resources.can_buy(self.ore) && node.robots.0 < max_robots {
                    let new_robots = node.robots + FourTuple(1, 0, 0, 0);
                    new_to_visit.push(Node {
                        robots: new_robots,
                        resources: node.resources + new_robots - self.ore,
                    })
                }
                if node.resources.can_buy(self.clay) && node.robots.1 < max_robots {
                    let new_robots = node.robots + FourTuple(0, 1, 0, 0);
                    new_to_visit.push(Node {
                        robots: new_robots,
                        resources: node.resources + new_robots - self.clay,
                    })
                }
                if node.resources.can_buy(self.obsidian) && node.robots.2 < max_robots {
                    let new_robots = node.robots + FourTuple(0, 0, 1, 0);
                    new_to_visit.push(Node {
                        robots: new_robots,
                        resources: node.resources + new_robots - self.obsidian,
                    })
                }
                if node.resources.can_buy(self.geode) && node.robots.3 < max_robots {
                    let new_robots = node.robots + FourTuple(0, 0, 0, 1);
                    new_to_visit.push(Node {
                        robots: new_robots,
                        resources: node.resources + new_robots - self.geode,
                    })
                }
                new_to_visit.push(Node {
                    robots: node.robots,
                    resources: node.resources + node.robots,
                })
            }
            to_visit = new_to_visit;
        }

        to_visit.iter().map(|node| node.resources.3).max().unwrap()
    }
}

#[derive(Clone, Copy)]
struct FourTuple(usize, usize, usize, usize);

impl Add for FourTuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        FourTuple(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}
impl Sub for FourTuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        FourTuple(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl FourTuple {
    fn can_buy(&self, other: FourTuple) -> bool {
        self.0 >= other.0 && self.1 >= other.1 && self.2 >= other.2 && self.3 >= other.3
    }
}

struct Node {
    robots: FourTuple,
    resources: FourTuple,
}

impl Solver for Problem {
    type Input = Vec<Blueprint>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        Ok(input.iter().map(|blueprint| blueprint.get_quality()).sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        todo!()
    }
}
