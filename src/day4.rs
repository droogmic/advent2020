use std::collections::HashMap;
use std::num::ParseIntError;

use crate::get_string;
use crate::{Day, Parts};

#[derive(Debug)]
pub enum Height {
    Cm(usize),
    In(usize),
}

#[derive(Debug)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazel,
    Other,
}

#[derive(Debug)]
pub struct PassportEntry {
    byr: Option<Result<usize, YearError>>,
    iyr: Option<Result<usize, YearError>>,
    eyr: Option<Result<usize, YearError>>,
    hgt: Option<Result<Height, HeightError>>,
    hcl: Option<Result<String, HairError>>,
    ecl: Option<Result<EyeColor, EyeError>>,
    pid: Option<Result<String, PassportError>>,
    cid: Option<String>,
}

#[derive(Debug)]
pub enum YearError {
    Parse(ParseIntError),
    Range,
}

#[derive(Debug, Clone)]
pub enum HeightError {
    Parse(ParseIntError),
    Unit,
    Range,
}

#[derive(Debug, Clone)]
pub struct HairError;

#[derive(Debug, Clone)]
pub struct EyeError;

#[derive(Debug, Clone)]
pub struct PassportError;

/// Parse year
///
/// ```
/// # use advent2020::day4::*;
///
/// let range = (10, 20);
/// assert!(matches!(parse_year("abra", range), Err(YearError::Parse(_))));
/// assert!(matches!(parse_year("9", range), Err(YearError::Range)));
/// assert!(matches!(parse_year("21", range), Err(YearError::Range)));
/// assert!(matches!(parse_year("10", range), Ok(10)));
/// assert!(matches!(parse_year("20", range), Ok(20)));
/// ```
pub fn parse_year<S: AsRef<str>>(s: S, (lower, upper): (usize, usize)) -> Result<usize, YearError> {
    match s.as_ref().parse() {
        Err(e) => Err(YearError::Parse(e)),
        Ok(v) => {
            if v >= lower && v <= upper {
                Ok(v)
            } else {
                Err(YearError::Range)
            }
        }
    }
}

/// Parse Height
///
/// ```
/// # use advent2020::day4::*;
///
/// assert!(matches!(parse_height("abra"), Err(HeightError::Parse(_))));
/// assert!(matches!(parse_height("100"), Err(HeightError::Unit)));
/// assert!(matches!(parse_height("149cm"), Err(HeightError::Range)));
/// assert!(matches!(parse_height("194cm"), Err(HeightError::Range)));
/// assert!(matches!(parse_height("58in"), Err(HeightError::Range)));
/// assert!(matches!(parse_height("77in"), Err(HeightError::Range)));
/// assert!(matches!(parse_height("150cm"), Ok(Height::Cm(150))));
/// assert!(matches!(parse_height("76in"), Ok(Height::In(76))));
/// ```
pub fn parse_height<S: AsRef<str>>(s: S) -> Result<Height, HeightError> {
    match s
        .as_ref()
        .trim_end_matches("cm")
        .trim_end_matches("in")
        .parse()
    {
        Err(e) => Err(HeightError::Parse(e)),
        Ok(height) => match s.as_ref().get(s.as_ref().len() - 2..) {
            Some("cm") => match height {
                150..=193 => Ok(Height::Cm(height)),
                _ => Err(HeightError::Range),
            },
            Some("in") => match height {
                59..=76 => Ok(Height::In(height)),
                _ => Err(HeightError::Range),
            },
            _ => Err(HeightError::Unit),
        },
    }
}

pub fn get_data(input: String) -> Vec<PassportEntry> {
    input
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|lines| lines.join(" "))
        .map(|entry| {
            let entry: HashMap<_, _> = entry
                .split_whitespace()
                .map(|kv| kv.split(':'))
                .map(|mut kv| {
                    (
                        String::from(kv.next().unwrap()),
                        String::from(kv.next().unwrap()),
                    )
                })
                .collect();
            PassportEntry {
                byr: entry.get("byr").map(|s| parse_year(s, (1920, 2002))),
                iyr: entry.get("iyr").map(|s| parse_year(s, (2010, 2020))),
                eyr: entry.get("eyr").map(|s| parse_year(s, (2020, 2030))),
                hgt: entry.get("hgt").map(parse_height),
                hcl: entry.get("hcl").map(|s| {
                    if s.starts_with('#') {
                        let s = String::from(s.trim_start_matches('#'));
                        if s.chars().all(|c| c.is_ascii_hexdigit()) && s.len() == 6 {
                            return Ok(s);
                        }
                    }
                    Err(HairError)
                }),
                ecl: entry.get("ecl").map(|s| match s.as_str() {
                    "amb" => Ok(EyeColor::Amber),
                    "blu" => Ok(EyeColor::Blue),
                    "brn" => Ok(EyeColor::Brown),
                    "gry" => Ok(EyeColor::Grey),
                    "grn" => Ok(EyeColor::Green),
                    "hzl" => Ok(EyeColor::Hazel),
                    "oth" => Ok(EyeColor::Other),
                    _ => Err(EyeError),
                }),
                pid: entry.get("pid").map(|s| {
                    if s.chars().all(|c| c.is_numeric()) && s.len() == 9 {
                        return Ok(String::from(s));
                    }
                    Err(PassportError)
                }),
                cid: entry.get("cid").map(String::from),
            }
        })
        .collect()
}

pub fn main() -> Day {
    let passports = get_data(get_string("day4.txt"));
    // println!("{:#?}", passports[3]);

    let part1_valid_passports: usize = passports
        .iter()
        .filter(|passport| {
            passport.byr.is_some()
                && passport.iyr.is_some()
                && passport.eyr.is_some()
                && passport.hgt.is_some()
                && passport.hcl.is_some()
                && passport.ecl.is_some()
                && passport.pid.is_some()
        })
        .count();
    let part1_display = format!("{} valid passports", part1_valid_passports);

    let part2_valid_passports: usize = passports
        .iter()
        .filter(|passport| {
            passport.byr.as_ref().map_or(false, |v| v.is_ok())
                && passport.iyr.as_ref().map_or(false, |v| v.is_ok())
                && passport.eyr.as_ref().map_or(false, |v| v.is_ok())
                && passport.hgt.as_ref().map_or(false, |v| v.is_ok())
                && passport.hcl.as_ref().map_or(false, |v| v.is_ok())
                && passport.ecl.as_ref().map_or(false, |v| v.is_ok())
                && passport.pid.as_ref().map_or(false, |v| v.is_ok())
        })
        .count();
    let part2_display = format!("{} valid passports", part2_valid_passports);

    Day {
        answers: Parts(
            part1_valid_passports.to_string(),
            part2_valid_passports.to_string(),
        ),
        display: Parts(part1_display, part2_display),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "182");
        assert_eq!(day.answers.1, "109");
    }
}
