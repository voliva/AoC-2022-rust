use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<usize>;
    type Output1 = String;
    type Output2 = isize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| parse_snafu(&line))
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let sum = input.iter().map(|x| *x).sum();

        let formatted = format_snafu(sum);
        println!("");
        println!("{sum} {formatted} {}", parse_snafu(&formatted));

        // 122--==-=211==-2-200
        Ok(formatted)
    }

    fn solve_second(&self, _: &Self::Input) -> Result<Self::Output2, String> {
        Ok(0)
    }
}

fn parse_snafu(v: &str) -> usize {
    v.chars()
        .rev()
        .enumerate()
        .map(|(power, c)| {
            let base = match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => unreachable!(),
            };
            5_isize.pow(power as u32) * base
        })
        .sum::<isize>() as usize
}

fn format_snafu(mut x: usize) -> String {
    let mut result = vec![];

    let mut extra = 0;
    loop {
        let m = x % 5 + extra;
        x = x / 5;

        let c = if m >= 3 {
            print!("{m}");
            extra = 1;
            if m == 3 {
                '='
            } else if m == 4 {
                '-'
            } else {
                '0'
            }
        } else {
            extra = 0;
            std::char::from_digit(m as u32, 5).unwrap()
        };

        // will panic if you use a bad radix (< 2 or > 36).
        result.push(c);
        if x == 0 {
            break;
        }
    }
    result.into_iter().rev().collect()
}
