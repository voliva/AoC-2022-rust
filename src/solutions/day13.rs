use itertools::Itertools;

use super::Solver;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

pub type List = Vec<ListValue>;

pub enum ListValue {
    Integer(usize),
    List(List),
}

impl Debug for ListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListValue::Integer(i) => write!(f, "{}", i),
            ListValue::List(l) => write!(f, "{:?}", l),
        }
    }
}

fn read_list(str: &str) -> (List, String) {
    let mut elements: List = vec![];
    let mut remaining = str.to_string();
    assert_eq!(&remaining[0..1], "[");
    remaining = remaining[1..].to_string();

    while remaining.len() > 0 && &remaining[0..1] != "]" {
        if &remaining[0..1] == "[" {
            let (sub_elements, sub_remaining) = read_list(&remaining);
            elements.push(ListValue::List(sub_elements));

            remaining = sub_remaining;
            if remaining.len() > 0 && &remaining[0..1] == "," {
                remaining = remaining[1..].to_string();
            }
        } else {
            let pos_comma = remaining.find(",");
            let pos_end = remaining.find("]");
            match (pos_comma, pos_end) {
                (Some(c), Some(e)) => {
                    let p = c.min(e);
                    if p != 0 {
                        let value: usize = remaining[0..p].parse().unwrap();
                        elements.push(ListValue::Integer(value));
                    }
                    remaining = remaining[(p + 1)..].to_string();
                    if p == e {
                        break;
                    }
                }
                (Some(p), _) => {
                    let value: usize = remaining[0..p].parse().unwrap();
                    elements.push(ListValue::Integer(value));
                    remaining = remaining[(p + 1)..].to_string();
                }
                (None, Some(p)) => {
                    if p != 0 {
                        let value: usize = remaining[0..p].parse().unwrap();
                        elements.push(ListValue::Integer(value));
                    }
                    remaining = remaining[(p + 1)..].to_string();
                    break;
                }
                _ => panic!("no marker found"),
            }
        }
    }

    if remaining.len() > 0 && &remaining[0..1] == "]" {
        remaining = remaining[1..].to_string()
    }

    // println!("{str} => {:?} => {remaining}", elements);
    (elements, remaining)
}

impl Solver for Problem {
    type Input = Vec<(List, List)>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        let mut result: Vec<(List, List)> = vec![];
        let mut line = 0;
        while line < lines.len() {
            let (left, _) = read_list(&lines[line]);
            let (right, _) = read_list(&lines[line + 1]);
            result.push((left, right));

            line += 3;
        }

        result
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        // for (left, right) in input {
        //     println!("{:?} {:?}", left, right);
        // }

        let res: usize = input
            .iter()
            .enumerate()
            .filter(|(_, (left, right))| {
                let res = cmp(left, right);
                if res == Ordering::Equal {
                    println!("{:?} {:?}", left, right)
                }
                res == Ordering::Less
            })
            .map(|(i, _)| i + 1)
            .sum();

        Ok(res)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        // 20196 too low
        let first = find_divider(input, 2) + 1;
        let second = find_divider(input, 6) + 2;
        Ok(first * second)
    }
}

fn cmp(left: &List, right: &List) -> Ordering {
    let left_len = left.len();
    let right_len = right.len();
    let min_len = left_len.min(right_len);

    // println!("cmp {:?} {:?}", left, right);

    for i in 0..min_len {
        let res = match (&left[i], &right[i]) {
            (ListValue::Integer(l), ListValue::Integer(r)) => l.cmp(r),
            (ListValue::List(l), ListValue::List(r)) => cmp(&l, &r),
            (ListValue::List(l), ListValue::Integer(r)) => cmp(&l, &vec![ListValue::Integer(*r)]),
            (ListValue::Integer(l), ListValue::List(r)) => cmp(&vec![ListValue::Integer(*l)], &r),
        };
        if res != Ordering::Equal {
            // println!("cmp {:?} {:?} => {:?}", left, right, res);
            return res;
        }
    }

    // println!("cmp {:?} {:?} eq, {left_len} ? {right_len}", left, right);
    if left_len < right_len {
        Ordering::Less
    } else if left_len > right_len {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn find_divider(packets: &Vec<(List, List)>, divider_num: usize) -> usize {
    let divider: List = vec![ListValue::List(vec![ListValue::Integer(divider_num)])];

    packets
        .iter()
        .flat_map(|(l, r)| vec![l, r])
        .filter(|packet| cmp(packet, &divider) == Ordering::Less)
        .count()
}
