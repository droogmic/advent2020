use itertools::Itertools;

use crate::get_string;
use crate::{Day, Parts};

pub fn get_data(input: String) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn main() -> Day {
    let numbers = get_data(get_string("day9.txt"));
    // println!("{:#?}", numbers[0]);

    const N: usize = 25;
    let sum_answer = numbers
        .iter()
        .enumerate()
        .skip(N)
        .filter_map(|(idx, num)| {
            let skip_count = idx.saturating_sub(N);
            let prev = numbers.iter().cloned().skip(skip_count).take(N);
            if !prev
                .combinations(2)
                .map(|v| v.into_iter().sum::<usize>())
                .any(|s| s == *num)
            {
                return Some(*num);
            }
            None
        })
        .next()
        .unwrap();
    let part1_display = format!("{} is the first number that is not a sum", sum_answer);

    let mut range = numbers
        .iter()
        .enumerate()
        .filter_map(|(start_idx, num)| {
            let mut end_idx = start_idx;
            let mut num = *num;
            while num < sum_answer {
                end_idx += 1;
                num += numbers.get(end_idx).unwrap();
            }
            if num == sum_answer {
                Some(numbers.get(start_idx..=end_idx).unwrap())
            } else {
                None
            }
        })
        .next()
        .unwrap()
        .to_vec();
    range.sort_unstable();
    let sum_small_big = range.first().unwrap() + range.last().unwrap();
    let part2_display = format!("{} = min + max of {} numbers", sum_small_big, range.len());

    Day {
        answers: Parts(sum_answer.to_string(), sum_small_big.to_string()),
        display: Parts(part1_display, part2_display),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "50047984");
        assert_eq!(day.answers.1, "5407707");
    }
}
