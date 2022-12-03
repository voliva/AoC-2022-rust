use super::Solver;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

fn get_char_priority(c: char) -> usize {
    if c >= 'a' && c <= 'z' {
        c as usize - 'a' as usize + 1
    } else {
        c as usize - 'A' as usize + 27
    }
}

fn get_priority(left: &Vec<char>, right: &Vec<char>) -> usize {
    let left_set: HashSet<&char> = left.iter().collect();

    let duplicate = right.iter().find(|c| left_set.contains(c));
    let c = duplicate.unwrap();

    get_char_priority(*c)
}

fn join_items((left, right): &(Vec<char>, Vec<char>)) -> Vec<char> {
    left.into_iter()
        .chain(right.into_iter())
        .map(|x| x.to_owned())
        .collect()
}

impl Solver for Problem {
    type Input = Vec<(Vec<char>, Vec<char>)>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| {
                (
                    line[..line.len() / 2].chars().collect(),
                    line[line.len() / 2..].chars().collect(),
                )
            })
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let res = input
            .into_iter()
            .map(|(left, right)| get_priority(left, right))
            .sum();

        Ok(res)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut total = 0;

        for i in (0..input.len()).step_by(3) {
            let items_a = join_items(&input[i]);
            let items_b = join_items(&input[i + 1]);
            let items_c = join_items(&input[i + 2]);

            let mut char_set: HashSet<&char> = items_a.iter().collect();
            char_set = items_b.iter().filter(|c| char_set.contains(c)).collect();
            char_set = items_c.iter().filter(|c| char_set.contains(c)).collect();

            let priority = get_char_priority(**char_set.iter().next().unwrap());
            total += priority;
        }

        Ok(total)
    }
}
