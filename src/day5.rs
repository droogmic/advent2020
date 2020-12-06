use crate::get_string;
use crate::{Day, Parts};

pub fn get_data(input: String) -> Vec<String> {
    input.lines().map(String::from).collect()
}

#[derive(Debug, Clone)]
pub enum SplitDir {
    F,
    B,
    L,
    R,
}

const ROWS: usize = 128;
const SEATS_IN_ROW: usize = 8;

fn split_axis((x, y): (usize, usize), dir: SplitDir) -> (usize, usize) {
    let y = y + 1;
    let mid = x / 2 + y / 2;
    let (x, y) = match dir {
        SplitDir::F | SplitDir::L => (x, mid),
        SplitDir::B | SplitDir::R => (mid, y),
    };
    (x, y - 1)
}

fn split_axis_rec((x, y): (usize, usize), dirs: &mut Vec<SplitDir>) -> usize {
    // println!("split_axis_rec, {}, {}, {:#?}", x, y, dirs);
    let dir = dirs.pop().unwrap();
    let (x, y) = split_axis((x, y), dir);
    if x == y {
        return x;
    }
    split_axis_rec((x, y), dirs)
}

pub fn calc(pass: &str) -> (usize, usize) {
    let mut row_dirs = pass
        .chars()
        .filter_map(|c| match c {
            'F' => Some(SplitDir::F),
            'B' => Some(SplitDir::B),
            _ => None,
        })
        .rev()
        .collect();
    let row = split_axis_rec((0, ROWS - 1), &mut row_dirs);
    let mut seat_dirs = pass
        .chars()
        .filter_map(|c| match c {
            'L' => Some(SplitDir::L),
            'R' => Some(SplitDir::R),
            _ => None,
        })
        .rev()
        .collect();
    let seat = split_axis_rec((0, SEATS_IN_ROW - 1), &mut seat_dirs);
    (row, seat)
}

pub fn main() -> Day {
    let passes = get_data(get_string("day5.txt"));
    // println!("{:#?}", passes[0]);

    let max_seatid = passes
        .iter()
        .map(|pass| calc(&pass))
        .map(|(row, seat)| row * SEATS_IN_ROW + seat)
        .max()
        .unwrap();
    let part1_display = format!("{} is the maximum Seat ID", max_seatid);

    let mut seatids: Vec<usize> = passes
        .iter()
        .map(|pass| calc(&pass))
        .map(|(row, seat)| row * SEATS_IN_ROW + seat)
        .collect();
    seatids.sort_unstable();
    // println!("{:#?}", seatids);
    let my_seatid = seatids
        .windows(2)
        .filter_map(|win| match win[1] - win[0] {
            2 => Some(win[0] + 1),
            _ => None,
        })
        .next()
        .unwrap();
    let part2_display = format!("{} is my Seat ID", my_seatid);

    Day {
        answers: Parts(max_seatid.to_string(), my_seatid.to_string()),
        display: Parts(part1_display, part2_display),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "935");
        assert_eq!(day.answers.1, "743");
    }
}
