use std::{collections::HashMap, iter};
use itertools::{intersperse_with, Itertools};

use input_curler::input_for;

type History = HashMap<(String, Vec<usize>, bool), u64>;

fn main() {
    let data = input_for(12).unwrap();

    let rows = data.lines().map(|line| {
        let (pattern, groups) = line.split_once(' ').unwrap();
        let group_counts = groups.split(',').map(|n| n.parse::<usize>().unwrap());
        (pattern, group_counts.collect::<Vec<usize>>())
    });

    let mut history = History::new();
    let answer_one: u64 = rows.clone().map(|row|
        count_possibilities(row.0, &row.1.clone(), false, &mut history)
    ).sum();
    println!("Part one: {}", answer_one);

    let answer_two: u64 = rows.map(|row| {
        let pattern = iter::repeat(row.0).take(5).intersperse("?").collect::<String>();
        let group_counts = row.1.repeat(5);
        count_possibilities(&pattern, &group_counts, false, &mut history)
    }).sum();
    println!("Part two: {}", answer_two);
}

fn count_possibilities(pattern: &str, groups: &[usize], in_run: bool, history: &mut History) -> u64 {
    if let Some(c) = history.get(&(pattern.to_string(), groups.to_vec(), in_run)) {
        return *c;
    }

    let mut groups_vec = groups.to_vec();
    let mut group_counts = groups_vec.as_mut_slice();
    let count = match pattern.chars().next() {
        Some('.') => {
            if in_run && group_counts[0] > 0 {
                0
            } else {
                if !group_counts.is_empty() && group_counts[0] == 0 {
                    group_counts = &mut group_counts[1..];
                }
                count_possibilities(&pattern[1..], group_counts, false, history)
            }
        },
        Some('#') => {
            if group_counts.is_empty() || group_counts[0] == 0 {
                0
            } else {
                group_counts[0] -= 1;
                count_possibilities(&pattern[1..], group_counts, true, history)
            }
        },
        Some('?') => {
            let mut pattern_ok = ".".to_string();
            pattern_ok.push_str(&pattern[1..]);
            let mut pattern_broken = "#".to_string();
            pattern_broken.push_str(&pattern[1..]);
            count_possibilities(pattern_ok.as_str(), group_counts, in_run, history)
                + count_possibilities(pattern_broken.as_str(), group_counts, in_run, history)
        },
        None => {
            if group_counts.is_empty() || (group_counts.len() == 1 && group_counts[0] == 0) {
                1
            } else {
                0
            }
        },
        _ => unreachable!()
    };

    history.insert((pattern.to_string(), groups.to_vec(), in_run), count);
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_when_no_unknown_and_matches() {
        let pattern = ".#..###.##..#";
        let mut group_counts = vec![1, 3, 2, 1];

        let mut history = History::new();
        let possibilities = count_possibilities(pattern, &group_counts, false, &mut history);
        assert_eq!(possibilities, 1);
    }

    #[test]
    fn test_when_no_unknown_and_cant_match() {
        let pattern = ".#..###.##..#";
        let group_counts = vec![1, 3, 3, 1];

        let mut history = History::new();
        let possibilities = count_possibilities(pattern, &group_counts, false, &mut history);
        assert_eq!(possibilities, 0);
    }

    #[test]
    fn test_when_unknown_leads_to_single_match() {
        let pattern = ".#..#?#.##..#";
        let group_counts = vec![1, 3, 2, 1];

        let mut history = History::new();
        let possibilities = count_possibilities(pattern, &group_counts, false, &mut history);
        assert_eq!(possibilities, 1);
    }

    #[test]
    fn test_when_unknown_leads_to_two_matches() {
        let pattern = ".??";
        let group_counts = vec![1];

        let mut history = History::new();
        let possibilities = count_possibilities(pattern, &group_counts, false, &mut history);
        assert_eq!(possibilities, 2);
    }

    #[test]
    fn test_when_unknown_leads_to_multiple_matches() {
        let pattern = ".??.??";
        let group_counts = vec![1, 1];

        let mut history = History::new();
        let possibilities = count_possibilities(pattern, &group_counts, false, &mut history);
        assert_eq!(possibilities, 4);
    }

    #[test]
    fn test_with_10_matches() {
        let pattern = "?###????????";
        let group_counts = vec![3, 2, 1];

        let mut history = History::new();
        let possibilities = count_possibilities(pattern, &group_counts, false, &mut history);
        assert_eq!(possibilities, 10);
    }
}
