use std::format;
use std::fs;

pub mod day1;

pub fn get_string(name: &str) -> String {
    let contents = fs::read_to_string(format!("inputs/{}", name))
        .expect("Something went wrong reading the file");
    contents
}
