use itertools::Itertools;

use crate::get_string;

pub fn calc(expenses: Vec<usize>, combinations: usize) -> Vec<Vec<usize>> {
    expenses
        .into_iter()
        .combinations(combinations)
        .filter(|v| v.iter().sum::<usize>() == 2020)
        .collect()
}

pub fn get_data(input: String) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse().expect("bad input"))
        .collect()
}

pub fn main() {
    let expenses = get_data(get_string("day1.txt"));
    for n in 2..=3 {
        for mut values in calc(expenses.to_vec(), n)
        {
            values.sort_unstable();
            println!(
                "{} = {}",
                values
                    .to_vec()
                    .into_iter()
                    .map(|v| v.to_string())
                    .join(" × "),
                values.to_vec().into_iter().product::<usize>(),
            );
        }
    }
}
