use input_curler::input_for;

#[derive(Debug, Clone, Copy)]
struct Pipe {
    location: (usize, usize),
    shape: char,
    exits: Option<[(usize, usize); 2]>,
    dist_from_s: Option<usize>,
    inside: Option<bool>
}
impl Pipe {
    fn new(location: (usize, usize), shape: char, max_x: usize, max_y: usize) -> Self {
        if shape == '.' {
            return Self {
                location,
                shape,
                exits: None,
                dist_from_s: None,
                inside: None
            }
        }

        let maybe_exits = match (location, shape) {
            ((0, _), '|') | ((0, _), 'L') | ((0, _), 'J') |
                ((_, 0), '-') | ((_, 0), 'J') | ((_, 0), '7') => None,
            ((y, _), '|') | ((y, _), '7') | ((y, _), 'F') if y == max_y => None,
            ((_, x), '-') | ((_, x), 'F') | ((_, x), 'L') if x == max_x => None,
            ((y, x), '|') => Some([(y - 1, x), (y + 1, x)]),
            ((y, x), '-') => Some([(y, x - 1), (y, x + 1)]),
            ((y, x), 'F') => Some([(y + 1, x), (y, x + 1)]),
            ((y, x), 'J') => Some([(y - 1, x), (y, x - 1)]),
            ((y, x), '7') => Some([(y + 1, x), (y, x - 1)]),
            ((y, x), 'L') => Some([(y - 1, x), (y, x + 1)]),
            _ => unreachable!()
        };
        Self {
            location,
            shape,
            exits: maybe_exits,
            dist_from_s: None,
            inside: None
        }
    }

    fn exits_to(&self, to_loc: (usize, usize)) -> bool {
        if let Some(exits) = self.exits {
            exits.contains(&to_loc)
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
struct PipeMap {
    pipes: Vec<Vec<Pipe>>
}
impl PipeMap {
    fn at_mut(&mut self, (row, col): (usize, usize)) -> &mut Pipe {
        &mut self.pipes[row][col]
    }
    fn at(&self, (row, col): (usize, usize)) -> &Pipe {
        &self.pipes[row][col]
    }

    fn height(&self) -> usize {
        self.pipes.len()
    }
    fn width(&self) -> usize {
        self.pipes[0].len()
    }
}

fn main() {
    let data = input_for(10).unwrap();
//     let data = "FF7FSF7F7F7F7F7F---7
// L|LJ||||||||||||F--J
// FL-7LJLJ||||||LJL-77
// F--JF--7||LJLJ7F7FJ-
// L---JF-JLJ.||-FJLJJ7
// |F|F-JF---7F7-L7L|7|
// |FFJF7L7F-JF7|JL---7
// 7-L-JL7||F7|L7F-7F7|
// L.L7LFJ|||||FJL7||LJ
// L7JLJL-JLJLJL--JLJ.L".to_string();

    let (mut pipes, start) = parse_data(&data);
    let answer_one = part_one(&mut pipes, start);
    println!("Part one: {}", answer_one);
    let answer_two = part_two(&mut pipes);
    println!("Part two: {}", answer_two);
}

fn parse_data(data: &str) -> (PipeMap, (usize, usize)) {
    let mut pipes = Vec::<Vec<Pipe>>::new();
    let mut start = (0, 0);

    let height = data.lines().count();
    let width = data.lines().next().unwrap().len();
    for (y, line) in data.lines().enumerate() {
        let mut row = Vec::<Pipe>::new();
        for (x, cell) in line.chars().enumerate() {
            if cell == 'S' {
                start = (y, x);
                row.push(Pipe {
                    location: start,
                    shape: 'S',
                    exits: None,
                    dist_from_s: Some(0),
                    inside: None
                })
            } else {
                row.push(Pipe::new((y, x), cell, width - 1, height - 1));
            }
        }
        pipes.push(row);
    }

    (PipeMap { pipes }, start)
}

fn part_one(pipes: &mut PipeMap, start: (usize, usize)) -> usize {
    let mut search_front = Vec::<(usize, usize)>::new();
    if start.0 != 0 && pipes.at((start.0 - 1, start.1)).exits_to(start) {
        search_front.push((start.0 - 1, start.1));
    }
    if start.0 != pipes.height() - 1 && pipes.at((start.0 + 1, start.1)).exits_to(start) {
        search_front.push((start.0 + 1, start.1));
    }
    if start.1 != 0 && pipes.at((start.0, start.1 - 1)).exits_to(start) {
        search_front.push((start.0, start.1 - 1));
    }
    if start.1 != pipes.width() - 1 && pipes.at((start.0, start.1 + 1)).exits_to(start) {
        search_front.push((start.0, start.1 + 1));
    }

    if search_front.len() == 2 {
        let start_pipe = pipes.at_mut(start);
        if search_front.contains(&(start.0 - 1, start.1)) {
            if search_front.contains(&(start.0, start.1 - 1)) {
                start_pipe.shape = 'J';
            } else if search_front.contains(&(start.0, start.1 + 1)) {
                start_pipe.shape = 'L';
            } else if search_front.contains(&(start.0 + 1, start.1)) {
                start_pipe.shape = '|';
            }
        } else if search_front.contains(&(start.0, start.1 - 1)) {
            if search_front.contains(&(start.0, start.1 + 1)) {
                start_pipe.shape = '-';
            } else if search_front.contains(&(start.0 + 1, start.1)) {
                start_pipe.shape = '7';
            }
        } else {
            start_pipe.shape = 'F';
        }
    }

    let mut steps = 1;

    loop {
        for &loc in search_front.iter() {
            let pipe = pipes.at_mut(loc);
            if let Some(dist) = pipe.dist_from_s {
                assert_eq!(dist, steps);
                return steps;
            }
            pipe.dist_from_s = Some(steps);
        }

        search_front = search_front.iter().filter_map(|&loc| {
            if let Some(exits) = pipes.at(loc).exits {
                if pipes.at(exits[0]).dist_from_s.is_none() {
                    Some(exits[0])
                } else if pipes.at(exits[1]).dist_from_s.is_none() {
                    Some(exits[1])
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();

        if search_front.is_empty() { panic!() }
        steps += 1;
    }
}

fn part_two(pipes: &mut PipeMap) -> usize {
    for row in pipes.pipes.iter_mut() {
        for cell in row.iter_mut() {
            if cell.dist_from_s.is_none() {
                cell.shape = '.';
            }
        }
    }

    let mut count = 0;
    for row  in 0..pipes.height() {
        for col in 0..pipes.width() {
            if pipes.at((row, col)).shape == '.' {
                print!("({}, {})", row, col);
                let inside = ray_cast_inside(pipes, (row, col));
                pipes.at_mut((row, col)).inside = Some(inside);

                if inside {
                    count += 1;
                }
                println!(": {}", inside);
            }
        }
    }

    count
}

#[derive(Debug, PartialEq, Eq)]
enum Directions {
    W,
    NW,
    N
}
use Directions::*;
fn ray_cast_inside(pipes: &PipeMap, mut from: (usize, usize)) -> bool {
    let mut odd_crossings = false;
    let mut direction = W;

    loop {
        if (direction == W || direction == NW) && from.1 == 0 {
            return odd_crossings;
        }
        if (direction == N || direction == NW) && from.0 == 0 {
            return odd_crossings;
        }

        from = match direction {
            W => (from.0, from.1 - 1),
            NW => (from.0 - 1, from.1 - 1),
            N => (from.0 - 1, from.1),
        };
        let pipe = pipes.at(from);
        match (&direction, pipe.shape) {
            (_, '.') => {
                return pipe.inside.unwrap() ^ odd_crossings;
            },
            (W, '|') | (N, '-') | (NW, 'J') | (NW, 'F') => { odd_crossings = !odd_crossings },
            (W, 'J') => {
                odd_crossings = !odd_crossings;
                direction = NW;
            },
            (W, '7') => { direction = NW },
            (NW, '7') | (NW, 'L') => {},
            (NW, '|') => {
                odd_crossings = !odd_crossings;
                direction = W;
            },
            (NW, '-') => {
                odd_crossings = !odd_crossings;
                direction = N;
            },
            (N, 'J') => {
                odd_crossings = !odd_crossings;
                direction = NW;
            },
            (N, 'L') => { direction = NW },
            _ => {
                println!("({},{}), {:?}, {}", from.0, from.1, direction, pipe.shape);
                unreachable!()
            }
        }
    }
}
