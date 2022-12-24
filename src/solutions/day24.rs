use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Clone)]
pub struct Blizzard {
    direction: Direction,
    position: (usize, usize),
}

impl Blizzard {
    fn step(&mut self, width: usize, height: usize) {
        self.position = match (self.direction, self.position) {
            (Direction::Right, (x, y)) if x == width - 2 => (1, y),
            (Direction::Right, (x, y)) => (x + 1, y),
            (Direction::Left, (x, y)) if x == 1 => (width - 2, y),
            (Direction::Left, (x, y)) => (x - 1, y),
            (Direction::Up, (x, y)) if y == 1 => (x, height - 2),
            (Direction::Up, (x, y)) => (x, y - 1),
            (Direction::Down, (x, y)) if y == height - 2 => (x, 1),
            (Direction::Down, (x, y)) => (x, y + 1),
        };
    }
}

#[derive(Clone)]
pub struct Field {
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
}

impl Field {
    fn step(&mut self) -> HashSet<(usize, usize)> {
        self.blizzards
            .iter_mut()
            .map(|blizzard| {
                blizzard.step(self.width, self.height);
                blizzard.position
            })
            .collect()
    }

    fn run(&mut self, start: (usize, usize), end: (usize, usize)) -> usize {
        let mut positions = HashSet::new();
        positions.insert(start);

        let mut step = 0;
        loop {
            let mut new_positions = HashSet::new();
            step += 1;
            let occupied = self.step();

            for position in positions {
                let adj: HashSet<_> = adjacent(self.width, self.height, &position)
                    .into_iter()
                    .filter(|p| !occupied.contains(p))
                    .collect();

                if adj.contains(&end) {
                    // println!("{step} final");
                    // for y in 0..self.height {
                    //     for x in 0..self.width {
                    //         if occupied.contains(&(x, y)) {
                    //             print!("@");
                    //         } else if end == (x, y) {
                    //             print!("E");
                    //         } else if x == 0
                    //             || (y == 0 && x != 1)
                    //             || x == self.width - 1
                    //             || (y == self.height - 1 && x != self.width - 2)
                    //         {
                    //             print!("#")
                    //         } else {
                    //             print!(".");
                    //         }
                    //     }
                    //     println!();
                    // }

                    return step;
                }

                adj.into_iter().for_each(|p| {
                    new_positions.insert(p);
                });
            }

            // println!("{step}");
            // for y in 0..self.height {
            //     for x in 0..self.width {
            //         if occupied.contains(&(x, y)) {
            //             print!("@");
            //         } else if new_positions.contains(&(x, y)) {
            //             print!("E");
            //         } else if x == 0
            //             || (y == 0 && x != 1)
            //             || x == self.width - 1
            //             || (y == self.height - 1 && x != self.width - 2)
            //         {
            //             print!("#")
            //         } else {
            //             print!(".");
            //         }
            //     }
            //     println!();
            // }

            positions = new_positions;
            // println!("{step} {:?}", positions);
        }
    }
}

impl Solver for Problem {
    type Input = Field;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.char_indices().collect_vec())
            .collect_vec();

        let height = lines.len();
        let width = lines[0].len();
        let blizzards = lines
            .into_iter()
            .enumerate()
            .skip(1)
            .flat_map(|(y, line)| {
                line.into_iter()
                    .filter(|(_, c)| *c != '#' && *c != '.')
                    .map(|(x, c)| Blizzard {
                        direction: match c {
                            '>' => Direction::Right,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            '^' => Direction::Up,
                            _ => panic!("unknown char"),
                        },
                        position: (x, y),
                    })
                    .collect_vec()
            })
            .collect_vec();
        Field {
            width,
            height,
            blizzards,
        }
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut field = input.clone();

        Ok(field.run((1, 0), (field.width - 2, field.height - 1)))
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut field = input.clone();

        let start = (1, 0);
        let end = (field.width - 2, field.height - 1);
        let mut res = field.run(start, end);
        res += field.run(end, start);
        res += field.run(start, end);

        Ok(res)
    }
}

fn adjacent(width: usize, height: usize, position: &(usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = position.clone();
    let mut res = vec![(x, y)];

    if y > 1 || (y == 1 && x == 1) {
        res.push((x, y - 1));
    }
    if y < height - 2 || (y == height - 2 && x == width - 2) {
        res.push((x, y + 1));
    }
    if x > 1 && y != height - 1 {
        res.push((x - 1, y));
    }
    if x < width - 2 && y != 0 {
        res.push((x + 1, y));
    }

    res
}

#[derive(Clone, Copy)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}
