use std::collections::HashMap;

use input_curler::input_for;
use itertools::Itertools;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}
use HandType::*;

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    bid: u32,
    classification: HandType
}
impl Hand {
    fn new(card_str: &str, bid: u32, part: u8) -> Self {
        let cards: [u8; 5] = card_str.chars().map(|c|
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => if part == 1 { 11 } else { 1 },
                'T' => 10,
                _ => c.to_digit(10).unwrap()
            } as u8
        ).collect::<Vec<u8>>().try_into().unwrap();
        let classification = Self::classify(&cards);

        Self {
            cards,
            bid,
            classification
        }
    }

    fn classify(cards: &[u8; 5]) -> HandType {
        let most_matching = Self::most_matching(cards);
        match most_matching {
            (5, 0) => FiveKind,
            (4, 1) => FourKind,
            (3, 2) => FullHouse,
            (3, 1) => ThreeKind,
            (2, 2) => TwoPair,
            (2, 1) => OnePair,
            (1, 1) => HighCard,
            _ => unreachable!()
        }
    }

    fn most_matching(cards: &[u8; 5]) -> (usize, usize) {
        let mut counts = cards.iter()
            .fold(HashMap::<u8, usize>::new(), |mut acc, c| {
                acc.entry(*c).and_modify(|cnt| *cnt += 1).or_insert(1);
                acc
            });

        let jokers = *counts.get(&1).unwrap_or(&0);
        let maybe_most = counts.iter_mut().filter(|(&val, _)| val != 1).max_by_key(|(_, num)| **num);
        if let Some(most) = maybe_most {
            *most.1 += jokers;
            counts.remove(&1);
        }
        counts.into_values()
            .sorted()
            .rev()
            .next_tuple()
            .unwrap_or((5, 0))
    }
}

fn main() {
    let data = input_for(7).unwrap();

    let hands = parse_data(&data, 1);
    let answer_one = score(&hands);
    println!("Part one: {}", answer_one);

    let hands = parse_data(&data, 2);
    let answer_two = score(&hands);
    println!("Part two: {}", answer_two);
}

fn parse_data(data: &str, part: u8) -> Vec<Hand> {
    data.lines().map(|line| {
        let (card_str, bid_str) = line.split_once(' ').unwrap();
        let bid = bid_str.parse::<u32>().unwrap();
        Hand::new(card_str, bid, part)
    }).collect()
}

fn score(hands: &[Hand]) -> u32 {
    hands.iter().sorted_by_key(|&hand| (&hand.classification, hand.cards)).enumerate().map(|(rank, hand)| {
        (rank as u32 + 1) * hand.bid
    }).sum()
}