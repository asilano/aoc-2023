use std::{collections::{HashMap, HashSet}, ops::Sub};

use input_curler::input_for;


type Brick = Vec<(usize, usize, usize)>;

fn main() {
    let data = input_for(22).unwrap();

    let mut bricks = data.lines().map(parse_brick).collect::<Vec<Brick>>();
    bricks.sort_by_key(|b| b[0].2);

    let mut cell_to_brick_map = HashMap::<(usize, usize, usize), usize>::new();
    apply_gravity(&mut bricks, &mut cell_to_brick_map);

    // Support map should be from Set (of supporting indices) to Set (of indices that will fall if all of those vanish)
    let mut support_map = HashMap::<Vec<usize>, HashSet<usize>>::new();
    let answer_one = part_one(&bricks, &cell_to_brick_map, &mut support_map);
    println!("Part one: {}", answer_one);

    let answer_two = part_two(&support_map, bricks.len());
    println!("Part two: {}", answer_two);
}

fn parse_brick(data: &str) -> Brick {
    let (start_str, end_str) = data.split_once('~').unwrap();
    let start = start_str.split(',').map(|d| d.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let end = end_str.split(',').map(|d| d.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    match ((start[0], start[1], start[2]), (end[0], end[1], end[2])) {
        ((x1, y1, z1), (x2, y2, z2)) if x1 != x2 => {
            (x1..=x2).map(|x| (x, y1, z1)).collect()
        },
        ((x1, y1, z1), (x2, y2, z2)) if y1 != y2 => {
            (y1..=y2).map(|y| (x1, y, z1)).collect()
        },
        ((x1, y1, z1), (x2, y2, z2)) => {
            (z1..=z2).map(|z| (x1, y1, z)).collect()
        }
    }
}

fn apply_gravity(bricks: &mut [Brick], cell_map: &mut HashMap<(usize, usize, usize), usize>) {
    for (ix, brick) in bricks.iter_mut().enumerate() {
        while !brick.iter().any(|c| cell_map.contains_key(&(c.0, c.1, c.2 - 1)) || c.2 == 1) {
            for cell in brick.iter_mut() {
                cell.2 -= 1;
            }
        }
        // Brick is resting
        for cell in brick {
            cell_map.insert(*cell, ix);
        }
    }
}

fn part_one(bricks: &[Brick], cell_map: &HashMap<(usize, usize, usize), usize>, support_map: &mut HashMap<Vec<usize>, HashSet<usize>>) -> usize {
    let mut single_supports = HashSet::<usize>::new();

    for (ix, brick) in bricks.iter().enumerate() {
        let supporters = brick.iter().filter_map(|cell| {
            if let Some(&under) = cell_map.get(&(cell.0, cell.1, cell.2 - 1)) {
                if under == ix { None } else { Some(under) }
            } else { None }
        }).collect::<HashSet<usize>>();

        let mut supporters_vec = supporters.clone().into_iter().collect::<Vec<usize>>();
        supporters_vec.sort();
        if supporters_vec.is_empty() { supporters_vec.push(10000); }
        support_map
            .entry(supporters_vec)
            .and_modify(|supported| { supported.insert(ix); })
            .or_insert(HashSet::from([ix]));

        if supporters.len() == 1 {
            single_supports.insert(supporters.into_iter().next().unwrap());

        }
    }

    bricks.len() - single_supports.len()
}

fn part_two(support_map: &HashMap<Vec<usize>, HashSet<usize>>, num_bricks: usize) -> usize {
    (0..num_bricks).map(|ix| {
        let mut falling = HashSet::from([ix]);

        loop {
            let also_falling = support_map.iter().filter_map(|(supports, above)| {
                if supports.iter().all(|s| falling.contains(s)) {
                    Some(above)
                } else {
                    None
                }
            }).fold(HashSet::<usize>::new(), |mut acc, set| {
                acc.extend(set);
                acc
            });

            if also_falling.sub(&falling).is_empty() { break; }
            falling.extend(also_falling);
        }

        falling.len() - 1
    }).sum()
}