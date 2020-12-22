use recap::Recap;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::get_string;
use crate::{Day, Parts};

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"^(?P<name>.+): (?P<r1>\d+)-(?P<r2>\d+) or (?P<r3>\d+)-(?P<r4>\d+)$"#)]
pub struct RuleStr {
    name: String,
    r1: String,
    r2: String,
    r3: String,
    r4: String,
}

#[derive(Clone)]
pub struct Rule {
    ranges: Vec<(usize, usize)>,
}

#[derive(Clone)]
pub struct Ticket {
    fields: Vec<usize>,
}

#[derive(Debug)]
pub struct ParseError;

impl std::str::FromStr for Ticket {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            fields: s
                .split(',')
                .map(|v| v.parse().map_err(|_| ParseError))
                .collect::<Result<Vec<usize>, ParseError>>()?,
        })
    }
}

#[derive(Clone)]
pub struct Notes {
    rules: HashMap<String, Rule>,
    ticket: Ticket,
    nearby: Vec<Ticket>,
}

fn parse_notes(s: &str) -> Notes {
    let mut groups = s
        .lines()
        .map(String::from)
        .collect::<Vec<String>>()
        .split(|line| line.is_empty())
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<String>>>()
        .into_iter();
    let mut rules = HashMap::new();
    // let rules: Vec<RuleStr> = groups.next().unwrap().iter().map(|r| r.parse().unwrap()).collect();
    for rule_str in groups
        .next()
        .unwrap()
        .iter()
        .map(|r| r.parse::<RuleStr>().unwrap())
    {
        rules.insert(
            rule_str.name,
            Rule {
                ranges: vec![
                    (rule_str.r1.parse().unwrap(), rule_str.r2.parse().unwrap()),
                    (rule_str.r3.parse().unwrap(), rule_str.r4.parse().unwrap()),
                ],
            },
        );
    }
    let ticket: Ticket = {
        let my_ticket_lines = groups.next().unwrap();
        assert_eq!(my_ticket_lines.get(0).unwrap(), "your ticket:");
        my_ticket_lines.get(1).unwrap().parse().unwrap()
    };
    let nearby: Vec<Ticket> = {
        let nearby_ticket_lines = groups.next().unwrap();
        assert_eq!(nearby_ticket_lines.get(0).unwrap(), "nearby tickets:");
        nearby_ticket_lines
            .iter()
            .skip(1)
            .map(|t| t.parse().unwrap())
            .collect()
    };
    Notes {
        rules,
        ticket,
        nearby,
    }
}

impl Rule {
    fn check_val(&self, val: usize) -> bool {
        self.ranges.iter().any(|r| r.0 <= val && val <= r.1)
    }
}

// const EXAMPLE: &str = "\
// class: 1-3 or 5-7
// row: 6-11 or 33-44
// seat: 13-40 or 45-50

// your ticket:
// 7,1,14

// nearby tickets:
// 7,3,47
// 40,4,50
// 55,2,20
// 38,6,12";

pub fn main() -> Day {
    // let notes = parse_notes(EXAMPLE);
    let notes = parse_notes(&get_string("day16.txt"));

    let ticket_scanning_error_rate: usize = notes
        .nearby
        .iter()
        .map(|ticket| {
            ticket
                .fields
                .iter()
                .filter_map(|&val| {
                    if notes.rules.values().any(|r| r.check_val(val)) {
                        None
                    } else {
                        Some(val)
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    let part1_display = format!("{} ticket scanning error rate", ticket_scanning_error_rate);

    let rules = notes.rules;
    let ticket = notes.ticket;
    let mut nearby = notes.nearby;
    nearby.retain(|ticket| {
        ticket
            .fields
            .iter()
            .all(|&val| rules.values().any(|r| r.check_val(val)))
    });
    let fields_count = nearby.first().unwrap().fields.len();
    let valid_idx_rule: HashMap<usize, HashSet<String>> = (0..fields_count)
        .into_iter()
        .map(|field_idx| {
            let fields: Vec<usize> = nearby
                .iter()
                .map(|ticket| ticket.fields[field_idx])
                .collect();
            let rule_names: HashSet<String> = rules
                .iter()
                .filter_map(move |(rule_name, rule)| {
                    if fields.iter().all(|&f| rule.check_val(f)) {
                        Some(rule_name.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            (field_idx, rule_names)
        })
        .collect();
    // println!("{:#?}", valid_idx_rule);
    let mut rule_idx: HashMap<String, usize> = HashMap::new();
    while rule_idx.len() < fields_count {
        let mapped_rules: HashSet<String> = rule_idx.keys().cloned().collect();
        for (&field_idx, rule_name) in
            valid_idx_rule.iter().filter_map(|(field_idx, rule_names)| {
                let unmapped_rule_names = rule_names
                    .difference(&mapped_rules)
                    .map(String::from)
                    .collect::<Vec<String>>();
                if unmapped_rule_names.len() == 1 {
                    Some((field_idx, unmapped_rule_names.first().unwrap().to_string()))
                } else {
                    None
                }
            })
        {
            rule_idx.insert(rule_name, field_idx);
        }
        // println!("{:#?}", rule_idx);
    }
    let ticket_departures: Vec<usize> = rule_idx
        .iter()
        .filter_map(|(rule_name, &field_idx)| {
            if rule_name.starts_with("departure") {
                Some(ticket.fields[field_idx])
            } else {
                None
            }
        })
        .collect();
    let ticket_departures_product = ticket_departures.iter().product::<usize>();
    let part_2_display = format!(
        "{} = {}, the departure fields on my ticket",
        ticket_departures_product,
        ticket_departures
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("×"),
    );

    Day {
        answers: Parts(
            ticket_scanning_error_rate.to_string(),
            ticket_departures_product.to_string(),
        ),
        display: Parts(part1_display, part_2_display),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "32842");
        assert_eq!(day.answers.1, "2628667251989");
    }
}
