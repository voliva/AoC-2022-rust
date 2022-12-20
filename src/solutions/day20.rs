use slotmap::{DefaultKey, Key, SlotMap};

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

struct Node {
    num: i64,
    prev: DefaultKey,
    next: DefaultKey,
}

impl Node {
    fn new(num: i64) -> Self {
        Self {
            num,
            prev: DefaultKey::null(),
            next: DefaultKey::null(),
        }
    }
}

impl Solver for Problem {
    type Input = Vec<isize>;
    type Output1 = i64;
    type Output2 = i64;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        file_reader
            .lines()
            .map(|x| x.unwrap())
            .map(|line| line.parse())
            .map(|x| x.unwrap())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut sm = SlotMap::new();

        let mut keys = vec![];
        let first = sm.insert(Node::new(input[0] as i64));
        keys.push(first);

        let mut prev = first.clone();
        let mut zero = DefaultKey::null();
        for i in 1..input.len() {
            let next = sm.insert(Node::new(input[i] as i64));
            keys.push(next);
            if input[i] == 0 {
                zero = next;
            }
            sm[prev].next = next;
            sm[next].prev = prev;
            prev = next;
        }
        sm[first].prev = prev;
        sm[prev].next = first;

        for key in keys {
            let prev = sm[key].prev;
            let next = sm[key].next;
            sm[prev].next = next;
            sm[next].prev = prev;

            let mut target = key;
            let node = &sm[key];
            for _ in 0..node.num.abs() {
                if node.num < 0 {
                    target = sm[target].prev
                } else {
                    target = sm[target].next
                }
            }

            if node.num <= 0 {
                target = sm[target].prev
            }

            let next = sm[target].next;
            sm[target].next = key;
            sm[next].prev = key;
            sm[key].prev = target;
            sm[key].next = next;
        }

        let mut target = zero;
        let mut res = 0;

        for _ in 0..3 {
            for _ in 0..1000 {
                target = sm[target].next;
            }
            println!("{}", sm[target].num);
            res += sm[target].num;
        }

        Ok(res)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut sm = SlotMap::new();

        let encryption_key = 811589153;
        let modulo = input.len() as i64;
        let mut keys = vec![];
        let first = sm.insert(Node::new(input[0] as i64 * encryption_key));
        keys.push(first);

        let mut prev = first.clone();
        let mut zero = DefaultKey::null();
        for i in 1..input.len() {
            let next = sm.insert(Node::new(input[i] as i64 * encryption_key));
            keys.push(next);
            if input[i] == 0 {
                zero = next;
            }
            sm[prev].next = next;
            sm[next].prev = prev;
            prev = next;
        }
        sm[first].prev = prev;
        sm[prev].next = first;

        // println!("original");
        // let mut target = zero;
        // for _ in 0..modulo {
        //     println!("{} ({})", sm[target].num / encryption_key, sm[target].num);
        //     target = sm[target].next;
        // }
        // println!("===");

        for _ in 0..10 {
            for key in keys.iter() {
                let key = *key;
                // println!("move {} ({})", sm[key].num / encryption_key, sm[key].num);
                let prev = sm[key].prev;
                let next = sm[key].next;
                sm[prev].next = next;
                sm[next].prev = prev;

                let mut target = key;
                let node = &sm[key];
                let num = node.num % (modulo - 1); // why?
                for _ in 0..num.abs() {
                    if num < 0 {
                        target = sm[target].prev
                    } else {
                        target = sm[target].next
                    }
                }

                if num <= 0 {
                    target = sm[target].prev
                }

                let next = sm[target].next;
                sm[target].next = key;
                sm[next].prev = key;
                sm[key].prev = target;
                sm[key].next = next;

                // let mut target = zero;
                // for _ in 0..modulo {
                //     println!("{} ({})", sm[target].num / encryption_key, sm[target].num);
                //     target = sm[target].next;
                // }
                // println!("===");
            }

            // println!("===");
            // let mut target = zero;
            // for _ in 0..modulo {
            //     println!("{}", sm[target].num);
            //     target = sm[target].next;
            // }
            // println!("===");
        }

        let mut target = zero;
        let mut res = 0;

        for _ in 0..3 {
            for _ in 0..1000 {
                target = sm[target].next;
            }
            println!("{}", sm[target].num);
            res += sm[target].num;
        }

        Ok(res)
    }
}
