use std::{collections::HashMap, ops::RangeInclusive};
use input_curler::input_for;
use regex::Regex;
use Action::*;

#[derive(Debug, Clone, Copy)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Action<'a> {
    Accept,
    Reject,
    Switch(&'a str)
}
impl<'a> From<&'a str> for Action<'a> {
    fn from(value: &'a str) -> Self {
        match value {
            "A" => Accept,
            "R" => Reject,
            name => Switch(name)
        }
    }
}

// #[derive(Debug)]
struct Rule<'a> {
    field: char,
    op: char,
    check_value: u32,
    check: Box<dyn Fn(&Part) -> bool>,
    action: Action<'a>
}

// #[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    fallback_action: Action<'a>
}
impl<'a> Workflow<'a> {
    fn apply(&self, part: &Part) -> Action {
        let matching_rule = self.rules.iter().find(|rule| (rule.check)(part));
        if let Some(rule) = matching_rule {
            rule.action
        } else {
            self.fallback_action
        }
    }
}

fn main() {
    let data = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}".to_string();
    let data = input_for(19).unwrap();

    let (workflows, parts) = parse_data(&data);
    let answer_one = parts
        .iter()
        .filter(|part| accept_by_workflows(part, &workflows))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<u32>();
    println!("Part one: {}", answer_one);

    let answer_two = part_two(&workflows);
    println!("Part two: {}", answer_two);
}

fn parse_data(data: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let mut lines = data.lines();

    let workflows = lines
        .by_ref()
        .map_while(|line|
            if line.is_empty() {
                None
            } else {
                Some(parse_workflow(line))
            })
        .collect();

    let part_regex = Regex::new(r"^\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}$").unwrap();
    let parts = lines.map(|line| {
        let captures = part_regex.captures(line).unwrap();
        Part {
            x: captures.name("x").unwrap().as_str().parse().unwrap(),
            m: captures.name("m").unwrap().as_str().parse().unwrap(),
            a: captures.name("a").unwrap().as_str().parse().unwrap(),
            s: captures.name("s").unwrap().as_str().parse().unwrap(),
        }
    }).collect();
    (workflows, parts)
}

fn parse_workflow(data: &str) -> (&str, Workflow) {
    let workflow_regex = Regex::new(r"^(?<name>\w+)\{(?<rules>.*)\}$").unwrap();
    let captures = workflow_regex.captures(data).unwrap();
    let name = captures.name("name").unwrap().as_str();

    let rule_parts = captures.name("rules").unwrap().as_str();
    let (rules_parts, fallback_part) = rule_parts.rsplit_once(',').unwrap();
    let rules = rules_parts.split(',').map(parse_rule).collect::<Vec<Rule>>();

    (name, Workflow {
        rules,
        fallback_action: Action::from(fallback_part)
    })
}

fn parse_rule(data: &str) -> Rule {
    let (check_part, action_part) = data.split_once(':').unwrap();
    let mut chars = check_part.chars();
    let field = chars.next().unwrap();
    let op = chars.next().unwrap();
    let check_value = chars.take_while(char::is_ascii_digit).collect::<String>().parse::<u32>().unwrap();
    let action = Action::from(action_part);

    Rule {
        field,
        op,
        check_value,
        check: Box::new(move |part: &Part| -> bool {
            let value = match field {
                'x' => part.x,
                'm' => part.m,
                'a' => part.a,
                's' => part.s,
                _ => unreachable!()
            };
            match op {
                '<' => value < check_value,
                '>' => value > check_value,
                _ => unreachable!()
            }
        }),
        action
    }
}

fn accept_by_workflows(part: &Part, workflows: &HashMap<&str, Workflow>) -> bool {
    let mut current_workflow = workflows.get(&"in").unwrap();

    loop {
        let action = current_workflow.apply(part);
        match action {
            Reject => return false,
            Accept => return true,
            Switch(name) => current_workflow = workflows.get(&name).unwrap()
        };
    }
}

fn part_two(workflows: &HashMap<&str, Workflow>) -> u64 {
    let mut might_accept: Vec<([RangeInclusive<u32>; 4], &str)> = vec![
        ([1..=4000, 1..=4000, 1..=4000, 1..=4000], "in")
    ];
    let mut accepted: Vec<[RangeInclusive<u32>; 4]> = vec![];

    while let Some(mut chunk) = might_accept.pop() {
        let workflow = workflows.get(chunk.1).unwrap();
        let mut fallthrough = true;
        for rule in workflow.rules.iter() {
            let range_of_interest_ix = match rule.field {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => unreachable!()
            };
            let range_of_interest = &mut chunk.0[range_of_interest_ix];
            let split_range = range_of_interest.contains(&rule.check_value);

            if split_range {
                // Part of the range satisfies the condition. Add the sub-range that does so onto might_accept
                // for the next step (or accept or reject it); then continue down workflow.rules with the remainder
                if rule.op == '<' {
                    let matched_part = *range_of_interest.start()..=(rule.check_value - 1);
                    let unmatched_part = rule.check_value..=*range_of_interest.end();
                    let mut matched_chunk = chunk.0.clone();
                    matched_chunk[range_of_interest_ix] = matched_part;
                    let mut unmatched_chunk = chunk.0.clone();
                    unmatched_chunk[range_of_interest_ix] = unmatched_part;
                    match rule.action {
                        Accept => accepted.push(matched_chunk),
                        Reject => {},
                        Switch(name) => might_accept.push((matched_chunk, name))
                    };
                    chunk.0 = unmatched_chunk;
                } else if rule.op == '>' {
                    let unmatched_part = *range_of_interest.start()..=rule.check_value;
                    let matched_part = (rule.check_value + 1)..=*range_of_interest.end();
                    let mut matched_chunk = chunk.0.clone();
                    matched_chunk[range_of_interest_ix] = matched_part;
                    let mut unmatched_chunk = chunk.0.clone();
                    unmatched_chunk[range_of_interest_ix] = unmatched_part;
                    match rule.action {
                        Accept => accepted.push(matched_chunk),
                        Reject => {},
                        Switch(name) => might_accept.push((matched_chunk, name))
                    };
                    chunk.0 = unmatched_chunk;
                }
            } else {
                // Either the whole range satisfies, or the whole range doesn't. Based on whether start does,
                // either push the range back onto might_accept for the next step (or accept or reject it); or
                // continue down with the whole range.
                if (rule.op == '<' && range_of_interest.start() < &rule.check_value) ||
                    (rule.op == '>' && range_of_interest.start() > &rule.check_value)
                {
                    match rule.action {
                        Accept => accepted.push(chunk.clone().0),
                        Reject => {},
                        Switch(name) => might_accept.push((chunk.clone().0, name))
                    };
                    fallthrough = false;
                    break;
                }
            }
        }

        if fallthrough {
            match workflow.fallback_action {
                Accept => accepted.push(chunk.0),
                Reject => {},
                Switch(name) => might_accept.push((chunk.0, name))
            };
        }
    }

    accepted.iter().map(|chunk| chunk.iter().map(|range| (range.end() - range.start() + 1) as u64).product::<u64>()).sum()
}