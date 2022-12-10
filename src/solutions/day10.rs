use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

pub enum Opcode {
    Noop,
    Addx(isize),
}

impl FromStr for Opcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split(" ").collect_vec();
        match split[0] {
            "noop" => Ok(Opcode::Noop),
            "addx" => Ok(Opcode::Addx(split[1].parse()?)),
            _ => panic!("unknown opcode"),
        }
    }
}

impl Solver for Problem {
    type Input = Vec<Opcode>;
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
        let mut x = 1;
        let mut cycle = 0;
        let mut next_test = 20;
        let mut sum = 0;

        for opcode in input {
            let op_len = match opcode {
                Opcode::Noop => 1,
                Opcode::Addx(_) => 2,
            };
            if cycle + op_len >= next_test {
                sum += x * next_test;
                next_test += 40;
            }
            cycle += op_len;
            if let Opcode::Addx(add) = opcode {
                x += add;
            }

            if cycle > 220 {
                break;
            }
        }

        Ok(sum)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut display = vec![" "; 6 * 40];

        let mut x: isize = 1;
        let mut crt_pointer: usize = 0;

        for opcode in input {
            let op_len = match opcode {
                Opcode::Noop => 1,
                Opcode::Addx(_) => 2,
            };

            for _ in 0..op_len {
                if ((crt_pointer % 40) as isize - x).abs() <= 1 {
                    display[crt_pointer] = "#";
                }

                crt_pointer += 1;
            }

            if let Opcode::Addx(add) = opcode {
                x += add;
            }

            if crt_pointer >= 6 * 40 {
                break;
            }
        }

        for y in 0..6 {
            for x in 0..40 {
                print!("{} ", display[y * 40 + x]);
            }
            println!("");
        }

        Ok(0)
    }
}
