fn main() {
//     let data = r".|...\....
// |.-.\.....
// .....|-...
// ........|.
// ..........
// .........\
// ..../.\\..
// .-.-/..|..
// .|....-|.\
// ..//.|....".to_string();
    let data = input_for(16).unwrap();

    let answer_one = part_one(&data);
    println!("Answer one: {}", answer_one);
    let answer_two = part_two(&data);
    println!("Answer two: {}", answer_two);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up
}
use std::collections::{HashMap, VecDeque};

use Direction::*;
use input_curler::input_for;

fn part_one(data: &str) -> usize {
    count_energised(data, (0, 0), Right)
}

fn part_two(data: &str) -> usize {
    let height = data.lines().count();
    let width = data.lines().next().unwrap().len();

    let top_max = (0..width).map(|x|
        count_energised(data, (0, x), Down)
    ).max().unwrap();
    let bottom_max = (0..width).map(|x|
        count_energised(data, (height - 1, x), Up)
    ).max().unwrap();
    let left_max = (0..height).map(|y|
        count_energised(data, (y, 0), Right)
    ).max().unwrap();
    let right_max = (0..height).map(|y|
        count_energised(data, (y, width - 1), Left)
    ).max().unwrap();

    top_max.max(bottom_max).max(left_max).max(right_max)
}

fn count_energised(data: &str, start_cell: (usize, usize), start_dir: Direction) -> usize {
    let height = data.lines().count();
    let width = data.lines().next().unwrap().len();

    let mut energised = HashMap::<(usize, usize), Vec<Direction>>::new();

    let mut beam_front = VecDeque::from([(start_cell, start_dir)]);
    while !beam_front.is_empty() {
        let current = beam_front.pop_front().unwrap();
        let current_posn = current.0;
        let current_dir = current.1;

        let cell = energised.entry(current_posn).or_default();
        if !cell.contains(&current_dir) {
            cell.push(current_dir);

            let contents = data.lines().nth(current_posn.0).unwrap().chars().nth(current_posn.1).unwrap();
            let can_go_right = current_posn.1 < width - 1;
            let can_go_left = current_posn.1 > 0;
            let can_go_down = current_posn.0 < height - 1;
            let can_go_up = current_posn.0 > 0;
            match (contents, current_dir) {
                ('.', _) | ('|', Up) | ('|', Down) | ('-', Right) | ('-', Left) => {
                    match current_dir {
                        Right => {
                            if can_go_right {
                                beam_front.push_back(((current_posn.0, current_posn.1 + 1), current_dir));
                            }
                        },
                        Left => {
                            if can_go_left {
                                beam_front.push_back(((current_posn.0, current_posn.1 - 1), current_dir));
                            }
                        },
                        Down => {
                            if can_go_down {
                                beam_front.push_back(((current_posn.0 + 1, current_posn.1), current_dir));
                            }
                        },
                        Up => {
                            if can_go_up {
                                beam_front.push_back(((current_posn.0 - 1, current_posn.1), current_dir));
                            }
                        },
                    }
                },
                ('/', Right) | ('\\', Left) => {
                    if can_go_up {
                        beam_front.push_back(((current_posn.0 - 1, current_posn.1), Up));
                    }
                },
                ('/', Down) | ('\\', Up) => {
                    if can_go_left {
                        beam_front.push_back(((current_posn.0, current_posn.1 - 1), Left));
                    }
                },
                ('/', Left) | ('\\', Right) => {
                    if can_go_down {
                        beam_front.push_back(((current_posn.0 + 1, current_posn.1), Down));
                    }
                },
                ('/', Up) | ('\\', Down) => {
                    if can_go_right {
                        beam_front.push_back(((current_posn.0, current_posn.1 + 1), Right));
                    }
                },
                ('|', Right) | ('|', Left) => {
                    if can_go_up {
                        beam_front.push_back(((current_posn.0 - 1, current_posn.1), Up));
                    }
                    if can_go_down {
                        beam_front.push_back(((current_posn.0 + 1, current_posn.1), Down));
                    }
                },
                ('-', Up) | ('-', Down) => {
                    if can_go_left {
                        beam_front.push_back(((current_posn.0, current_posn.1 - 1), Left));
                    }
                    if can_go_right {
                        beam_front.push_back(((current_posn.0, current_posn.1 + 1), Right));
                    }
                },
                _ => unreachable!()
            }
        }
    }

    energised.len()
}
