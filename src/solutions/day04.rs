use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

fn merge((lf, ls): (usize, usize), (rf, rs): (usize, usize)) -> (usize, usize) {
    (lf.min(rf), ls.max(rs))
}

impl Solver for Problem {
    type Input = Vec<((usize, usize), (usize, usize))>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                let mut x = line.split(",");
                let mut l = x.next().unwrap().split("-");
                let lf: usize = l.next().unwrap().parse().unwrap();
                let ls: usize = l.next().unwrap().parse().unwrap();
                let mut r = x.next().unwrap().split("-");
                let rf: usize = r.next().unwrap().parse().unwrap();
                let rs: usize = r.next().unwrap().parse().unwrap();
                ((lf, ls), (rf, rs))
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let res = input
            .iter()
            .filter(|(left, right)| {
                let merged = merge(*left, *right);
                return *left == merged || *right == merged;
            })
            .count();

        Ok(res)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let res = input
            .iter()
            .filter(|((lf, ls), (rf, rs))| {
                return rf <= lf && lf <= rs
                    || rf <= ls && ls <= rs
                    || lf <= rf && rf <= ls
                    || lf <= rs && rs <= ls;
            })
            .count();

        Ok(res)
    }
}
