use std::{collections::HashMap, iter};

use input_curler::input_for;

fn main() {
    let data = input_for(14).unwrap();

    let answer_one = part_one(&data);
    println!("Part one: {}", answer_one);

    let answer_two = part_two(&data);
    println!("Part two: {}", answer_two);
}

fn part_one(data: &str) -> i32 {
    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();

    (0..width).map(|col| {
        let mut moment = height as i32;
        let mut weight = 0;

        for (row, line) in data.lines().enumerate() {
            let cell = line.chars().nth(col).unwrap();
            match cell {
                '.' => {},
                'O' => {
                    weight += moment;
                    moment -= 1;
                },
                '#' => {
                    moment = (height - row - 1) as i32;
                },
                _ => unreachable!()
            }
        }

        weight
    }).sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum RockType {
    Fixed,
    Rolling
}
use RockType::*;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Rock {
    x: i32,
    y: i32,
    rock_type: RockType
}

fn part_two(data: &str) -> i32 {
    let height = data.lines().count();
    let width = data.lines().next().unwrap().len();
    let mut rocks = data.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().filter_map(|(col, cell)| {
            match cell {
                'O' => Some(Rock { y: row as i32, x: col as i32, rock_type: Rolling }),
                '#' => Some(Rock { y: row as i32, x: col as i32, rock_type: Fixed }),
                '.' => None,
                _ => unreachable!()
            }
        }).collect::<Vec<Rock>>()
    }).collect::<Vec<Rock>>();

    let mut seen = HashMap::<Vec<Rock>, i32>::new();
    let mut iteration = 0i32;
    let limit = 1_000_000_000;
    while iteration < limit {
        rocks = (0..width).flat_map(|col| {
            roll_rocks_towards_iter_start(
                rocks
                    .iter()
                    .filter(|rock| rock.x == col as i32)
                    .sorted_by_key(|rock| rock.y),
                (0, col as i32),
                    true,
                    true
            )
        }).collect();
        rocks = (0..height).flat_map(|row| {
            roll_rocks_towards_iter_start(
                rocks
                    .iter()
                    .filter(|rock| rock.y == row as i32)
                    .sorted_by_key(|rock| rock.x),
                    (row as i32, 0),
                        false,
                        true
            )
        }).collect();
        rocks = (0..width).flat_map(|col| {
            roll_rocks_towards_iter_start(
                rocks
                    .iter()
                    .filter(|rock| rock.x == col as i32)
                    .sorted_by_key(|rock| rock.y)
                    .rev(),
                    (height as i32 - 1, col as i32),
                        true,
                        false
            )
        }).collect();
        rocks = (0..height).flat_map(|row| {
            roll_rocks_towards_iter_start(
                rocks
                    .iter()
                    .filter(|rock| rock.y == row as i32)
                    .sorted_by_key(|rock| rock.x)
                    .rev(),
                    (row as i32, width as i32 - 1),
                        false,
                        false
            )
        }).collect();
        rocks.sort();

        if let Some(when) = seen.get(&rocks) {
            let period = iteration - when;
            while (iteration + period) < limit {
                iteration += period;
            }
        } else {
            seen.insert(rocks.clone(), iteration);
        }
        iteration += 1;
    }

    score_rocks(&rocks, height as i32)
}

fn roll_rocks_towards_iter_start<'a, Iter>(
    rocks: Iter,
    first_coords: (i32, i32),
    changing_y: bool,
    increasing: bool) -> Vec<Rock>
where Iter: Iterator<Item = &'a Rock>
{
    let mut next_coords = first_coords;

    let rocks_vec: Vec<&Rock> = rocks.collect();

    rocks_vec.iter().map(|r| {
        let cur_coords = next_coords;
        if r.rock_type == Rolling {
            if changing_y {
                next_coords.0 += if increasing { 1 } else { -1 };
            } else {
                next_coords.1 += if increasing { 1 } else { -1 };
            }

            Rock {
                y: cur_coords.0,
                x: cur_coords.1,
                rock_type: Rolling
            }
        } else {
            if changing_y {
                next_coords.0 = r.y + if increasing { 1 } else { -1 };
            } else {
                next_coords.1 = r.x + if increasing { 1 } else { -1 };
            }
            **r
        }
    }).collect()
}

fn score_rocks(rocks: &[Rock], height: i32) -> i32 {
    rocks
        .iter()
        .filter(|r| r.rock_type == Rolling)
        .map(|r| height - r.y)
        .sum()
}