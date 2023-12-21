use std::collections::HashSet;

use input_curler::input_for;

fn main() {
    let data = input_for(21).unwrap();

    let rock_locations: HashSet<(i64, i64)> = data.lines().enumerate().flat_map(|(y, row)| {
        row.chars().enumerate().filter_map(|(x, cell)| {
            if cell != '#' { None } else { Some((y as i64, x as i64)) }
        }).collect::<HashSet<(i64, i64)>>()
    }).collect();
    let sy = data.lines().enumerate().find(|(_, row)| row.contains('S')).unwrap().0;
    let sx = data.lines().nth(sy).unwrap().chars().enumerate().find(|(_, cell)| cell == &'S').unwrap().0;
    let start = (sy as i64, sx as i64);
    let height = data.lines().count() as i64;
    let width = data.lines().next().unwrap().len() as i64;

    let answer_one = part_one(&rock_locations, start, height, width, 64);
    println!("Part one: {}", answer_one);
    let answer_two = part_one(&rock_locations, start, height, width, 600);
    // println!("Part two: {}", answer_two);
}

fn part_one(rocks: &HashSet<(i64, i64)>, start: (i64, i64), height: i64, width: i64, steps: u32) -> i64 {
    let mut searchfront = HashSet::<(i64, i64)>::new();
    searchfront.insert(start);

    let mut x_repeat = 1;
    let mut y_repeat = 1;
    for _step in 1..=steps {
        let mut next_searchfront = HashSet::<(i64, i64)>::new();
        for previous in searchfront {
            if !rocks.contains(&((previous.0 - 1).rem_euclid(height), previous.1.rem_euclid(width))) {
                next_searchfront.insert((previous.0 - 1, previous.1));
            }
            if !rocks.contains(&(previous.0.rem_euclid(height), (previous.1 - 1).rem_euclid(width))) {
                next_searchfront.insert((previous.0, previous.1 - 1));
            }
            if !rocks.contains(&((previous.0 + 1).rem_euclid(height), previous.1.rem_euclid(width))) {
                next_searchfront.insert((previous.0 + 1, previous.1));
            }
            if !rocks.contains(&(previous.0.rem_euclid(height), (previous.1 + 1).rem_euclid(width))) {
                next_searchfront.insert((previous.0, previous.1 + 1));
            }
        }

        searchfront = next_searchfront;
        // if searchfront.contains(&(start.0, start.1 - (width * x_repeat))) {
        //     println!("x-repeat in {}. Size: {}", _step + 1, searchfront.len());
        //     x_repeat += 1;
        // }
        // if searchfront.contains(&(start.0 - (height * y_repeat), start.1)) {
        //     println!("y-repeat in {}. Size: {}", _step + 1, searchfront.len());
        //     y_repeat += 1;
        // }
        if vec![131, 196, 262, 327, 393, 458].contains(&_step) {
            println!("Step {} - {}", _step, searchfront.len());
        }
    }

    searchfront.len() as i64
}
