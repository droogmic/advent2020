use advent2020::day16::{Rule, RuleStr, parse_notes};

use proptest::prelude::*;

prop_compose! {
    fn arbitrary_rule(max: usize)
            (start in 0usize..max, d1 in 0usize..max, d2 in 0usize..max, d3 in 0usize..max)
                -> Rule {
        Rule::new((start, start + d1), (start + d1 + d2, start + d1 + d2 + d3))
    }
}

proptest! {
    #[test]
    fn rule_check_val(rule in arbitrary_rule(1000usize), v in 0usize..10000usize) {
        rule.check_val(v);
    }
}

prop_compose! {
    fn arbitrary_rule_range(max: usize)
            (start in 0usize..max, d1 in 0usize..max, d2 in 0usize..max, d3 in 0usize..max)
                -> ((usize, usize), (usize, usize)) {
        ((start, start + d1), (start + d1 + d2, start + d1 + d2 + d3))
    }
}

fn format_rule_range(idx: usize, rule: ((usize, usize), (usize, usize))) -> String {
    format!(
        "Rule {}: {}-{} or {}-{}",
        idx, rule.0 .0, rule.0 .1, rule.1 .0, rule.1 .1,
    )
}

fn arbitrary_fields(rules: &[((usize, usize), (usize, usize))]) -> impl Strategy<Value = Vec<usize>> {
    rules.into_iter().map(|rule| {
        println!("{:#?}", rule);
        prop_oneof![
            (rule.0.0..=rule.0.1),
            (rule.1.0..=rule.1.1),
        ]
    }).collect::<Vec<_>>()
}

fn arbitrary_notes() -> impl Strategy<Value = String> {
    prop::collection::vec(arbitrary_rule_range(1000), 10..20)
    .prop_flat_map(|rules| {
        let ticket_strategy = prop::collection::vec(arbitrary_fields(&rules), 200..300);
        (Just(rules), ticket_strategy)
    })
    .prop_map(|(rules, tickets)| {
        let rule_str = rules.into_iter()
            .enumerate()
            .map(|(idx, rule)| {
                format_rule_range(idx, rule)
            })
            .collect::<Vec<String>>()
            .join("\n");
        let ticket_strs = tickets.into_iter().map(|ticket| ticket.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(",")).collect::<Vec<String>>();
        let your_ticket = ticket_strs.first().unwrap();
        let nearby_tickets = ticket_strs.iter().skip(1).map(|t| t.to_string()).collect::<Vec<String>>().join("\n");
        format!(
            "\
{}

your ticket:
{}

nearby tickets:
{}", rule_str, your_ticket, nearby_tickets, 
        )
    })
}

proptest! {
    #[test]
    fn rule_parse(rule_range in arbitrary_rule_range(1000)) {
        let rule_str = format_rule_range(0, rule_range);
        println!("{}", rule_str);
        rule_str.parse::<RuleStr>().unwrap();
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1))]
    #[test]
    fn notes_parse(note_str in arbitrary_notes()) {
        println!("{}", note_str);
        parse_notes(&note_str);
    }
}
