use std::collections::HashMap;
use std::num::ParseIntError;

use crate::get_string;

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
    byr: Option<Result<usize, ParseIntError>>,
    iyr: Option<Result<usize, ParseIntError>>,
    eyr: Option<Result<usize, ParseIntError>>,
    hgt: Option<Result<Height, HeightError>>,
    hcl: Option<Result<String, HairError>>,
    ecl: Option<Result<EyeColor, EyeError>>,
    pid: Option<Result<String, PassportError>>,
    cid: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HeightError;

#[derive(Debug, Clone)]
pub struct HairError;

#[derive(Debug, Clone)]
pub struct EyeError;

#[derive(Debug, Clone)]
pub struct PassportError;

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
                byr: entry.get("byr").map(|s| s.parse()),
                iyr: entry.get("iyr").map(|s| s.parse()),
                eyr: entry.get("eyr").map(|s| s.parse()),
                hgt: entry.get("hgt").map(|s| {
                    let height = s
                        .trim_end_matches("cm")
                        .trim_end_matches("in")
                        .parse()
                        .unwrap();
                    match s.get(s.len() - 2..) {
                        Some("cm") => Ok(Height::Cm(height)),
                        Some("in") => Ok(Height::In(height)),
                        _ => Err(HeightError),
                    }
                }),
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

pub fn main() {
    let passports = get_data(get_string("day4.txt"));
    // println!("{:#?}", passports[3]);

    let valid_passports: usize = passports
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
    println!("Part 1: {} valid passports", valid_passports);

    let valid_passports: usize = passports
        .iter()
        .filter(|passport| {
            passport.byr.is_some()
                && passport.byr.as_ref().unwrap().is_ok()
                && passport.byr.as_ref().unwrap().as_ref().unwrap() >= &1920
                && passport.byr.as_ref().unwrap().as_ref().unwrap() <= &2002
                && passport.iyr.is_some()
                && passport.iyr.as_ref().unwrap().is_ok()
                && passport.iyr.as_ref().unwrap().as_ref().unwrap() >= &2010
                && passport.iyr.as_ref().unwrap().as_ref().unwrap() <= &2020
                && passport.eyr.is_some()
                && passport.eyr.as_ref().unwrap().is_ok()
                && passport.eyr.as_ref().unwrap().as_ref().unwrap() >= &2020
                && passport.eyr.as_ref().unwrap().as_ref().unwrap() <= &2030
                && passport.hgt.is_some()
                && passport.hgt.as_ref().unwrap().as_ref().is_ok()
                && match *passport.hgt.as_ref().unwrap().as_ref().unwrap() {
                    Height::Cm(h) => h >= 150 && h <= 193,
                    Height::In(h) => h >= 59 && h <= 76,
                }
                && passport.hcl.is_some()
                && passport.hcl.as_ref().unwrap().as_ref().is_ok()
                && passport.ecl.is_some()
                && passport.ecl.as_ref().unwrap().as_ref().is_ok()
                && passport.pid.is_some()
                && passport.pid.as_ref().unwrap().as_ref().is_ok()
        })
        .count();
    println!("Part 2: {} valid passports", valid_passports);
}
