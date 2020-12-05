use std::format;
use std::fs;

pub mod day1;
pub mod day2;

pub fn get_days() -> Vec<Box<dyn Fn()>> {
    vec![Box::new(day1::main), Box::new(day2::main)]
}

pub fn get_string(name: &str) -> String {
    fs::read_to_string(format!("inputs/{}", name))
        .expect("Something went wrong reading the file")
}
