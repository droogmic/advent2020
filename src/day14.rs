use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;

use crate::get_string;
use crate::{Day, Parts};

// const BITS: usize = 36;
// const SIZE: usize = 0b1 << 36;
type M = u64;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"^mem\[(?P<addr>\d+)\] = (?P<val>\d+)$"#)]
pub struct MemSet {
    addr: M,
    val: M,
}

impl std::fmt::Display for MemSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mem[{}] = {}", self.addr, self.val)
    }
}

#[derive(Debug, Deserialize)]
pub enum MaskVal {
    V0,
    V1,
    X,
}

impl std::fmt::Display for MaskVal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::V0 => "0",
                Self::V1 => "1",
                Self::X => "X",
            }
        )
    }
}

#[derive(Debug)]
pub struct ParseError;

impl std::str::FromStr for MaskVal {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Self::V0),
            "1" => Ok(Self::V1),
            "X" => Ok(Self::X),
            _ => Err(ParseError),
        }
    }
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"^mask = (?P<mask>(?:0|1|X)+)$"#)]
pub struct MaskStr {
    mask: String,
}

#[derive(Debug)]
pub struct MaskBlock {
    mask: Vec<MaskVal>,
    writes: Vec<MemSet>,
}

impl std::fmt::Display for MaskBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "mask = {}",
            self.mask
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>()
                .join("")
        )?;
        for write in &self.writes {
            writeln!(f, "{}", write)?;
        }
        Ok(())
    }
}

pub enum Mask {
    Or(M),
    And(M),
    Floating(M),
}

impl MaskBlock {
    fn get_val_masks(&self) -> Vec<Mask> {
        self.mask
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(idx, m)| match m {
                MaskVal::X => None,
                MaskVal::V0 => Some(Mask::And(!(0b1 << idx))),
                MaskVal::V1 => Some(Mask::Or(0b1 << idx)),
            })
            .collect()
    }
    fn get_addr_masks(&self) -> Vec<Mask> {
        self.mask
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(idx, m)| match m {
                MaskVal::X => Some(Mask::Floating(0b1 << idx)),
                MaskVal::V0 => None,
                MaskVal::V1 => Some(Mask::Or(0b1 << idx)),
            })
            .collect()
    }
}

impl MemSet {
    fn get_val(&self, masks: &[Mask]) -> M {
        masks.iter().fold(self.val, |acc, m| match m {
            Mask::Or(m) => acc | m,
            Mask::And(m) => acc & m,
            _ => panic!(),
        })
    }
    fn get_addr(&self, masks: &[Mask]) -> Vec<M> {
        let addr = masks.iter().fold(self.addr, |acc, m| match m {
            Mask::Or(m) => acc | m,
            Mask::And(m) => acc & m,
            Mask::Floating(_m) => acc,
        });
        masks
            .iter()
            .filter_map(|m| match m {
                Mask::Floating(m) => Some(vec![0b0, *m].into_iter()),
                _ => None,
            })
            .multi_cartesian_product()
            .map(|floats| floats.iter().fold(addr, |acc, f| acc ^ f))
            .collect()
    }
}

pub fn get_data(input: &str) -> Vec<MaskBlock> {
    let mut mask_blocks = Vec::new();
    for line in input.lines() {
        if line.starts_with("mask") {
            mask_blocks.push(MaskBlock {
                mask: line
                    .parse::<MaskStr>()
                    .unwrap()
                    .mask
                    .chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .collect(),
                writes: Vec::new(),
            });
        } else if line.starts_with("mem") {
            mask_blocks
                .last_mut()
                .unwrap()
                .writes
                .push(line.parse().unwrap());
        } else {
            panic!();
        }
    }
    mask_blocks
}

// const EXAMPLE_1: &str = "\
// mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
// mem[8] = 11
// mem[7] = 101
// mem[8] = 0";

// const EXAMPLE_2: &str = "\
// mask = 000000000000000000000000000000X1001X
// mem[42] = 100
// mask = 00000000000000000000000000000000X0XX
// mem[26] = 1";

pub fn main() -> Day {
    // let blocks: Vec<MaskBlock> = get_data(EXAMPLE_1);
    let blocks: Vec<MaskBlock> = get_data(&get_string("day14.txt"));

    let mut memory: HashMap<M, M> = HashMap::new();
    for block in &blocks {
        // println!("{}", block);
        let masks = block.get_val_masks();
        for write in &block.writes {
            memory.insert(write.addr, write.get_val(&masks));
            // println!("memory[{}] = {}", write.addr, write.get_val(&masks));
        }
    }
    let part1_sum = memory.values().sum::<M>();
    let part1_display = format!("Sum: {}", part1_sum);

    // let blocks: Vec<MaskBlock> = get_data(EXAMPLE_2);

    let mut memory: HashMap<M, M> = HashMap::new();
    for block in &blocks {
        // println!("{}", block);
        let masks = block.get_addr_masks();
        for write in &block.writes {
            for addr in write.get_addr(&masks) {
                memory.insert(addr, write.val);
                // println!("memory[{}] = {}", addr, write.val);
            }
        }
    }
    let part2_sum = memory.values().sum::<M>();
    let part2_display = format!("Sum: {}", part2_sum);

    Day {
        answers: Parts(part1_sum.to_string(), part2_sum.to_string()),
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
        assert_eq!(day.answers.0, "15919415426101");
        assert_eq!(day.answers.1, "3443997590975");
    }
}
