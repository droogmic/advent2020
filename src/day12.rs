// use std::cmp;
// use std::collections::HashMap;
// use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::get_string;
use crate::{Day, Parts};

// #[derive(Debug, Clone)]
// pub enum Instruction {
//     North(u32),
//     South(u32),
//     East(u32),
//     West(u32),
//     Left(u32),
//     Right(u32),
//     Forward(u32),
// }

#[derive(Clone, Copy)]
pub enum InstructionAction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl fmt::Display for InstructionAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => "N",
                Self::South => "S",
                Self::East => "E",
                Self::West => "W",
                Self::Left => "L",
                Self::Right => "R",
                Self::Forward => "F",
            }
        )
    }
}

#[derive(Debug)]
pub struct ParseError;

impl FromStr for InstructionAction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Self::North),
            "S" => Ok(Self::South),
            "E" => Ok(Self::East),
            "W" => Ok(Self::West),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            "F" => Ok(Self::Forward),
            _ => Err(ParseError),
        }
    }
}

#[derive(Clone)]
pub struct Instruction {
    pub action: InstructionAction,
    pub val: u32,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.action, self.val)
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = s.chars().next().unwrap().to_string().parse()?;
        let val = s.get(1..).unwrap().parse().map_err(|_| ParseError)?;
        Ok(Self { action, val })
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::North => "N",
                Self::South => "S",
                Self::East => "E",
                Self::West => "W",
            }
        )
    }
}

impl Direction {
    pub fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
    pub fn opposite(&self) -> Self {
        self.left().left()
    }
    pub fn right(&self) -> Self {
        self.left().left().left()
    }
}

#[derive(Clone, Copy)]
pub struct Pos {
    east: i64,
    north: i64,
}

#[derive(Clone)]
pub struct Ferry {
    pub facing: Direction,
    pub position: Pos,
    pub waypoint: Pos,
}

impl fmt::Display for Ferry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({},{}) ({}, {})",
            self.facing,
            self.position.east,
            self.position.north,
            self.waypoint.east,
            self.waypoint.north
        )
    }
}

impl Ferry {
    pub fn move_part1(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction {
                action: InstructionAction::North,
                val,
            } => self.position.north += i64::from(*val),
            Instruction {
                action: InstructionAction::South,
                val,
            } => self.position.north -= i64::from(*val),
            Instruction {
                action: InstructionAction::East,
                val,
            } => self.position.east += i64::from(*val),
            Instruction {
                action: InstructionAction::West,
                val,
            } => self.position.east -= i64::from(*val),
            Instruction {
                action: InstructionAction::Left,
                val: 90,
            }
            | Instruction {
                action: InstructionAction::Right,
                val: 270,
            } => self.facing = self.facing.left(),
            Instruction {
                action: InstructionAction::Right,
                val: 90,
            }
            | Instruction {
                action: InstructionAction::Left,
                val: 270,
            } => self.facing = self.facing.right(),
            Instruction {
                action: InstructionAction::Left,
                val: 180,
            }
            | Instruction {
                action: InstructionAction::Right,
                val: 180,
            } => self.facing = self.facing.opposite(),
            Instruction {
                action: InstructionAction::Forward,
                val,
            } => match self.facing {
                Direction::North => self.position.north += i64::from(*val),
                Direction::South => self.position.north -= i64::from(*val),
                Direction::East => self.position.east += i64::from(*val),
                Direction::West => self.position.east -= i64::from(*val),
            },
            i => panic!("{}", i),
        }
    }

    pub fn move_part2(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction {
                action: InstructionAction::North,
                val,
            } => self.waypoint.north += i64::from(*val),
            Instruction {
                action: InstructionAction::South,
                val,
            } => self.waypoint.north -= i64::from(*val),
            Instruction {
                action: InstructionAction::East,
                val,
            } => self.waypoint.east += i64::from(*val),
            Instruction {
                action: InstructionAction::West,
                val,
            } => self.waypoint.east -= i64::from(*val),
            Instruction {
                action: a @ InstructionAction::Right,
                val,
            }
            | Instruction {
                action: a @ InstructionAction::Left,
                val,
            } => {
                match val {
                    90 | 180 | 270 => {}
                    _ => panic!(),
                }
                let count = if let InstructionAction::Right = a {
                    val / 90
                } else {
                    4 - (val / 90)
                };
                for _ in 0..count {
                    let prev_waypoint = self.waypoint;
                    self.waypoint.east = prev_waypoint.north;
                    self.waypoint.north = -prev_waypoint.east;
                }
            }
            Instruction {
                action: InstructionAction::Forward,
                val,
            } => {
                self.position.north += i64::from(*val) * self.waypoint.north;
                self.position.east += i64::from(*val) * self.waypoint.east;
            }
            // i => panic!("{}", i),
        }
    }
}

// const EXAMPLE: &str = "\
// F10
// N3
// F7
// R90
// F11";

pub fn main() -> Day {
    // let instructions: Vec<Instruction> = EXAMPLE.lines().map(|l| l.parse().unwrap()).collect();
    let instructions: Vec<Instruction> = get_string("day12.txt")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let init_ferry = Ferry {
        facing: Direction::East,
        position: Pos { east: 0, north: 0 },
        waypoint: Pos { east: 10, north: 1 },
    };
    let mut ferry = init_ferry.clone();
    // println!("{}", ferry);
    for instruction in instructions.iter() {
        ferry.move_part1(&instruction);
        // println!("{}", ferry);
    }
    let part1 = ferry.position.east.abs() + ferry.position.north.abs();
    let part1_display = format!("Manhattan distance: {}", part1);

    let mut ferry = init_ferry;
    // println!("{}", ferry);
    for instruction in instructions.iter() {
        ferry.move_part2(&instruction);
        // println!("{}", ferry);
    }
    let part2 = ferry.position.east.abs() + ferry.position.north.abs();
    let part2_display = format!("Manhattan distance: {}", part2);

    Day {
        answers: Parts(part1.to_string(), part2.to_string()),
        display: Parts(part1_display, part2_display),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "858");
        assert_eq!(day.answers.1, "39140");
    }
}
