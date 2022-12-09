use itertools::Itertools;

use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Operation {
    direction: Direction,
    amount: usize,
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect_vec();

        Ok(Operation {
            direction: match split[0] {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("no match for direction"),
            },
            amount: split[1].parse()?,
        })
    }
}

pub struct State {
    visited: HashSet<(isize, isize)>,
    knots: Vec<(isize, isize)>,
}

impl State {
    fn new(len: usize) -> Self {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        let knots = vec![(0, 0); len];

        State { visited, knots }
    }

    fn move_head(&mut self, op: &Operation) {
        // println!("");
        // println!("{:?}", op);

        for _ in 0..op.amount {
            self.move_one(&op.direction);
        }
    }
    fn move_one(&mut self, direction: &Direction) {
        let last_pos = self.knots.len() - 1;
        let head = self.knots[last_pos];
        self.knots[last_pos] = match direction {
            Direction::Up => (head.0, head.1 - 1),
            Direction::Down => (head.0, head.1 + 1),
            Direction::Left => (head.0 - 1, head.1),
            Direction::Right => (head.0 + 1, head.1),
        };

        self.equalize(last_pos - 1);
    }
    fn equalize(&mut self, pos: usize) {
        let head = self.knots[pos + 1];
        let tail = self.knots[pos];

        let diff = (head.0 - tail.0, head.1 - tail.1);
        if diff.0.abs() <= 1 && diff.1.abs() <= 1 {
            return;
        }

        // self.print();
        // println!("");

        match diff {
            (2, 0) => self.move_tail(pos, (1, 0)),
            (-2, 0) => self.move_tail(pos, (-1, 0)),
            (0, 2) => self.move_tail(pos, (0, 1)),
            (0, -2) => self.move_tail(pos, (0, -1)),
            (2, 2) => self.move_tail(pos, (1, 1)),
            (2, -2) => self.move_tail(pos, (1, -1)),
            (-2, 2) => self.move_tail(pos, (-1, 1)),
            (-2, -2) => self.move_tail(pos, (-1, -1)),
            (2, y) => self.move_tail(pos, (1, y)),
            (-2, y) => self.move_tail(pos, (-1, y)),
            (x, 2) => self.move_tail(pos, (x, 1)),
            (x, -2) => self.move_tail(pos, (x, -1)),
            x => panic!("diff not seen {:?}", x),
        }

        if pos > 0 {
            self.equalize(pos - 1);
        }
    }
    fn move_tail(&mut self, pos: usize, delta: (isize, isize)) {
        let tail = self.knots[pos];

        self.knots[pos] = (tail.0 + delta.0, tail.1 + delta.1);

        if pos == 0 {
            self.visited.insert(self.knots[pos].clone());
        }
    }

    fn _print(&self) {
        let min_x = self.knots.iter().map(|x| x.0).min().unwrap();
        let max_x = self.knots.iter().map(|x| x.0).max().unwrap();
        let min_y = self.knots.iter().map(|x| x.1).min().unwrap();
        let max_y = self.knots.iter().map(|x| x.1).max().unwrap();

        let mut lines = (min_y..max_y + 1)
            .map(|_| (min_x..max_x + 1).map(|_| ".").join(""))
            .collect_vec();

        for (i, knot) in self.knots.iter().enumerate() {
            lines[(knot.1 - min_y) as usize].replace_range(
                (knot.0 - min_x) as usize..(knot.0 - min_x + 1) as usize,
                i.to_string().as_str(),
            );
        }

        let result = lines.join("\n");
        println!("{result}");
    }
}

impl Solver for Problem {
    type Input = Vec<Operation>;
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
        let mut state = State::new(2);

        for op in input.into_iter() {
            state.move_head(op);
        }

        Ok(state.visited.len())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut state = State::new(10);

        for op in input.into_iter() {
            state.move_head(op);
        }

        Ok(state.visited.len())
    }
}
