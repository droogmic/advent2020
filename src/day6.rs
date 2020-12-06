use itertools::Itertools;
use std::collections::HashSet;

use crate::get_string;
use crate::{Day, Parts};

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

pub fn main() -> Day {
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
    let part1_answer = union_groups.iter().map(|g| g.len()).sum::<usize>();
    let part1_display = format!(
        "{} = {} ...",
        part1_answer,
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
                ('a'..='z').into_iter().collect::<HashSet<_>>(),
                |acc, person| acc.intersection(&person).copied().collect(),
            )
        })
        .collect();
    let part2_answer = intersection_groups.iter().map(|g| g.len()).sum::<usize>();
    let part2_display = format!(
        "{} = {} ...",
        part2_answer,
        intersection_groups
            .iter()
            .map(|g| String::from("'") + &g.iter().join("") + "'")
            .take(10)
            .join(" + "),
    );

    Day {
        answers: Parts(part1_answer.to_string(), part2_answer.to_string()),
        display: Parts(part1_display, part2_display),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "6809");
        assert_eq!(day.answers.1, "3394");
    }
}
