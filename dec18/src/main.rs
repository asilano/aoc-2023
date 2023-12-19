use std::collections::{HashMap, HashSet};

use Direction::*;
use CellType::*;
use input_curler::input_for;

fn main() {
    let data = input_for(18).unwrap();

    {
        let (instructions, perimeter) = parse_data_one(&data);
        let mut lagoon = dig_trench(&instructions);

        let answer_one = count_by_outside(&mut lagoon);
        println!("Part one: {}", answer_one);

        let corners = lagoon_corners(&instructions);
        let answer_one = area_by_shoelace(&corners, perimeter);
        println!("Part one again: {}", answer_one);
    }
    {
        let (instructions, perimeter) = parse_data_two(&data);
        let corners = lagoon_corners(&instructions);
        let answer_two = area_by_shoelace(&corners, perimeter);
        println!("Part two: {}", answer_two);
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}
impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'U' => Up,
            'R' => Right,
            'D' => Down,
            'L' => Left,
            '3' => Up,
            '0' => Right,
            '1' => Down,
            '2' => Left,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: usize,
    // colour:
}

fn parse_data_one(data: &str) -> (Vec<Instruction>, usize) {
    let instrs: Vec<Instruction> = data.lines().map(|line| {
        let mut parts = line.split_whitespace();
        Instruction {
            direction: Direction::from(parts.next().unwrap().chars().next().unwrap()),
            distance: parts.next().unwrap().parse::<usize>().unwrap(),
            // colour
        }
    }).collect();
    let perim = instrs.iter().map(|i| i.distance).sum();
    (instrs, perim)
}

fn parse_data_two(data: &str) -> (Vec<Instruction>, usize) {
    let instrs: Vec<Instruction> = data.lines().map(|line| {
        let coded = line.rsplit_once(' ').unwrap().1;
        Instruction {
            direction: Direction::from(coded.chars().rev().nth(1).unwrap()),
            distance: usize::from_str_radix(&coded.chars().skip(2).take(5).collect::<String>(), 16).unwrap(),
            // colour
        }
    }).collect();
    let perim = instrs.iter().map(|i| i.distance).sum();
    (instrs, perim)
}

fn lagoon_corners(instructions: &[Instruction]) -> Vec<(i64, i64)> {
    let mut current = (0i64, 0i64);
    let mut corners = vec![];

    for instruction in instructions {
        corners.push(current);
        match instruction.direction {
            Up => current.0 -= instruction.distance as i64,
            Right => current.1 += instruction.distance as i64,
            Down => current.0 += instruction.distance as i64,
            Left => current.1 -= instruction.distance as i64,
        }
    }
    corners.push((0, 0));
    corners
}

fn area_by_shoelace(corners: &[(i64, i64)], perimeter: usize) -> u64 {
    corners
        .windows(2)
        .fold(0, |acc, corner_pair| {
            acc + corner_pair[0].0 * corner_pair[1].1 - corner_pair[1].0 * corner_pair[0].1
        }).unsigned_abs() / 2
    + perimeter as u64 / 2 + 1
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum CellType {
    Trench,
    Outside,
    Inside
}

fn dig_trench(instructions: &[Instruction]) -> HashMap<(i64, i64), CellType> {
    let mut lagoon = HashMap::new();

    let mut digger = (0i64, 0i64);
    lagoon.insert((0, 0), Trench);
    for instruction in instructions {
        for _step in 0..instruction.distance {
            match instruction.direction {
                Up => digger.0 -= 1,
                Down => digger.0 += 1,
                Left => digger.1 -= 1,
                Right => digger.1 += 1,
            }
            lagoon.insert(digger, Trench);
        }
    }

    lagoon
}

fn count_by_outside(lagoon: &mut HashMap<(i64, i64), CellType>) -> usize {
    let min_y = lagoon.keys().map(|coord| coord.0).min().unwrap() - 1;
    let min_x = lagoon.keys().map(|coord| coord.1).min().unwrap() - 1;
    let max_y = lagoon.keys().map(|coord| coord.0).max().unwrap() + 1;
    let max_x = lagoon.keys().map(|coord| coord.1).max().unwrap() + 1;
println!("Lagoon is {}", (max_x - min_x + 1) * (max_y - min_y + 1));

    // Floodfill "Outside"
    let mut searchfront: HashSet<(i64, i64)> = HashSet::from([(min_y, min_x)]);
    while let Some(&paint_me) = searchfront.iter().next() {
        searchfront.remove(&paint_me);
        lagoon.insert(paint_me, Outside);

        if paint_me.0 > min_y &&
            !lagoon.contains_key(&(paint_me.0 - 1, paint_me.1)) &&
            !searchfront.contains(&(paint_me.0 - 1, paint_me.1))
        {
            searchfront.insert((paint_me.0 - 1, paint_me.1));
        }
        if paint_me.1 > min_x &&
            !lagoon.contains_key(&(paint_me.0, paint_me.1 - 1))  &&
            !searchfront.contains(&(paint_me.0, paint_me.1 - 1))
        {
            searchfront.insert((paint_me.0, paint_me.1 - 1));
        }
        if paint_me.0 < max_y &&
            !lagoon.contains_key(&(paint_me.0 + 1, paint_me.1)) &&
            !searchfront.contains(&(paint_me.0 + 1, paint_me.1))
        {
            searchfront.insert((paint_me.0 + 1, paint_me.1));
        }
        if paint_me.1 < max_x &&
            !lagoon.contains_key(&(paint_me.0, paint_me.1 + 1)) &&
            !searchfront.contains(&(paint_me.0, paint_me.1 + 1))
        {
            searchfront.insert((paint_me.0, paint_me.1 + 1));
        }
    }

    let outside = lagoon.values().filter(|&&v| v == Outside).count();
    ((max_x - min_x + 1) * (max_y - min_y + 1)) as usize - outside
}
