use itertools::Itertools;
use std::collections::HashSet;

use crate::get_string;

pub fn get_data(input: String) -> Vec<Vec<HashSet<char>>> {
    input
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|group| {
            group
                .iter()
                .map(|person| person.chars().collect::<HashSet<char>>())
                .collect()
        })
        .collect()
}

pub fn main() {
    let groups = get_data(get_string("day6.txt"));
    // println!("{:#?}", groups[0]);

    let union_groups: Vec<HashSet<char>> = groups
        .iter()
        .map(|group| {
            group.iter().fold(HashSet::<char>::new(), |acc, person| {
                acc.union(&person).copied().collect()
            })
        })
        .collect();
    println!(
        "Part 1: {} = {} ...",
        union_groups.iter().map(|g| g.len()).sum::<usize>(),
        union_groups
            .iter()
            .map(|g| String::from("'") + &g.iter().join("") + "'")
            .take(10)
            .join(" + "),
    );

    let intersection_groups: Vec<HashSet<char>> = groups
        .iter()
        .map(|group| {
            group.iter().fold(
                (b'a'..=b'z').map(|c| c as char).collect::<HashSet<_>>(),
                |acc, person| acc.intersection(&person).copied().collect(),
            )
        })
        .collect();
    println!(
        "Part 2: {} = {} ...",
        intersection_groups.iter().map(|g| g.len()).sum::<usize>(),
        intersection_groups
            .iter()
            .map(|g| String::from("'") + &g.iter().join("") + "'")
            .take(10)
            .join(" + "),
    );
}
