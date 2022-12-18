use super::Solver;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

lazy_static! {
    #[rustfmt::skip]
    static ref SHAPES: Vec<Vec<u8>> = vec![
        vec![0b0011110],
        vec![
            0b0001000,
            0b0011100,
            0b0001000
        ],
        vec![
            0b0011100, // [0] is the bottom edge!
            0b0000100,
            0b0000100,
        ],
        vec![
            0b0010000,
            0b0010000,
            0b0010000,
            0b0010000
        ],
        vec![
            0b0011000,
            0b0011000
        ]
    ];
}

impl Solver for Problem {
    type Input = Vec<Direction>;
    type Output1 = usize;
    type Output2 = u64;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let first_line = file_reader
            .lines()
            .map(|x| x.unwrap())
            .find(|_| true)
            .unwrap();

        return first_line
            .chars()
            .map(|c| {
                if c == '>' {
                    Direction::Right
                } else {
                    Direction::Left
                }
            })
            .collect_vec();
    }

    fn solve_first(&self, input: &Self::Input) -> Result<Self::Output1, String> {
        let mut field = Field::new();

        let mut shape_index = 0;
        let mut op_index = 0;
        for _ in 0..2022 {
            let mut shape = SHAPES[shape_index % SHAPES.len()].clone();
            shape_index += 1;

            for o in -4.. {
                let dir = input[op_index % input.len()];
                op_index += 1;

                let can_move = shape.iter().all(|v| match dir {
                    Direction::Left => (v & 0b0100_0000) == 0,
                    Direction::Right => (v & 0b0000_0001) == 0,
                });

                let shape_bak = shape.clone();
                if can_move {
                    for i in 0..shape.len() {
                        shape[i] = match dir {
                            Direction::Left => shape[i] << 1,
                            Direction::Right => shape[i] >> 1,
                        };
                    }
                }

                if o >= 0 {
                    if field.intersects(&shape, o as usize) {
                        shape = shape_bak;
                    }
                }

                if o >= -1 {
                    if field.intersects(&shape, (o + 1) as usize) {
                        field.push_shape(&shape, o);
                        break;
                    }
                }
            }
            // field.print();
            // println!();
        }

        Ok(field.lines.len())
    }

    fn solve_second(&self, input: &Self::Input) -> Result<Self::Output2, String> {
        let mut field = Field::new();

        let mut shape_index = 0;
        let mut op_index = 0;

        for _ in 0.. {
            if shape_index > 0 && shape_index % SHAPES.len() == 0 && op_index % input.len() == 0 {
                break;
            }

            let mut shape = SHAPES[shape_index % SHAPES.len()].clone();
            shape_index += 1;

            for o in -4.. {
                let dir = input[op_index % input.len()];
                op_index += 1;

                let can_move = shape.iter().all(|v| match dir {
                    Direction::Left => (v & 0b0100_0000) == 0,
                    Direction::Right => (v & 0b0000_0001) == 0,
                });

                let shape_bak = shape.clone();
                if can_move {
                    for i in 0..shape.len() {
                        shape[i] = match dir {
                            Direction::Left => shape[i] << 1,
                            Direction::Right => shape[i] >> 1,
                        };
                    }
                }

                if o >= 0 {
                    if field.intersects(&shape, o as usize) {
                        shape = shape_bak;
                    }
                }

                if o >= -1 {
                    if field.intersects(&shape, (o + 1) as usize) {
                        field.push_shape(&shape, o);
                        break;
                    }
                }
            }
        }

        let every_round = field.lines.len();
        let round_size = shape_index;
        let v: u64 = 1000000000000;
        let total_rounds = v / (round_size as u64);
        let remaining_rounds = v % (round_size as u64);

        println!("er: {every_round}, rs: {round_size}, tr: {total_rounds}, rr: {remaining_rounds}");

        Ok(total_rounds)
    }
}

struct Field {
    /**
     * lines[0] is the bottom of the pit
     * lines[len-1] is the top of the pit
     */
    lines: Vec<u8>,
}

impl Field {
    fn new() -> Self {
        Field { lines: vec![] }
    }
    /**
     * offset=0 means the bottom-most row is aligned with the top of the existing lines
     * offset=1 means the bottom-most row is 1 below top of the existing lines
     */
    fn intersects(&self, shape: &Vec<u8>, offset: usize) -> bool {
        if offset >= self.lines.len() {
            return true;
        }
        for o in 0..=offset {
            if offset - o < shape.len() {
                let line = shape[offset - o];
                if self.lines[self.lines.len() - o - 1] & line > 0 {
                    return true;
                }
            }
        }
        return false;
    }

    fn push_shape(&mut self, shape: &Vec<u8>, offset: isize) {
        let initial_len = self.lines.len();
        for o in 0..shape.len() {
            let line = shape[o];
            let self_pos = (initial_len as isize - offset + o as isize - 1) as usize;
            // println!("{self_pos} {offset} {o}");
            if self_pos == self.lines.len() {
                self.lines.push(line);
            } else if self_pos > self.lines.len() {
                panic!("self pos out of bounds {self_pos} {}", self.lines.len());
            } else {
                self.lines[self_pos] = self.lines[self_pos] | line;
            }
        }
    }

    fn _print(&self) {
        for line in self.lines.iter().rev() {
            let binary = format!("{line:0>7b}");
            let transformed = binary
                .chars()
                .map(|c| if c == '0' { '.' } else { '#' })
                .join("");

            println!("|{transformed}|");
        }
        println!("+-------+")
    }
}
