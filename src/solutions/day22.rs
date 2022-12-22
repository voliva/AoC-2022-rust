use itertools::Itertools;

use super::Solver;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Problem;

#[derive(PartialEq, Debug)]
pub enum Tile {
    Open,
    Solid,
    Void,
}

pub enum Instruction {
    Move(usize),
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

#[derive(Debug)]
struct Player {
    facing: Direction,
    position: (usize, usize),
}

type Field = Vec<Vec<Tile>>;

impl Player {
    fn new(field: &Field) -> Self {
        let position = (
            0,
            field[0]
                .iter()
                .find_position(|t| **t != Tile::Void)
                .unwrap()
                .0,
        );
        Self {
            position,
            facing: Direction::Right,
        }
    }
    fn apply_instr(&mut self, field: &Field, instruction: &Instruction) {
        match instruction {
            Instruction::Left => self.rotate_left(),
            Instruction::Right => self.rotate_right(),
            Instruction::Move(quantity) => self.advance(field, *quantity),
        }
    }
    fn apply_instr_2(&mut self, field: &Field, instruction: &Instruction) {
        match instruction {
            Instruction::Left => self.rotate_left(),
            Instruction::Right => self.rotate_right(),
            Instruction::Move(quantity) => self.advance_2(field, *quantity),
        }
    }
    fn rotate_left(&mut self) {
        self.facing = match self.facing {
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
        }
    }
    fn rotate_right(&mut self) {
        self.facing = match self.facing {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            Direction::Up => Direction::Right,
        }
    }
    fn advance(&mut self, field: &Field, quantity: usize) {
        let delta = match self.facing {
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
        };

        for _ in 0..quantity {
            let mut next_row = self.position.0;
            loop {
                if next_row == 0 && delta.0 < 0 {
                    next_row = field.len() - 1;
                    // field
                    //     .iter()
                    //     .enumerate()
                    //     .rev()
                    //     .find(|(_, row)| {
                    //         row.len() > self.position.1 && row[self.position.1] != Tile::Void
                    //     })
                    //     .unwrap()
                    //     .0
                } else {
                    next_row = (next_row as isize + delta.0) as usize % field.len();
                };

                if field[next_row].len() > self.position.1
                    && field[next_row][self.position.1] != Tile::Void
                {
                    break;
                }
            }

            let mut next_col = self.position.1;
            loop {
                if next_col == 0 && delta.1 < 0 {
                    next_col = field[next_row].len() - 1;
                } else {
                    next_col =
                        (next_col as isize + delta.1) as usize % field[self.position.0].len();
                };
                // println!("test b {} {}", self.position.0, next_col);
                if field[self.position.0].len() > next_col
                    && field[self.position.0][next_col] != Tile::Void
                {
                    break;
                }
            }

            if field[next_row][next_col] == Tile::Solid {
                return;
            }
            self.position = (next_row, next_col);
        }
    }
    fn advance_2(&mut self, field: &Field, quantity: usize) {
        /*
         * Layout
         *  EF
         *  D
         * BC
         * A
         */
        for _ in 0..quantity {
            let delta = match self.facing {
                Direction::Down => (1, 0),
                Direction::Left => (0, -1),
                Direction::Right => (0, 1),
                Direction::Up => (-1, 0),
            };

            let next_unwrapped_pos = (
                self.position.0 as isize + delta.0,
                self.position.1 as isize + delta.1,
            );

            let mut next_facing = self.facing.clone();
            let next_pos: (usize, usize) = if next_unwrapped_pos.0 < 0 {
                // we could be on E or F
                if next_unwrapped_pos.1 < 100 {
                    // E wraps to A left
                    next_facing = Direction::Right;
                    ((next_unwrapped_pos.1 - 50 + 150) as usize, 0)
                } else {
                    // F wraps to A bottom
                    next_facing = Direction::Up;
                    (field.len() - 1, (next_unwrapped_pos.1 - 100) as usize)
                }
            } else if next_unwrapped_pos.0 < 100
                && next_unwrapped_pos.1 < 50
                && next_unwrapped_pos.0 >= 50
            {
                // We're on B-D
                if self.position.0 >= 100 {
                    // We're on B, wrap to D
                    next_facing = Direction::Right;
                    (50 + next_unwrapped_pos.1 as usize, 50)
                } else {
                    // We're on D, wrap to B
                    next_facing = Direction::Down;
                    (100, next_unwrapped_pos.0 as usize - 50)
                }
            } else if next_unwrapped_pos.1 >= 100 && next_unwrapped_pos.0 >= 100 {
                // We're on C, wrap to F
                next_facing = Direction::Left;
                ((49 - (next_unwrapped_pos.0 - 100)) as usize, 149)
            } else if next_unwrapped_pos.0 >= 50 && next_unwrapped_pos.1 >= 100 {
                // We're on F-D
                if self.position.0 < 50 {
                    // We're on F, wrap to D
                    next_facing = Direction::Left;
                    (next_unwrapped_pos.1 as usize - 100 + 50, 99)
                } else {
                    // We're on D, wrap to F
                    next_facing = Direction::Up;
                    (49, next_unwrapped_pos.0 as usize - 50 + 100)
                }
            } else if next_unwrapped_pos.0 >= 150 && next_unwrapped_pos.1 >= 50 {
                // We're on A-C
                if self.position.0 < 150 {
                    // We're on C, wrap to A
                    next_facing = Direction::Left;
                    (next_unwrapped_pos.1 as usize - 50 + 150, 49)
                } else {
                    // We're on A, wrap to C
                    next_facing = Direction::Up;
                    (149, next_unwrapped_pos.0 as usize - 150 + 50)
                }
            } else if next_unwrapped_pos.0 >= 200 {
                // We're on A, wrap to F
                next_facing = Direction::Down;
                (0, next_unwrapped_pos.1 as usize + 100)
            } else if next_unwrapped_pos.1 < 0 {
                // We're on B or A
                if next_unwrapped_pos.0 < 150 {
                    next_facing = Direction::Right;
                    // We're on B, wrap to E
                    ((49 - (next_unwrapped_pos.0 - 100)) as usize, 50)
                } else {
                    next_facing = Direction::Down;
                    // We're on A, wrap to E
                    (0, next_unwrapped_pos.0 as usize - 150 + 50)
                }
            } else if next_unwrapped_pos.1 < 50 && next_unwrapped_pos.0 < 50 {
                // We're on E wrap to B
                next_facing = Direction::Right;
                ((149 - next_unwrapped_pos.0) as usize, 0)
            } else if next_unwrapped_pos.1 >= 150 {
                // We're on F, wrap to C
                next_facing = Direction::Left;
                ((149 - next_unwrapped_pos.0) as usize, 99)
            } else {
                (next_unwrapped_pos.0 as usize, next_unwrapped_pos.1 as usize)
            };

            if field[next_pos.0][next_pos.1] == Tile::Void {
                panic!("out of bounds!");
            }
            if field[next_pos.0][next_pos.1] == Tile::Solid {
                return;
            }
            self.facing = next_facing;
            self.position = next_pos;
        }
    }
}

impl Solver for Problem {
    type Input = (Field, Vec<Instruction>);
    type Output1 = usize;
    type Output2 = usize;

    fn read_input(&self, file_reader: BufReader<&File>) -> Self::Input {
        let lines = file_reader.lines().map(|x| x.unwrap()).collect_vec();
        let field = lines
            .iter()
            .take(lines.len() - 2)
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        ' ' => Tile::Void,
                        '.' => Tile::Open,
                        '#' => Tile::Solid,
                        _ => panic!("unknown char {c}"),
                    })
                    .collect_vec()
            })
            .collect_vec();

        let mut instructions = vec![];
        let instr_line = &lines[lines.len() - 1];
        let mut head = 0;
        while head < instr_line.len() {
            let next_char = instr_line[head..]
                .char_indices()
                .find(|(_, c)| *c == 'L' || *c == 'R')
                .unwrap_or((instr_line.len() - head, '.'))
                .0
                + head;
            let move_amount = instr_line[head..next_char].parse().unwrap();
            instructions.push(Instruction::Move(move_amount));

            if next_char < instr_line.len() {
                let instr = match &instr_line[next_char..next_char + 1] {
                    "L" => Instruction::Left,
                    "R" => Instruction::Right,
                    _ => panic!("unknown next_char"),
                };
                instructions.push(instr);
            }
            head = next_char + 1;
        }

        (field, instructions)
    }

    fn solve_first(&self, (field, instructions): &Self::Input) -> Result<Self::Output1, String> {
        let mut player = Player::new(field);

        for instruction in instructions {
            player.apply_instr(field, instruction);
        }

        Ok(1000 * (player.position.0 + 1) + 4 * (player.position.1 + 1) + player.facing as usize)
    }

    fn solve_second(&self, (field, instructions): &Self::Input) -> Result<Self::Output2, String> {
        let mut player = Player::new(field);

        for instruction in instructions {
            player.apply_instr_2(field, instruction);
        }

        Ok(1000 * (player.position.0 + 1) + 4 * (player.position.1 + 1) + player.facing as usize)
    }
}
