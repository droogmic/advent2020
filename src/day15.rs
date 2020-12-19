use crate::{Day, Parts};

use std::collections::HashMap;

#[derive(Clone)]
struct MemoryGame {
    mem: HashMap<usize, usize>,
    last_spoken: usize,
    last_turn: usize,
}

impl MemoryGame {
    fn new(start: &[usize]) -> MemoryGame {
        let mut mem: HashMap<usize, usize> = HashMap::new();
        for (idx, v) in start.iter().enumerate() {
            mem.insert(*v, idx + 1);
        }
        MemoryGame {
            mem,
            last_spoken: *start.last().unwrap(),
            last_turn: start.len(),
        }
    }
}

impl Iterator for MemoryGame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // println!("{},{}", self.last_spoken, self.last_turn);
        // println!("{:#?}", self.mem);
        let spoken = match self.mem.get(&self.last_spoken) {
            None => 0,
            Some(prev_spoken) => self.last_turn.checked_sub(*prev_spoken).unwrap(),
        };
        self.mem.insert(self.last_spoken, self.last_turn);
        self.last_spoken = spoken;
        self.last_turn += 1;
        Some(self.last_spoken)
    }
}

pub fn main() -> Day {
    // let start = vec![0, 3, 6];
    let start = vec![14, 8, 16, 0, 1, 17];
    let mut mem_game = MemoryGame::new(&start);

    // println!("[{}]", mem_game.clone().take(10).map(|v| v.to_string()).collect::<Vec<String>>().join(","));
    let part1 = mem_game.clone().nth(2020 - start.len() - 1).unwrap();
    let part1_display = format!("{} = turn 2020", part1);

    let part2 = mem_game.nth(30000000 - start.len() - 1).unwrap();
    let part2_display = format!("{} = turn 30000000", part2);

    Day {
        answers: Parts(part1.to_string(), part2.to_string()),
        display: Parts(part1_display, part2_display),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_main() {
    //     let day = main();
    //     assert_eq!(day.answers.0, "2020");
    //     assert_eq!(day.answers.1, "30000000");
    // }

    #[test]
    fn test_examples_part1() {
        let examples = vec![
            (vec![0, 3, 6], 436),
            (vec![1, 3, 2], 1),
            (vec![2, 1, 3], 10),
            (vec![1, 2, 3], 27),
            (vec![2, 3, 1], 78),
            (vec![3, 2, 1], 438),
            (vec![3, 1, 2], 1836),
        ];
        for example in examples {
            let mut mem_game = MemoryGame::new(&example.0);
            assert_eq!(
                mem_game.nth(2020 - example.0.len() - 1).unwrap(),
                example.1,
                "{:#?}",
                example.0
            );
        }
    }
}
