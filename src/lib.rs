#[macro_use]
extern crate lazy_static;

use std::format;
use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

#[derive(Debug, Default)]
pub struct Parts(pub String, pub String);

#[derive(Debug, Default)]
pub struct Day {
    pub answers: Parts,
    pub display: Parts,
    pub visual: Option<String>,
}

pub fn get_days() -> Vec<fn() -> Day> {
    vec![
        day1::main,
        day2::main,
        day3::main,
        day4::main,
        day5::main,
        day6::main,
        day7::main,
        day8::main,
    ]
}

pub fn get_string(name: &str) -> String {
    fs::read_to_string(format!("inputs/{}", name)).expect("Something went wrong reading the file")
}
