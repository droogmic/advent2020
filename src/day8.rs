use crate::get_string;
use crate::{Day, Parts};
use recap::Recap;
use serde::Deserialize;
use std::collections::HashSet;
use std::convert::TryFrom;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InstructionType {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"^(?P<typ>nop|acc|jmp) (?P<val>.\d+)$"#)]
pub struct Instruction {
    pub typ: InstructionType,
    pub val: i32,
}

pub struct MachineState {
    pub pc: u16,
    pub acc: i32,
}

pub fn get_data(input: String) -> Vec<Instruction> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn calc(instructions: &[&Instruction]) -> (bool, MachineState, HashSet<u16>) {
    let mut seen = HashSet::<u16>::new();
    let mut machine_state = MachineState { pc: 0, acc: 0 };
    let is_infinite_loop: bool = loop {
        if !seen.insert(machine_state.pc) {
            break true;
        }
        match instructions.get(usize::try_from(machine_state.pc).unwrap()) {
            Some(Instruction {
                typ: InstructionType::Nop,
                val: _,
            }) => {
                machine_state.pc += 1;
            }
            Some(Instruction {
                typ: InstructionType::Acc,
                val: v,
            }) => {
                machine_state.pc += 1;
                machine_state.acc += v;
            }
            Some(Instruction {
                typ: InstructionType::Jmp,
                val: v,
            }) => {
                machine_state.pc = u16::try_from(i32::from(machine_state.pc) + v).unwrap();
            }
            None => {
                break false;
            }
        }
    };
    (is_infinite_loop, machine_state, seen)
}

pub fn main() -> Day {
    let instructions = get_data(get_string("day8.txt"));
    // println!("{:#?}", instructions[0]);

    let (is_infinite_loop, machine_state, seen) = calc(
        instructions
            .iter()
            .collect::<Vec<&Instruction>>()
            .as_slice(),
    );
    assert!(is_infinite_loop);
    let part1_acc = machine_state.acc;
    let part1_display = format!(
        "Accumulator is {} after {} instructions before looping again.",
        part1_acc,
        seen.len()
    );

    let modified_instructions = (0..instructions.len())
        .rev()
        .filter_map(|idx| match instructions.get(idx) {
            Some(Instruction {
                typ: InstructionType::Nop,
                val: v,
            }) if *v != 0 => Some((
                idx,
                Instruction {
                    typ: InstructionType::Jmp,
                    val: *v,
                },
            )),
            Some(Instruction {
                typ: InstructionType::Jmp,
                val: v,
            }) => Some((
                idx,
                Instruction {
                    typ: InstructionType::Nop,
                    val: *v,
                },
            )),
            _ => None,
        })
        .collect::<Vec<(usize, Instruction)>>();
    let alternatives_solutions = modified_instructions
        .iter()
        .map(|(idx, modified)| {
            instructions
                .iter()
                .take(*idx)
                .chain(std::iter::once(modified))
                .chain(instructions.iter().skip(idx + 1))
                .collect::<Vec<&Instruction>>()
        })
        .map(|instructions| calc(instructions.as_slice()))
        .collect::<Vec<(bool, MachineState, HashSet<u16>)>>();
    let (machine_state, seen) = alternatives_solutions
        .iter()
        .filter_map(|(is_infinite_loop, machine_state, seen)| {
            if *is_infinite_loop {
                None
            } else {
                Some((machine_state, seen))
            }
        })
        .next()
        .unwrap();
    let part2_acc = machine_state.acc;
    let part2_display = format!(
        "Accumulator is {} after {} instructions before exiting.",
        part2_acc,
        seen.len()
    );

    Day {
        answers: Parts(part1_acc.to_string(), part2_acc.to_string()),
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
        assert_eq!(day.answers.0, "1671");
        assert_eq!(day.answers.1, "892");
    }

    #[test]
    fn test_parse_instruction() {
        let instruction: Instruction = get_data("acc -99".to_string()).drain(..1).next().unwrap();
        assert_eq!(instruction.typ, InstructionType::Acc);
        assert_eq!(instruction.val, -99);
    }
}
