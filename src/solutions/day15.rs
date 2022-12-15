use super::Solver;
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;
use std::vec;

pub struct Problem;

pub struct Detection {
    sensor: (isize, isize),
    beacon: (isize, isize),
}

impl FromStr for Detection {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(": closest beacon is at x=").collect_vec();
        let beacon: (isize, isize) = split[1]
            .split(", y=")
            .map(|x| x.parse().unwrap())
            .collect_tuple()
            .unwrap();

        let split = split[0].split(", y=").collect_vec();
        let sensor: (isize, isize) = (
            split[0][("Sensor at x=".len())..].parse().unwrap(),
            split[1].parse().unwrap(),
        );

        Ok(Detection { sensor, beacon })
    }
}

impl Solver for Problem {
    type Input = Vec<Detection>;
    type Output1 = isize;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut row = Row {
            segments: vec![],
            beacons: HashSet::new(),
        };
        let row_pos: isize = 2_000_000;

        for detection in input {
            let (sx, sy) = detection.sensor;
            let (bx, by) = detection.beacon;
            let distance = (sx - bx).abs() + (sy - by).abs();
            let row_distance = (sy - row_pos).abs();
            // width = 0 -> distance, 1 -> distance-1, 2 -> distance-2, etc.
            let signal_strength = distance - row_distance;
            if signal_strength >= 0 {
                row.add_segment((sx - signal_strength, sx + signal_strength));
            }
            if by == row_pos {
                row.add_beacon(bx);
            }
        }

        Ok(row.get_length())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut rows = vec![];
        let max = 4_000_000;
        for _ in 0..(max + 1) {
            rows.push(Row {
                segments: vec![],
                beacons: HashSet::new(),
            });
        }

        for detection in input {
            let (sx, sy) = detection.sensor;
            let (bx, by) = detection.beacon;

            for row_pos in 0..rows.len() {
                let distance = (sx - bx).abs() + (sy - by).abs();
                let row_distance = (sy - row_pos as isize).abs();
                // width = 0 -> distance, 1 -> distance-1, 2 -> distance-2, etc.
                let signal_strength = distance - row_distance;
                if signal_strength >= 0 {
                    rows[row_pos].add_segment((sx - signal_strength, sx + signal_strength));
                }
            }
        }

        for row_pos in 0..rows.len() {
            if let Some(x) = rows[row_pos].get_free_pos(max) {
                // println!("{x}");
                return Ok(x * 4000000 + row_pos as isize);
            }
        }

        // not 10229183267339
        Ok(0)
    }
}

struct Row {
    beacons: HashSet<isize>,
    segments: Vec<(isize, isize)>,
}

impl Row {
    fn add_segment(&mut self, segment: (isize, isize)) {
        self.segments.push(segment);
        self.segments.sort_by(|a, b| a.0.cmp(&b.0));
        let mut new_segments = vec![self.segments[0]];

        for s in self.segments.iter() {
            let last = new_segments.len() - 1;
            if new_segments[last].1 >= s.0 {
                new_segments[last].1 = s.1.max(new_segments[last].1);
            } else {
                new_segments.push(*s);
            }
        }

        self.segments = new_segments;
        // println!("add segment {:?} -> {:?}", segment, self.segments);
    }
    fn add_beacon(&mut self, x: isize) {
        self.beacons.insert(x);
    }
    fn get_length(&self) -> isize {
        self.segments
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum::<isize>()
            - self.beacons.len() as isize
    }
    fn get_free_pos(&self, max: isize) -> Option<isize> {
        // println!("{:?}", self.segments);
        self.segments
            .iter()
            .map(|s| s.to_owned())
            .find(|(start, end)| 0 < *start && *start < max || 0 < *end && *end < max)
            .map(|(start, end)| {
                if 0 < start && start < max {
                    return start - 1;
                }
                return end + 1;
            })
    }
}
