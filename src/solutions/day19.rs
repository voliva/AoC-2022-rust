use itertools::Itertools;
use regex::Regex;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::{Add, Mul, Sub};
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
    fn get_quality(&self, time: usize) -> usize {
        let geode = self.get_max_geode(time);
        // println!("## {} {geode} ##", self.id);
        self.id * geode
    }

    fn get_max_geode(&self, time: usize) -> usize {
        let mut to_visit: Vec<Node> = vec![];
        to_visit.push(Node {
            robots: FourTuple(1, 0, 0, 0),
            resources: FourTuple(0, 0, 0, 0),
            interests: (true, true, true, true),
        });

        let max_costs = self
            .ore
            .max_cost(self.clay)
            .max_cost(self.obsidian)
            .max_cost(self.geode);

        for _ in 0..time {
            // println!("{}: {i} {}", self.id, to_visit.len());
            let mut new_to_visit: Vec<Node> = vec![];
            for node in to_visit.iter() {
                // println!("{:?}", node);

                let mut new_interests = node.interests.clone();

                let will_have = node.resources + node.robots * time;
                let will_buy = (
                    node.interests.0 && will_have.can_buy(self.ore) && node.robots.0 < max_costs.0,
                    node.interests.1 && will_have.can_buy(self.clay) && node.robots.1 < max_costs.1,
                    node.interests.2
                        && will_have.can_buy(self.obsidian)
                        && node.robots.2 < max_costs.2,
                    node.interests.3 && will_have.can_buy(self.geode),
                );

                if node.resources.can_buy(self.ore) && will_buy.0 {
                    new_interests.0 = false;
                    let new_robots = node.robots + FourTuple(1, 0, 0, 0);
                    new_to_visit.push(Node {
                        robots: new_robots,
                        resources: node.resources + node.robots - self.ore,
                        interests: (true, true, true, true),
                    })
                }
                if node.resources.can_buy(self.clay) && will_buy.1 {
                    new_interests.1 = false;
                    let new_robots = node.robots + FourTuple(0, 1, 0, 0);
                    new_to_visit.push(Node {
                        robots: new_robots,
                        resources: node.resources + node.robots - self.clay,
                        interests: (true, true, true, true),
                    })
                }
                if node.resources.can_buy(self.obsidian) && will_buy.2 {
                    new_interests.2 = false;
                    let new_robots = node.robots + FourTuple(0, 0, 1, 0);
                    new_to_visit.push(Node {
                        robots: new_robots,
                        resources: node.resources + node.robots - self.obsidian,
                        interests: (true, true, true, true),
                    })
                }
                if node.resources.can_buy(self.geode) && will_buy.3 {
                    new_interests.3 = false;
                    let new_robots = node.robots + FourTuple(0, 0, 0, 1);
                    new_to_visit.push(Node {
                        robots: new_robots,
                        resources: node.resources + node.robots - self.geode,
                        interests: (true, true, true, true),
                    })
                }

                // If this path won't buy more robots, then stop...
                if !will_buy.0 && !will_buy.1 && !will_buy.2 && !will_buy.3 {
                    continue;
                }

                new_to_visit.push(Node {
                    robots: node.robots,
                    resources: node.resources + node.robots,
                    interests: new_interests,
                })
            }
            to_visit = new_to_visit;
        }

        // to_visit.iter().map(|node| node.resources.3).max().unwrap()
        to_visit
            .iter()
            .sorted_by(|a, b| b.resources.3.cmp(&a.resources.3))
            .take(1)
            .map(|node| node.resources.3)
            .max()
            .unwrap()
    }
}

#[derive(Clone, Copy, Debug)]
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
impl Mul<usize> for FourTuple {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        FourTuple(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl FourTuple {
    fn can_buy(&self, other: FourTuple) -> bool {
        self.0 >= other.0 && self.1 >= other.1 && self.2 >= other.2 && self.3 >= other.3
    }

    fn max_cost(&self, other: FourTuple) -> FourTuple {
        FourTuple(
            self.0.max(other.0),
            self.1.max(other.1),
            self.2.max(other.2),
            self.3.max(other.3),
        )
    }
}

#[derive(Debug)]
struct Node {
    robots: FourTuple,
    resources: FourTuple,
    interests: (bool, bool, bool, bool),
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
        Ok(input
            .iter()
            .map(|blueprint| blueprint.get_quality(24))
            .sum())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        // <53940
        Ok(input
            .iter()
            .take(3)
            .map(|blueprint| blueprint.get_max_geode(32))
            .product())
    }
}
