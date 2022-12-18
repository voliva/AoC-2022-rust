use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Droplet {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Droplet {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(",").collect_vec();

        Ok(Droplet {
            x: split[0].parse()?,
            y: split[1].parse()?,
            z: split[2].parse()?,
        })
    }
}

impl Droplet {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}

impl Solver for Problem {
    type Input = Vec<Droplet>;
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
        let droplets: HashSet<Droplet> = HashSet::from_iter(input.iter().map(|x| x.clone()));

        Ok(input
            .iter()
            .flat_map(|d| get_adjacent(d))
            .filter(|d| !droplets.contains(d))
            .count())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        // let mut droplets: HashMap<isize, Vec<Droplet>> = HashMap::new();

        // input.iter().for_each(|d| {
        //     if !droplets.contains_key(&d.x) {
        //         droplets.insert(d.x, vec![]);
        //     }
        //     droplets.get_mut(&d.x).unwrap().push(d.clone());
        // });

        // droplets.keys().sorted().for_each(|x| {
        //     let group = droplets.get(x).unwrap();

        //     let max_y = group.iter().map(|d| d.y).max().unwrap();
        //     let max_z = group.iter().map(|d| d.z).max().unwrap();
        //     let set: HashSet<(isize, isize)> = HashSet::from_iter(group.iter().map(|d| (d.y, d.z)));

        //     println!("{x}: ");
        //     for y in 0..=max_y {
        //         for z in 0..=max_z {
        //             if set.contains(&(y, z)) {
        //                 print!("#")
        //             } else {
        //                 print!(".")
        //             }
        //         }
        //         println!("");
        //     }
        //     println!("");
        // });

        let droplets: HashSet<Droplet> = HashSet::from_iter(input.iter().map(|x| x.clone()));

        let candidates: HashSet<Droplet> = input
            .iter()
            .flat_map(|d| get_adjacent(d))
            .filter(|d| !droplets.contains(d))
            .collect();

        let start = candidates
            .iter()
            .reduce(|acc, v| {
                if v.x < acc.x && v.y < acc.y && v.z < acc.z {
                    v
                } else {
                    acc
                }
            })
            .unwrap();

        let mut visited: HashSet<Droplet> = HashSet::new();
        let mut visited_candidates: HashSet<Droplet> = HashSet::new();
        let mut to_visit: Vec<Droplet> = vec![start.clone()];

        while !to_visit.is_empty() {
            let p = to_visit.pop().unwrap();
            if visited.contains(&p) {
                continue;
            }
            visited.insert(p.clone());
            let adjacent = get_adjacent(&p);
            let consider = if candidates.contains(&p) {
                visited_candidates.insert(p.clone());
                adjacent
                    .iter()
                    .filter(|x| !droplets.contains(x))
                    .collect_vec()
            } else {
                adjacent
                    .iter()
                    .filter(|x| candidates.contains(x))
                    .collect_vec()
            };
            consider.into_iter().for_each(|c| to_visit.push(c.clone()));
        }

        let res = visited_candidates
            .iter()
            // .filter(|c| !visited_candidates.contains(c))
            .flat_map(|c| get_adjacent(c))
            .filter(|c| droplets.contains(c))
            .count();

        // println!(
        //     "{} {} {}",
        //     candidates.len(),
        //     visited_candidates,
        //     candidates.len() - visited_candidates
        // );

        // >663
        Ok(res)
    }
}

fn get_adjacent(droplet: &Droplet) -> Vec<Droplet> {
    vec![
        Droplet::new(droplet.x + 1, droplet.y, droplet.z),
        Droplet::new(droplet.x - 1, droplet.y, droplet.z),
        Droplet::new(droplet.x, droplet.y + 1, droplet.z),
        Droplet::new(droplet.x, droplet.y - 1, droplet.z),
        Droplet::new(droplet.x, droplet.y, droplet.z + 1),
        Droplet::new(droplet.x, droplet.y, droplet.z - 1),
    ]
}
