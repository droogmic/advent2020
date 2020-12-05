use crate::get_string;

use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?P<lower>\d+)-(?P<upper>\d+) (?P<character>\S): (?P<password>.+)"#)]
pub struct PasswordEntry {
    lower: usize,
    upper: usize,
    character: char,
    password: String,
}

pub fn get_data(input: String) -> Vec<PasswordEntry> {
    input
        .lines()
        .map(|line| line.parse().expect("bad input"))
        .collect()
}

pub fn main() {
    let database = get_data(get_string("day2.txt"));
    // println!("{:#?}", database.first().unwrap());

    let part_1_valid_count = database
        .iter()
        .filter(|entry| {
            let count = entry
                .password
                .chars()
                .into_iter()
                .filter(|c| c == &entry.character)
                .count();
            (entry.lower <= count) && (count <= entry.upper)
        })
        .count();
    println!("Part 1: {} valid entries", part_1_valid_count);

    let part_2_valid_count = database
        .iter()
        .filter(|entry| {
            let chars: Vec<char> = entry.password.chars().collect();
            let pos1: bool = chars[entry.lower - 1] == entry.character;
            let pos2: bool = chars[entry.upper - 1] == entry.character;
            pos1 ^ pos2
        })
        .count();
    println!("Part 2: {} valid entries", part_2_valid_count);
}
