use std::cmp;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::get_string;
use crate::{Day, Parts};

#[derive(Debug, Clone)]
pub enum GridState {
    Floor,
    Empty,
    Occupied,
}

impl fmt::Display for GridState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Floor => ".",
                Self::Empty => "L",
                Self::Occupied => "#",
            }
        )
    }
}

#[derive(Debug)]
pub struct GridMapError;

impl FromStr for GridState {
    type Err = GridMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Floor),
            "L" => Ok(Self::Empty),
            "#" => Ok(Self::Occupied),
            _ => Err(GridMapError),
        }
    }
}

type GridMapType = HashMap<(usize, usize), GridState>;

#[derive(Clone)]
pub struct GridMap {
    pub map: GridMapType,
    pub rows: usize,
    pub cols: usize,
}

impl fmt::Display for GridMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row_idx in 0..self.rows {
            writeln!(
                f,
                "{}",
                (0..self.cols)
                    .into_iter()
                    .map(|col_idx| self.map.get(&(col_idx, row_idx)).unwrap().to_string())
                    .collect::<Vec<String>>()
                    .join("")
            )?
        }
        Ok(())
    }
}

impl FromStr for GridMap {
    type Err = GridMapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid_map = GridMapType::new();
        for (x, y, v) in s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, c)| (x, y, c.to_string()))
            })
            .flatten()
        {
            grid_map.insert((x, y), v.parse()?);
        }
        Ok(GridMap {
            map: grid_map,
            rows: s.lines().count(),
            cols: s.lines().next().unwrap().chars().count(),
        })
    }
}

impl GridMap {
    pub fn count_occupied(&self) -> usize {
        self.map
            .values()
            .filter(|&v| matches!(v, GridState::Occupied))
            .count()
    }

    pub fn solve_part1(&mut self) {
        let grid_map = self;
        loop {
            let prev_grid_map = grid_map.clone();
            for row_idx in 0..grid_map.rows {
                for col_idx in 0..grid_map.cols {
                    let grid_pos: &GridState = prev_grid_map.map.get(&(col_idx, row_idx)).unwrap();
                    match grid_pos {
                        GridState::Empty | GridState::Occupied => {
                            let mut occupied_count = 0;
                            for adj_row_idx in
                                row_idx.saturating_sub(1)..=cmp::min(row_idx + 1, grid_map.rows - 1)
                            {
                                for adj_col_idx in col_idx.saturating_sub(1)
                                    ..=cmp::min(col_idx + 1, grid_map.cols - 1)
                                {
                                    if adj_row_idx != row_idx || adj_col_idx != col_idx {
                                        if let GridState::Occupied = prev_grid_map
                                            .map
                                            .get(&(adj_col_idx, adj_row_idx))
                                            .unwrap()
                                        {
                                            occupied_count += 1;
                                        }
                                    }
                                }
                            }
                            let next_grid_pos: &mut GridState =
                                grid_map.map.get_mut(&(col_idx, row_idx)).unwrap();
                            match grid_pos {
                                GridState::Empty => {
                                    if occupied_count == 0 {
                                        *next_grid_pos = GridState::Occupied;
                                    }
                                }
                                GridState::Occupied => {
                                    if occupied_count >= 4 {
                                        *next_grid_pos = GridState::Empty;
                                    }
                                }
                                _ => panic!(),
                            }
                        }
                        GridState::Floor => {}
                    }
                }
            }
            // println!("{}", grid_map);
            if prev_grid_map.count_occupied() == grid_map.count_occupied() {
                break;
            }
        }
    }

    pub fn solve_part2(&mut self) {
        let grid_map = self;
        loop {
            let prev_grid_map = grid_map.clone();
            for row_idx in 0..grid_map.rows {
                for col_idx in 0..grid_map.cols {
                    let grid_pos: &GridState = prev_grid_map.map.get(&(col_idx, row_idx)).unwrap();
                    match grid_pos {
                        GridState::Empty | GridState::Occupied => {
                            let mut occupied_count = 0;
                            for (col_dir, row_dir) in [
                                (1, 0),
                                (1, 1),
                                (0, 1),
                                (-1, 1),
                                (-1, 0),
                                (-1, -1),
                                (0, -1),
                                (1, -1),
                            ]
                            .iter()
                            {
                                for delta_idx in 1..cmp::max(prev_grid_map.rows, prev_grid_map.cols)
                                {
                                    let row_chk = i64::try_from(row_idx).unwrap()
                                        + row_dir * i64::try_from(delta_idx).unwrap();
                                    let col_chk = i64::try_from(col_idx).unwrap()
                                        + col_dir * i64::try_from(delta_idx).unwrap();
                                    if row_chk < 0
                                        || row_chk >= i64::try_from(prev_grid_map.rows).unwrap()
                                        || col_chk < 0
                                        || col_chk >= i64::try_from(prev_grid_map.cols).unwrap()
                                    {
                                        break;
                                    }
                                    match prev_grid_map
                                        .map
                                        .get(&(
                                            usize::try_from(col_chk).unwrap(),
                                            usize::try_from(row_chk).unwrap(),
                                        ))
                                        .unwrap()
                                    {
                                        GridState::Empty => {
                                            break;
                                        }
                                        GridState::Occupied => {
                                            occupied_count += 1;
                                            break;
                                        }
                                        GridState::Floor => {}
                                    }
                                }
                            }
                            // println!("{}", occupied_count);
                            let next_grid_pos: &mut GridState =
                                grid_map.map.get_mut(&(col_idx, row_idx)).unwrap();
                            match grid_pos {
                                GridState::Empty => {
                                    if occupied_count == 0 {
                                        *next_grid_pos = GridState::Occupied;
                                    }
                                }
                                GridState::Occupied => {
                                    if occupied_count >= 5 {
                                        *next_grid_pos = GridState::Empty;
                                    }
                                }
                                _ => panic!(),
                            }
                        }
                        GridState::Floor => {}
                    }
                }
            }
            // println!("{}", grid_map);
            if prev_grid_map.count_occupied() == grid_map.count_occupied() {
                break;
            }
        }
    }
}

pub fn main() -> Day {
    // let init_grid_map: GridMap = EXAMPLE.parse().unwrap();
    let init_grid_map: GridMap = get_string("day11.txt").parse().unwrap();
    // println!("{}", init_grid_map);

    let mut grid_map = init_grid_map.clone();
    grid_map.solve_part1();
    let part1_occupied = grid_map.count_occupied();
    let part1_display = format!("{} occupied.", part1_occupied);

    let mut grid_map = init_grid_map;
    grid_map.solve_part2();
    let part2_occupied = grid_map.count_occupied();
    let part2_display = format!("{} occupied.", part2_occupied);

    Day {
        answers: Parts(part1_occupied.to_string(), part2_occupied.to_string()),
        display: Parts(part1_display, part2_display),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    // #[test]
    // fn test_main() {
    //     let day = main();
    //     assert_eq!(day.answers.0, "2222");
    //     assert_eq!(day.answers.1, "2032");
    // }

    #[test]
    fn test_part2() {
        let mut grid_map: GridMap = EXAMPLE.parse().unwrap();
        grid_map.solve_part2();
        assert_eq!(grid_map.count_occupied(), 26);
    }

    #[test]
    fn test_part1() {
        let mut grid_map: GridMap = EXAMPLE.parse().unwrap();
        grid_map.solve_part1();
        assert_eq!(grid_map.count_occupied(), 37);
    }
}
