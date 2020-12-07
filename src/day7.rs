use recap::Recap;
use regex::Regex;
use serde::Deserialize;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use crate::get_string;
use crate::{Day, Parts};

#[derive(PartialEq, Eq, Hash, Clone, Debug, Deserialize, Recap)]
#[recap(regex = r#"(?P<attribute>.+) (?P<color>.+) bags?"#)]
pub struct Bag {
    pub attribute: String,
    pub color: String,
}

#[derive(Debug)]
pub struct BagRule {
    pub outer: Bag,
    pub contents: Vec<(usize, Bag)>,
}

#[derive(Debug, Clone)]
pub struct BagRuleError;

impl FromStr for BagRule {
    type Err = BagRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_RULE: Regex = Regex::new(r"^(.+) contain (.+).$").unwrap();
            static ref RE_COUNT: Regex = Regex::new(r"^(\d) (.+)|no other bags$").unwrap();
        }
        let captures = RE_RULE.captures(s).unwrap();
        let contents: Vec<(usize, Bag)> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(", ")
            .filter_map(|count| {
                let captures = RE_COUNT.captures(count).unwrap();
                match captures.get(0).unwrap().as_str() {
                    "no other bags" => None,
                    _ => Some((
                        captures.get(1).unwrap().as_str().parse().unwrap(),
                        captures.get(2).unwrap().as_str().parse().unwrap(),
                    )),
                }
            })
            .collect();
        Ok(BagRule {
            outer: captures.get(1).unwrap().as_str().parse().unwrap(),
            contents,
        })
    }
}

pub fn get_data(input: String) -> Vec<BagRule> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn build_invert_tree<'a>(bag_rules: &'a [BagRule]) -> HashMap<&'a Bag, Vec<&'a Bag>> {
    let mut inverse_tree: HashMap<_, Vec<&Bag>> = HashMap::new();
    for bag_rule in bag_rules {
        for content in &bag_rule.contents {
            match inverse_tree.entry(&content.1) {
                Entry::Occupied(o) => o.into_mut().push(&bag_rule.outer),
                Entry::Vacant(v) => {
                    v.insert(vec![&bag_rule.outer]);
                }
            };
        }
    }
    inverse_tree
}

pub fn build_tree_count<'a>(bag_rules: &'a [BagRule]) -> HashMap<&'a Bag, Vec<(usize, &'a Bag)>> {
    let mut tree: HashMap<&Bag, Vec<(usize, &Bag)>> = HashMap::new();
    for bag_rule in bag_rules {
        tree.insert(
            &bag_rule.outer,
            bag_rule.contents.iter().map(|c| (c.0, &c.1)).collect(),
        );
    }
    tree
}

pub fn walk_tree<'a>(
    hash_map: &HashMap<&'a Bag, Vec<&'a Bag>>,
    start: &'a Bag,
) -> HashSet<&'a Bag> {
    let mut retval = HashSet::new();
    retval.insert(start);
    if let Some(leafs) = hash_map.get(start) {
        for leaf in leafs
            .iter()
            .cloned()
            .map(|b| walk_tree(hash_map, b))
            .flatten()
        {
            retval.insert(leaf);
        }
    }
    retval
}

pub fn walk_tree_count<'a>(
    hash_map: &HashMap<&'a Bag, Vec<(usize, &'a Bag)>>,
    start: &'a Bag,
) -> HashMap<&'a Bag, usize> {
    let mut retval: HashMap<&Bag, usize> = HashMap::new();
    if let Some(leafs) = hash_map.get(start) {
        for (count, bag) in leafs {
            retval
                .entry(bag)
                .and_modify(|c| *c += count)
                .or_insert(*count);
        }
        for (parent_count, child_map) in leafs
            .iter()
            .map(|leaf| (leaf.0, walk_tree_count(hash_map, leaf.1)))
        {
            for (bag, count) in child_map {
                let count = parent_count * count;
                retval
                    .entry(bag)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            }
        }
    }
    retval
}

pub fn main() -> Day {
    let bag_rules = get_data(get_string("day7.txt"));
    // println!("{:#?}", bag_rules[0]);

    let my_bag = Bag {
        attribute: "shiny".to_string(),
        color: "gold".to_string(),
    };
    let child_to_parent = build_invert_tree(&bag_rules);
    let parents = walk_tree(&child_to_parent, &my_bag);
    // println!("{:#?}", parents.iter().take(5).collect::<Vec<&&Bag>>());
    let part1_bags = parents.len() - 1; // Don't count initial bag
    let part1_display = format!(
        "{} bag colors can eventually contain at least one shiny gold bag.",
        part1_bags
    );

    let bag_to_contents = build_tree_count(&bag_rules);
    let contents = walk_tree_count(&bag_to_contents, &my_bag);
    // println!("{:#?}", contents.iter().take(5).collect::<Vec<(&&Bag, &usize)>>());
    let part2_bags = contents.iter().map(|t| t.1).sum::<usize>(); // Don't count initial bag
    let part2_display = format!(
        "{} individual bags are required inside my single shiny gold bag.",
        part2_bags
    );

    Day {
        answers: Parts(part1_bags.to_string(), part2_bags.to_string()),
        display: Parts(part1_display, part2_display),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let day = main();
        assert_eq!(day.answers.0, "246");
        assert_eq!(day.answers.1, "2976");
    }

    #[test]
    fn test_bag() {
        let bag = "light red bags".parse::<Bag>().unwrap();
        assert_eq!(bag.attribute, "light");
        assert_eq!(bag.color, "red");
    }

    #[test]
    fn test_bagrule() {
        let bagrule = "light red bags contain 1 bright white bag, 2 muted yellow bags."
            .parse::<BagRule>()
            .unwrap();
        assert_eq!(bagrule.outer.attribute, "light");
        assert_eq!(bagrule.outer.color, "red");
        assert_eq!(bagrule.contents.len(), 2);
        assert_eq!(bagrule.contents[0].0, 1);
        assert_eq!(bagrule.contents[0].1.color, "white");
        assert_eq!(bagrule.contents[1].0, 2);
        assert_eq!(bagrule.contents[1].1.attribute, "muted");
    }

    #[test]
    fn test_part1_example() {
        let bagrules = get_data(
            "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
                .to_string(),
        );
        assert_eq!(bagrules.len(), 9);
    }
}
