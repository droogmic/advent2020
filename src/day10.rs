use itertools::Itertools;
use std::cmp;
use std::collections::HashMap;

use crate::get_string;
use crate::{Day, Parts};

pub fn get_data(input: String) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

const N: usize = 3;

/// Exhaustive count combinations
pub fn exhaustive_count(sorted_numbers: Vec<usize>) -> usize {
    let min = cmp::max(sorted_numbers.len() / N, 1);
    let max = sorted_numbers.len();
    (min..=max)
        .into_iter()
        .map(|combos| sorted_numbers.iter().copied().combinations(combos))
        .flatten()
        .filter(|combo| {
            combo.first().unwrap() == sorted_numbers.first().unwrap()
                && combo.last().unwrap() == sorted_numbers.last().unwrap()
        })
        .filter(|combo| combo.windows(2).map(|w| w[1] - w[0]).all(|diff| diff <= N))
        .count()
}

/// Obvious section
pub fn section_count(sorted_numbers: Vec<usize>) -> usize {
    // println!("section_count([{}])", sorted_numbers.iter().join(","));
    let diffs: Vec<usize> = sorted_numbers.windows(2).map(|w| w[1] - w[0]).collect();
    std::iter::once(0)
        .chain(diffs.windows(2).enumerate().filter_map(|(idx, w)| {
            if w[0] == N && w[1] == N {
                Some(idx + 1)
            } else {
                None
            }
        }))
        .chain(std::iter::once(sorted_numbers.len() - 1))
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|idxs| {
            let slice = sorted_numbers.get(idxs[0]..=idxs[1]).unwrap();
            exhaustive_count(slice.to_vec())
        })
        .product()
}

pub fn main() -> Day {
    // let mut numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    let mut numbers = get_data(get_string("day10.txt"));
    // println!("{:#?}", numbers[0]);
    numbers.push(0);
    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap() + N);

    let numbers = numbers.to_vec();
    let mut counts = HashMap::new();
    for difference in numbers.windows(2).map(|w| w[1] - w[0]) {
        counts
            .entry(difference)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    let part1_answer = counts.get(&1).unwrap() * counts.get(&N).unwrap();

    let numbers = numbers.to_vec();
    let distinct_arrangements = section_count(numbers);
    Day {
        answers: Parts(part1_answer.to_string(), distinct_arrangements.to_string()),
        display: Parts(
            format!("{} jolts", part1_answer),
            format!("{} distinct arrangements", distinct_arrangements),
        ),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "2592");
        assert_eq!(day.answers.1, "198428693313536");
    }

    #[test]
    fn test_count() {
        for (name, count) in [
            (
                "exhaustive_count",
                exhaustive_count as fn(Vec<usize>) -> usize,
            ),
            ("section_count", section_count),
        ]
        .iter()
        {
            println!("{}", name);
            assert_eq!(count(vec![0, 1]), 1, "[0, 1]");
            assert_eq!(count(vec![0, 2]), 1);
            assert_eq!(count(vec![0, 3]), 1);
            assert_eq!(count(vec![1, 2]), 1);
            assert_eq!(count(vec![3, 6]), 1);
            assert_eq!(count(vec![0, 1, 2]), 2);
            assert_eq!(count(vec![0, 1, 3]), 2);
            assert_eq!(count(vec![0, 2, 3]), 2);
            assert_eq!(count(vec![0, 1, 2, 3]), 4);
            assert_eq!(count(vec![0, 4]), 0, "[0, 4]");
            assert_eq!(count(vec![0, 3, 6]), 1, "[0, 3, 6]");
            assert_eq!(count(vec![0, 1, 3, 6]), 2);
            assert_eq!(count(vec![0, 3, 4, 7]), 1);
            assert_eq!(count(vec![0, 1, 3, 4, 6]), 5);
            assert_eq!(count(vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22]), 8);
        }
    }
}
