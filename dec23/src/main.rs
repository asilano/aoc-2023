use std::collections::HashMap;

use input_curler::input_for;


struct Segment {
    size: usize,
    next_regions: Vec<usize>,
    bidir_next_regions: Vec<usize>
}

fn main() {
    let data = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#".to_string();
    let data = input_for(23).unwrap();

    let height = data.lines().count();
    // let width = data.lines().next().unwrap().len();
    let (segments, start, end) = parse_trails(&data, height);

    let answer_one = part_one(&segments, start, end);
    println!("Part one: {}", answer_one);
    let answer_two = part_two(&segments, start, end);
    println!("Part two: {}", answer_two);
}

fn parse_trails(data: &str, height: usize) -> (Vec<Segment>, usize, usize) {
    let mut trail_map = HashMap::new();
    let mut slopes = Vec::new();
    convert_to_trail_cells(data, &mut trail_map, &mut slopes);

    let mut next_label = 0usize;
    while let Some((&exemplar, _)) = trail_map.iter().find(|(_, label)| label.is_none()) {
        label_segment_by_flood(&mut trail_map, next_label, exemplar);
        next_label += 1;
    }

    let mut segments = (0..next_label).map(|label| {
        let size = trail_map.values().filter(|&&v| v == Some(label)).count();
        Segment {
            size,
            next_regions: vec![],
            bidir_next_regions: vec![]
        }
    }).collect::<Vec<Segment>>();

    for slope in slopes {
        let from = trail_map.get(&slope.0).unwrap().unwrap();
        let to = trail_map.get(&slope.1).unwrap().unwrap();
        segments.get_mut(from).unwrap().next_regions.push(to);
        segments.get_mut(from).unwrap().bidir_next_regions.push(to);
        segments.get_mut(to).unwrap().bidir_next_regions.push(from);
    }

    let start = trail_map.iter().find_map(|(cell, label)| {
        if cell.0 == 0 { *label } else { None }
    }).unwrap();
    let end = trail_map.iter().find_map(|(cell, label)| {
        if cell.0 == height - 1 { *label } else { None }
    }).unwrap();

    (segments, start, end)
}

fn convert_to_trail_cells(
    data: &str,
    trail_map: &mut HashMap<(usize, usize), Option<usize>>,
    slopes: &mut Vec<((usize, usize), (usize, usize))>
 )
{
    for (row, line) in data.lines().enumerate() {
        for (col, cell) in line.chars().enumerate() {
            match cell {
                '.' => { trail_map.insert((row, col), None); },
                'v' => { slopes.push(((row - 1, col), (row + 1, col))); },
                '^' => { slopes.push(((row + 1, col), (row - 1, col))); },
                '>' => { slopes.push(((row, col - 1), (row, col + 1))); },
                '<' => { slopes.push(((row, col + 1), (row, col - 1))); },
                '#' => {},
                _ => unreachable!()
            }
        }
    }
}

fn label_segment_by_flood(trail_map: &mut HashMap<(usize, usize), Option<usize>>, label: usize, exemplar: (usize, usize)) {
    // Assume paths are one cell wide
    let mut searchfront = vec![exemplar];
    while let Some(current) = searchfront.pop() {
        let map_entry = trail_map.get_mut(&current).unwrap();
        if map_entry.is_none() {
            *map_entry = Some(label);
            if current.0 > 0 && trail_map.contains_key(&(current.0 - 1, current.1)) {
                searchfront.push((current.0 - 1, current.1));
            }
            if current.1 > 0 && trail_map.contains_key(&(current.0, current.1 - 1)) {
                searchfront.push((current.0, current.1 - 1));
            }
            if trail_map.contains_key(&(current.0 + 1, current.1)) {
                searchfront.push((current.0 + 1, current.1));
            }
            if trail_map.contains_key(&(current.0, current.1 + 1)) {
                searchfront.push((current.0, current.1 + 1));
            }
        }
    }
}

fn part_one(segments: &Vec<Segment>, start: usize, end: usize) -> usize {
    let mut longest = 0;
    let current = 0;
    let mut visited = Vec::<usize>::new();

    walk(segments, start, end, current, &mut visited, &mut longest, false);
    longest - 1
}

fn part_two(segments: &Vec<Segment>, start: usize, end: usize) -> usize {
    let mut longest = 0;
    let current = 0;
    let mut visited = Vec::<usize>::new();

    walk(segments, start, end, current, &mut visited, &mut longest, true);
    longest - 1
}

fn walk(
    segments: &Vec<Segment>,
    index: usize,
    end: usize,
    mut current: usize,
    visited: &mut Vec<usize>,
    longest: &mut usize,
    bidir: bool)
{
    if visited.contains(&index) { return; }

    let segment = &segments[index];
    current += segment.size;

    if index == end {
        if current > *longest {
            *longest = current;
        }
    } else {
        visited.push(index);
        let next_regions = if bidir {
            &segment.bidir_next_regions
        } else {
            &segment.next_regions
        };
        for &next_ix in next_regions.iter() {
            walk(segments, next_ix, end, current + 1, visited, longest, bidir);
        }
        visited.pop();
    }
}