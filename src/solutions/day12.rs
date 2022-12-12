use itertools::Itertools;
use ndarray::Array2;
use pathfinding::prelude::dijkstra;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

pub struct InputStruct {
    maze: Array2<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Solver for Problem {
    type Input = InputStruct;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let maze_vec = file_reader
            .lines()
            .map(|x| x.unwrap())
            .enumerate()
            .map(|(line_num, line)| {
                line.bytes()
                    .enumerate()
                    .map(|(col_num, x)| match x {
                        b'S' => {
                            start = (line_num, col_num);
                            0
                        }
                        b'E' => {
                            end = (line_num, col_num);
                            b'z' - b'a'
                        }
                        o => o - b'a',
                    })
                    .collect_vec()
            })
            .collect_vec();

        let maze =
            Array2::from_shape_fn((maze_vec.len(), maze_vec[0].len()), |(r, c)| maze_vec[r][c]);
        InputStruct { maze, start, end }
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let shape = input.maze.shape();
        // println!("{:?} {:?}", input.start, input.end);

        let path = dijkstra(
            &input.start,
            |actual| {
                let res = adjacent(shape, actual)
                    .filter(|(r, c)| {
                        let current = input.maze[[actual.0, actual.1]];
                        let v = input.maze[[*r, *c]];
                        return v <= current + 1;
                    })
                    .map(|p| (p, 1))
                    .collect_vec();
                // println!("{:?} {:?}", actual, res);
                res
            },
            |p| *p == input.end,
        );

        match path {
            Some((path, _)) => Ok(path.len() - 1),
            None => Err("path not found".to_string()),
        }
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let shape = input.maze.shape();

        let starts = input
            .maze
            .indexed_iter()
            .filter(|(_, v)| **v == 0)
            .map(|(x, _)| (x, 1))
            .collect_vec();

        let nil = (usize::MAX, usize::MAX);

        let path = dijkstra(
            &nil,
            |actual| {
                if actual == &nil {
                    starts.clone()
                } else {
                    adjacent(shape, actual)
                        .filter(|(r, c)| {
                            let current = input.maze[[actual.0, actual.1]];
                            let v = input.maze[[*r, *c]];
                            return v <= current + 1;
                        })
                        .map(|p| (p, 1))
                        .collect_vec()
                }
            },
            |p| *p == input.end,
        );

        match path {
            Some((path, _)) => Ok(path.len() - 2),
            None => Err("path not found".to_string()),
        }
    }
}

fn adjacent(shape: &[usize], position: &(usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let max_y = shape[0];
    let max_x = shape[1];
    let (y, x) = *position;

    [
        (Some(x), y.checked_sub(1)),
        (x.checked_sub(1), Some(y)),
        (Some(x + 1), Some(y)),
        (Some(x), Some(y + 1)),
    ]
    .into_iter()
    .filter_map(move |v| match v {
        (Some(x), Some(y)) if x < max_x && y < max_y => Some((x, y)),
        _ => None,
    })
    .map(move |(x, y)| (y, x))
}
