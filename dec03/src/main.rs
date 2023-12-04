use std::collections::HashMap;

use input_curler::input_for;
use regex::Regex;

fn main() {
    let data = input_for(3).unwrap();

    let answer_one = part_one(&data);
    println!("Part one: {}", answer_one);

    let answer_two = part_two(&data);
    println!("Part two: {}", answer_two);
}

fn part_one(data: &String) -> u32 {
    let mut symbol_adjacent_locations: Vec<(usize, usize)> = vec![];

    let height = data.lines().count();
    let width = data.lines().next().unwrap().chars().count();
    for (row_num, line) in data.lines().enumerate() {
        for (col_num, cell) in line.chars().enumerate() {
            if !cell.is_ascii_digit() && cell != '.' {
                let cols = if col_num == 0 {
                    vec![0, 1]
                } else if col_num == width - 1 {
                    vec![width - 2, width - 1]
                } else {
                    vec![col_num - 1, col_num, col_num + 1]
                };
                let rows = if row_num == 0 {
                    vec![0, 1]
                } else if row_num == height - 1 {
                    vec![height - 2, height - 1]
                } else {
                    vec![row_num - 1, row_num, row_num + 1]
                };
                cols.iter().for_each(|&col|
                    rows.iter().for_each(|&row|
                        if col != col_num || row != row_num {
                            symbol_adjacent_locations.push((row, col));
                        }
                    )
                );
            }
        }
    }

    let mut sum = 0;
    let num_re = Regex::new(r"\d+").unwrap();
    for (row_num, line) in data.lines().enumerate() {
        for num_match in num_re.find_iter(line) {
            let start = num_match.start();
            let range = start..start + num_match.len();
            if symbol_adjacent_locations.iter().any(|&loc| loc.0 == row_num && range.contains(&loc.1)) {
                sum += num_match.as_str().parse::<u32>().unwrap();
            }
        }
    }
    sum
}

fn part_two(data: &String) -> u32 {
    let mut symbol_adjacent_locations = HashMap::<(usize, usize), Vec::<(usize, usize)>>::new();

    let height = data.lines().count();
    let width = data.lines().next().unwrap().chars().count();
    for (row_num, line) in data.lines().enumerate() {
        for (col_num, cell) in line.chars().enumerate() {
            if cell == '*' {
                let cols = if col_num == 0 {
                    vec![0, 1]
                } else if col_num == width - 1 {
                    vec![width - 2, width - 1]
                } else {
                    vec![col_num - 1, col_num, col_num + 1]
                };
                let rows = if row_num == 0 {
                    vec![0, 1]
                } else if row_num == height - 1 {
                    vec![height - 2, height - 1]
                } else {
                    vec![row_num - 1, row_num, row_num + 1]
                };
                symbol_adjacent_locations.insert((row_num, col_num), vec![]);
                let adjacencies = symbol_adjacent_locations.get_mut(&(row_num, col_num)).unwrap();
                cols.iter().for_each(|&col|
                    rows.iter().for_each(|&row|
                        if col != col_num || row != row_num {
                            adjacencies.push((row, col));
                        }
                    )
                );
            }
        }
    }

    let mut possible_cogs = HashMap::<(usize, usize), Vec<u32>>::new();
    let num_re = Regex::new(r"\d+").unwrap();
    for (row_num, line) in data.lines().enumerate() {
        for num_match in num_re.find_iter(line) {
            let start = num_match.start();
            let range = start..start + num_match.len();
            symbol_adjacent_locations
                .iter()
                .filter(|&(_, locs)| locs.iter().any(|loc| loc.0 == row_num && range.contains(&loc.1)))
                .for_each(|(sym_loc, _)| {
                    let num = num_match.as_str().parse::<u32>().unwrap();
                    if let Some(numbers) = possible_cogs.get_mut(sym_loc) {
                        numbers.push(num);
                    } else {
                        possible_cogs.insert(*sym_loc, vec![num]);
                    }
            });
        }
    }

    possible_cogs.iter().filter_map(|(_, numbers)| if numbers.len() == 2 {
        Some(numbers.iter().product::<u32>())
    } else {
        None
    }).sum()
}
