use itertools::Itertools;
use slab_tree::{NodeId, Tree, TreeBuilder};

use super::Solver;
use std::cell::RefCell;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Problem;

#[derive(Debug)]
pub enum NodeType {
    Dir,
    File(usize),
}

#[derive(Debug)]
pub struct Node {
    node_type: NodeType,
    name: String,
}

impl FromStr for Node {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("dir") {
            Ok(Node {
                node_type: NodeType::Dir,
                name: s[4..].to_owned(),
            })
        } else {
            let r = s.split(" ").collect_vec();
            Ok(Node {
                node_type: NodeType::File(r[0].parse()?),
                name: r[1].to_owned(),
            })
        }
    }
}

impl Solver for Problem {
    type Input = Tree<Node>;
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let mut tree = TreeBuilder::new()
            .with_root(Node {
                node_type: NodeType::Dir,
                name: "/".to_owned(),
            })
            .build();
        let root_id = tree.root_id().unwrap();
        let mut cd_id = root_id;

        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();

        for line in lines {
            let split = line.split(" ").collect_vec();
            if split[0] == "$" {
                if split[1] == "cd" {
                    match split[2] {
                        ".." => {
                            cd_id = tree.get(cd_id).unwrap().parent().unwrap().node_id();
                        }
                        "/" => {
                            cd_id = root_id;
                        }
                        dir => {
                            let cd = tree.get(cd_id).unwrap();
                            let child_option = cd.children().find(|c| c.data().name == dir);
                            let child = child_option.expect("cd dir: child not found");
                            cd_id = child.node_id();
                        }
                    }
                }
            } else {
                let node: Node = line.parse().unwrap();
                let mut cd = tree.get_mut(cd_id).unwrap();
                cd.append(node);
            }
        }

        tree
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        // let mut s = String::new();
        // input.write_formatted(&mut s).unwrap();
        // println!("{}", s);

        let (_, sum) = sum_dir_sizes(&input, input.root_id().unwrap(), 100000);

        Ok(sum)
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let (total, _) = sum_dir_sizes(&input, input.root_id().unwrap(), 0);
        let unused_space = 70000000 - total;
        let needs_to_delete = 30000000 - unused_space;

        let (_, res) = find_smallest_freeing(&input, input.root_id().unwrap(), needs_to_delete);

        Ok(res.unwrap())
    }
}

fn sum_dir_sizes(tree: &Tree<Node>, cd_id: NodeId, max_size: usize) -> (usize, usize) {
    // (total, sum_max)
    let mut total = 0;
    let mut sum_max = 0;
    let node = tree.get(cd_id).unwrap();

    for child in node.children() {
        let data = child.data();
        match data.node_type {
            NodeType::Dir => {
                let (child_total, child_sum_max) = sum_dir_sizes(tree, child.node_id(), max_size);
                total += child_total;
                sum_max += child_sum_max;
            }
            NodeType::File(file_size) => {
                total += file_size;
            }
        }
    }
    if total <= max_size {
        sum_max += total;
    }

    (total, sum_max)
}

fn find_smallest_freeing(
    tree: &Tree<Node>,
    cd_id: NodeId,
    min_size: usize,
) -> (usize, Option<usize>) {
    // (total, sum_max)
    let mut total = 0;
    let mut current_min: Option<usize> = None;
    let node = tree.get(cd_id).unwrap();

    for child in node.children() {
        let data = child.data();
        match data.node_type {
            NodeType::Dir => {
                let (branch_total, branch_min) =
                    find_smallest_freeing(tree, child.node_id(), min_size);
                if let Some(min) = branch_min {
                    if let Some(v) = current_min {
                        current_min = Some(v.min(min))
                    } else {
                        current_min = branch_min;
                    }
                }
                total = total + branch_total;
            }
            NodeType::File(file_size) => {
                total += file_size;
            }
        }
    }

    if total >= min_size {
        current_min = current_min.map(|v| v.min(total)).or(Some(total))
    }

    (total, current_min)
}
