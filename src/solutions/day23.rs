use itertools::Itertools;

use super::Solver;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = HashSet<(isize, isize)>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .enumerate()
            .flat_map(|(y, line)| {
                line.char_indices()
                    .filter(|(_, c)| *c == '#')
                    .map(|(x, _)| (x as isize, y as isize))
                    .collect_vec()
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut field = Field::new(input.clone());

        for _ in 0..10 {
            field.turn();
        }

        let min = field.elves.iter().fold((isize::MAX, isize::MAX), |acc, v| {
            (acc.0.min(v.0), acc.1.min(v.1))
        });
        let max = field.elves.iter().fold((isize::MIN, isize::MIN), |acc, v| {
            (acc.0.max(v.0), acc.1.max(v.1))
        });

        // for y in min.1..=max.1 {
        //     for x in min.0..=max.0 {
        //         if field.elves.contains(&(x, y)) {
        //             print!("#")
        //         } else {
        //             print!(".")
        //         }
        //     }
        //     println!();
        // }

        let diff = (max.0 - min.0, max.1 - min.1);

        Ok(((diff.0 + 1) * (diff.1 + 1)) as usize - field.elves.len())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut field = Field::new(input.clone());

        for i in 1.. {
            if !field.turn() {
                return Ok(i);
            }
        }

        return Ok(0);
    }
}

struct Field {
    elves: HashSet<(isize, isize)>,
    step: usize,
}

impl Field {
    fn new(elves: HashSet<(isize, isize)>) -> Self {
        Self { elves, step: 0 }
    }

    fn turn(&mut self) -> bool {
        let checks = vec![
            (vec![(1, -1), (0, -1), (-1, -1)], (0, -1)),
            (vec![(1, 1), (0, 1), (-1, 1)], (0, 1)),
            (vec![(-1, -1), (-1, 0), (-1, 1)], (-1, 0)),
            (vec![(1, -1), (1, 0), (1, 1)], (1, 0)),
        ];

        let mut aborted_positions = HashSet::new();
        let mut intentions = HashSet::new();
        let mut moves = HashMap::new();
        for elf in self.elves.iter() {
            if !self.elves.contains(&(elf.0 + 1, elf.1 - 1))
                && !self.elves.contains(&(elf.0 + 1, elf.1))
                && !self.elves.contains(&(elf.0 + 1, elf.1 + 1))
                && !self.elves.contains(&(elf.0, elf.1 - 1))
                && !self.elves.contains(&(elf.0, elf.1 + 1))
                && !self.elves.contains(&(elf.0 - 1, elf.1 - 1))
                && !self.elves.contains(&(elf.0 - 1, elf.1))
                && !self.elves.contains(&(elf.0 - 1, elf.1 + 1))
            {
                continue;
            }

            for i in 0..4 {
                let (check, delta) = &checks[(self.step + i) % checks.len()];
                if self.run_check(elf, check) {
                    let dest = (elf.0 + delta.0, elf.1 + delta.1);
                    if intentions.contains(&dest) {
                        aborted_positions.insert(dest);
                    } else {
                        intentions.insert(dest);
                        moves.insert(elf, dest);
                    }
                    break;
                }
            }
        }

        let mut new_positions = HashSet::new();
        let mut some_moved = false;
        for elf in self.elves.iter() {
            if let Some(dest) = moves.get(elf) {
                if !aborted_positions.contains(dest) {
                    some_moved = true;
                    new_positions.insert(*dest);
                } else {
                    new_positions.insert(*elf);
                }
            } else {
                new_positions.insert(*elf);
            }
        }

        self.elves = new_positions;
        self.step = (self.step + 1) % checks.len();
        return some_moved;
    }

    fn run_check(&self, elf: &(isize, isize), check: &Vec<(isize, isize)>) -> bool {
        check
            .iter()
            .all(|deltas| !self.elves.contains(&(elf.0 + deltas.0, elf.1 + deltas.1)))
    }
}
