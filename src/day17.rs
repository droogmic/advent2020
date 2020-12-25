use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryFrom;

use crate::get_string;
use crate::{Day, Parts};

#[derive(Clone, Hash, PartialEq, Eq)]
struct Cood {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Cood {
    fn len() -> usize {
        4
    }

    fn from_slice(s: &[i64]) -> Cood {
        Cood {
            x: *s.get(0).unwrap(),
            y: *s.get(1).unwrap(),
            z: *s.get(2).unwrap_or(&0),
            w: *s.get(3).unwrap_or(&0),
        }
    }

    fn get(&self, index: usize) -> Option<i64> {
        Some(match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => return None,
        })
    }
}

impl IntoIterator for Cood {
    type Item = i64;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.x, self.y, self.z].into_iter()
    }
}

impl<'a> IntoIterator for &'a Cood {
    type Item = i64;
    type IntoIter = CoodIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CoodIterator {
            cood: self,
            index: 0,
        }
    }
}

struct CoodIterator<'a> {
    cood: &'a Cood,
    index: usize,
}

impl<'a> Iterator for CoodIterator<'a> {
    type Item = i64;
    fn next(&mut self) -> Option<i64> {
        let result = self.cood.get(self.index);
        self.index += 1;
        result
    }
}

impl std::fmt::Display for Cood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{{}}}",
            self.into_iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}

impl std::ops::Add for Cood {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[derive(Clone)]
enum CubeState {
    Active,
    Inactive,
}

#[derive(Debug)]
pub struct ParseError;

impl std::str::FromStr for CubeState {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Inactive),
            "#" => Ok(Self::Active),
            _ => Err(ParseError),
        }
    }
}

impl std::fmt::Display for CubeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Inactive => ".",
                Self::Active => "#",
            }
        )
    }
}

struct PocketDimension {
    grid: HashMap<Cood, CubeState>,
}

impl PocketDimension {
    fn min_max(&self) -> (Cood, Cood) {
        assert!(
            self.grid.keys().count() >= 1,
            "{} >= 1",
            self.grid.keys().count()
        );
        (
            Cood::from_slice(
                &(0..Cood::len())
                    .map(|idx| {
                        self.grid
                            .keys()
                            .map(|cood| cood.into_iter().nth(idx).unwrap())
                            .min()
                            .unwrap()
                    })
                    .collect::<Vec<i64>>(),
            ),
            Cood::from_slice(
                &(0..Cood::len())
                    .map(|idx| {
                        self.grid
                            .keys()
                            .map(|cood| cood.into_iter().nth(idx).unwrap())
                            .max()
                            .unwrap()
                    })
                    .collect::<Vec<i64>>(),
            ),
        )
    }

    fn from_initial_state(s: &str) -> PocketDimension {
        let mut grid: HashMap<Cood, CubeState> = HashMap::new();
        s.lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (x, y, c.to_string()))
            })
            .flatten()
            .for_each(|(x, y, v)| {
                grid.insert(
                    Cood {
                        x: i64::try_from(x).unwrap(),
                        y: i64::try_from(y).unwrap(),
                        z: 0,
                        w: 0,
                    },
                    v.parse().unwrap(),
                );
            });
        PocketDimension { grid }
    }

    fn step(
        &mut self,
        dimensions: usize,
        rules: fn(cube_state: CubeState, active_count: usize) -> CubeState,
    ) {
        let min_max = self.min_max();
        let prev_grid = &self.grid;
        let mut next_grid: HashMap<Cood, CubeState> = HashMap::new();
        let ranges = (0..dimensions)
            .map(|idx| min_max.0.get(idx).unwrap() - 1..=min_max.1.get(idx).unwrap() + 1);

        for cood in ranges.into_iter().multi_cartesian_product() {
            let cood = Cood::from_slice(&cood);
            let active_count = std::iter::repeat(-1..=1)
                .take(dimensions)
                .multi_cartesian_product()
                .filter(|delta| {
                    delta.iter().any(|&d| d != 0)
                        && matches!(
                            prev_grid.get(&(cood.clone() + Cood::from_slice(delta))),
                            Some(CubeState::Active)
                        )
                })
                .count();
            let next_state = rules(
                prev_grid.get(&cood).cloned().unwrap_or(CubeState::Inactive),
                active_count,
            );
            if let CubeState::Active = next_state {
                next_grid.insert(cood, next_state);
            }
        }
        self.grid = next_grid;
    }

    fn count_active(&self) -> usize {
        self.grid
            .values()
            .filter(|v| matches!(v, CubeState::Active))
            .count()
    }
}

fn rules(cube_state: CubeState, active_count: usize) -> CubeState {
    match (cube_state, active_count) {
        (CubeState::Active, 2..=3) => CubeState::Active,
        (CubeState::Active, _) => CubeState::Inactive,
        (CubeState::Inactive, 3) => CubeState::Active,
        (CubeState::Inactive, _) => CubeState::Inactive,
    }
}

impl std::fmt::Display for PocketDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x_min = self.grid.keys().map(|cood| cood.x).min().unwrap();
        let x_max = self.grid.keys().map(|cood| cood.x).max().unwrap();
        let y_min = self.grid.keys().map(|cood| cood.y).min().unwrap();
        let y_max = self.grid.keys().map(|cood| cood.y).max().unwrap();
        let z_min = self.grid.keys().map(|cood| cood.z).min().unwrap();
        let z_max = self.grid.keys().map(|cood| cood.z).max().unwrap();
        for z_idx in z_min..=z_max {
            write!(
                f,
                "z={:<width$} ",
                z_idx,
                width = usize::try_from(x_max.saturating_sub(x_min).saturating_sub(1)).unwrap()
            )?;
        }
        writeln!(f)?;
        for y_idx in y_min..=y_max {
            for z_idx in z_min..=z_max {
                for x_idx in x_min..=x_max {
                    write!(
                        f,
                        "{}",
                        self.grid
                            .get(&Cood {
                                x: x_idx,
                                y: y_idx,
                                z: z_idx,
                                w: 0,
                            })
                            .unwrap_or(&CubeState::Inactive)
                    )?;
                }
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn main() -> Day {
    let mut pocket_dimension = PocketDimension::from_initial_state(&get_string("day17.txt"));
    for _ in 0..6 {
        // println!("{}", pocket_dimension);
        pocket_dimension.step(3, rules);
    }
    // println!("{}", pocket_dimension);
    let part1 = pocket_dimension.count_active();
    let part1_display = format!("{} active cubes", pocket_dimension.count_active());

    let mut pocket_dimension = PocketDimension::from_initial_state(&get_string("day17.txt"));
    for _ in 0..6 {
        // println!("{}", pocket_dimension);
        pocket_dimension.step(4, rules);
    }
    // println!("{}", pocket_dimension);
    let part2 = pocket_dimension.count_active();
    let part2_display = format!("{} active cubes", pocket_dimension.count_active());

    Day {
        answers: Parts(part1.to_string(), part2.to_string()),
        display: Parts(part1_display, part2_display),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
.#.
..#
###";

    // #[test]
    // fn test_main() {
    //     let day = main();
    //     assert_eq!(day.answers.0, "247");
    //     assert_eq!(day.answers.1, "1392");
    // }

    #[test]
    fn test_example() {
        let mut pocket_dimension = PocketDimension::from_initial_state(EXAMPLE);
        for _ in 0..6 {
            pocket_dimension.step(3, rules);
        }
        assert_eq!(pocket_dimension.count_active(), 112);
    }
}
