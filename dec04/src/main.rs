use std::collections::HashSet;

use input_curler::input_for;

struct Scratchcard
{
    id: u32,
    winners: HashSet<u32>,
    owned: HashSet<u32>
}

fn main() {
    let data = input_for(4).unwrap();
    let cards = parse_data(&data);

    let answer_one = part_one(&cards);
    println!("Part one: {}", answer_one);
    let answer_two = part_two(&cards);
    println!("Part two: {}", answer_two);
}

fn parse_data(data: &String) -> Vec<Scratchcard> {
    data.lines().map(|line| {
        let (name_part, numbers_part) = line.split_once(':').unwrap();
        let id_part = name_part.split_whitespace().last().unwrap();
        let id = id_part.parse::<u32>().unwrap();
        let (winner_part, owned_part) = numbers_part.split_once('|').unwrap();

        let winners = winner_part.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<u32>>();
        let owned = owned_part.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<HashSet<u32>>();

        Scratchcard { id, winners, owned }
    }).collect()
}

fn part_one(cards: &[Scratchcard]) -> u32 {
    cards.iter().map(|card| {
        let win_count = card.winners.intersection(&card.owned).count() as u32;
        if win_count == 0 { 0 } else { 2u32.pow(win_count - 1) }
    }).sum()
}

fn part_two(cards: &[Scratchcard]) -> u32 {
    let mut card_counts = vec![1u32; cards.len()];
    for (ix, card) in cards.iter().enumerate() {
        let win_count = card.winners.intersection(&card.owned).count();

        let this_card_count = card_counts[ix];
        for inc in 0..win_count {
            *card_counts.get_mut(ix + inc + 1).unwrap() += this_card_count;
        }
    }

    card_counts.iter().sum()
}