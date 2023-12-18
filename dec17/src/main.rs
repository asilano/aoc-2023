use std::collections::{HashMap, VecDeque};
use input_curler::input_for;

fn main() {
    let data = input_for(17).unwrap();

    let answer_one = search(&data, false);
    println!("Part one: {}", answer_one);
    let answer_two = search(&data, true);
    println!("Part two: {}", answer_two);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}
impl Direction {
    fn inverse(&self) -> Self {
        match self {
            Up => Down,
            Left => Right,
            Down => Up,
            Right => Left
        }
    }
}
use Direction::*;

type FullLocation = (usize, usize, Direction, usize);

fn search(data: &str, ultra_crucible: bool) -> u32 {
    let costs = data.lines().map(|line|
        line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
    ).collect::<Vec<Vec<u32>>>();
    let height = costs.len();
    let width = costs[0].len();

    let mut cheapest = HashMap::<FullLocation, (u32, FullLocation)>::new();
    let mut searchfront: VecDeque<(FullLocation, u32, FullLocation)> = VecDeque::from([
        ((0, 1, Right, 1), 0u32, (0, 0, Down, 1)),
        ((1, 0, Right, 1), 0u32, (0, 0, Right, 1))
    ]);
    while !searchfront.is_empty() {
        let (new_loc, prev_cost, prev_loc) = searchfront.pop_front().unwrap();
        let (row, col, direction, steps) = new_loc;
        let new_cost = prev_cost + costs[row][col];
        let been_here_before = cheapest.get(&(row, col, direction, steps));
        if been_here_before.is_none() || been_here_before.unwrap().0 > new_cost {
            cheapest.insert(new_loc, (new_cost, prev_loc));
            searchfront.append(
                &mut next_locations(&new_loc, height, width, ultra_crucible).iter().map(|&loc|
                    (loc, new_cost, new_loc)
                ).collect::<VecDeque<(FullLocation, u32, FullLocation)>>()
            );
        }
    }

    cheapest.iter().filter_map(|(location, (cost, _))| {
        if location.0 == height - 1 && location.1 == width - 1 {
            if ultra_crucible && location.3 < 4 {
                None
            } else {
                Some(*cost)
            }
        } else {
            None
        }
    }).min().unwrap()
}

fn next_locations(current: &FullLocation, height: usize, width: usize, ultra_crucible: bool) -> Vec<FullLocation> {
    let (row, col, in_dir, steps) = *current;

    [Up, Down, Left, Right].iter().filter_map(|new_dir| {
        let next_cell = match new_dir {
            Up => (row as i32 - 1, col as i32),
            Down => (row as i32 + 1, col as i32),
            Left => (row as i32, col as i32 - 1),
            Right => (row as i32, col as i32 + 1)
        };

        if next_cell.0 < 0 || next_cell.1 < 0 || next_cell.0 >= height as i32 || next_cell.1 >= width as i32 {
            None
        } else if !ultra_crucible && (*new_dir == in_dir && steps == 3) {
            None
        } else if ultra_crucible && (*new_dir == in_dir && steps == 10) {
            None
        } else if *new_dir == in_dir.inverse() {
            None
        } else if *new_dir == in_dir {
            Some((next_cell.0 as usize, next_cell.1 as usize, in_dir, steps + 1))
        } else if ultra_crucible && steps < 4 {
            None
        } else {
            Some((next_cell.0 as usize, next_cell.1 as usize, *new_dir, 1))
        }
    }).collect()
}
