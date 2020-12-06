use std::fmt;

use colored::*;
use itertools::Itertools;

use crate::get_string;
use crate::{Day, Parts};

#[derive(Clone)]
pub enum Pos {
    Open,
    Tree,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pos::Open => '.',
                Pos::Tree => '#',
            }
        )
    }
}

pub struct Map {
    columns: Vec<Vec<Pos>>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.columns.first().unwrap().len() {
            let pattern = self
                .columns
                .iter()
                .map(|col| col.get(row).unwrap().to_string())
                .fold(String::new(), |a, b| a + &b);
            writeln!(f, "{}{}{}", pattern.blue(), pattern, pattern.blue())?
        }
        Ok(())
    }
}

pub struct Xy {
    pub x: usize,
    pub y: usize,
}

pub fn get_data(input: String) -> Map {
    let mut map = Map {
        columns: vec![vec![]; input.lines().next().unwrap().len()],
    };
    for line in input.lines() {
        for (idx, character) in line.chars().enumerate() {
            map.columns.get_mut(idx).unwrap().push(match character {
                '.' => Pos::Open,
                '#' => Pos::Tree,
                _ => panic!("Unexpected map"),
            });
        }
    }
    map
}

pub fn traverse(map: &Map, trajectory: Xy) -> usize {
    let mut pos = Xy { x: 0, y: 0 };
    let mut tree_count: usize = 0;
    while pos.y < map.columns.first().unwrap().len() {
        if let Pos::Tree = map.columns[pos.x][pos.y] {
            tree_count += 1
        }
        pos.y += trajectory.y;
        pos.x = (pos.x + trajectory.x) % map.columns.len();
    }
    tree_count
}

pub fn main() -> Day {
    let map = get_data(get_string("day3.txt"));
    // println!("{}", map);

    let part1_answer = traverse(&map, Xy { x: 3, y: 1 });
    let part1_display = format!("{} trees", part1_answer);

    let trees: Vec<usize> = vec![
        Xy { x: 1, y: 1 },
        Xy { x: 3, y: 1 },
        Xy { x: 5, y: 1 },
        Xy { x: 7, y: 1 },
        Xy { x: 1, y: 2 },
    ]
    .into_iter()
    .map(|trajectory| traverse(&map, trajectory))
    .collect();
    let part2_answer = trees.to_vec().into_iter().product::<usize>();
    let part2_display = format!(
        "{} = {} trees",
        part2_answer,
        trees
            .to_vec()
            .into_iter()
            .map(|v| v.to_string())
            .join(" × "),
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
        assert_eq!(day.answers.0, "156");
        assert_eq!(day.answers.1, "3521829480");
    }
}
