use crate::get_string;
use crate::{Day, Parts};

pub fn get_data(input: String) -> Vec<String> {
    input
        .lines()
        .map(String::from)
        .collect()
}

pub fn main() -> Day {
    let lines = get_data(get_string("day6.txt"));
    println!("{:#?}", lines[0]);

    Day {
        answers: Parts("".to_string(), "".to_string()),
        display: Parts(String::new(), String::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "");
        assert_eq!(day.answers.1, "");
    }
}
