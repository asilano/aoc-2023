use std::collections::HashMap;
use num::integer::lcm;

use input_curler::input_for;
use regex::Regex;

struct Location<'a> {
    left: &'a str,
    right: &'a str
}
type DesertMap<'a> = HashMap<&'a str, Location<'a>>;

fn main() {
   let data = input_for(8).unwrap();
//      let data = "LR

// 11A = (11B, XXX)
// 11B = (XXX, 11Z)
// 11Z = (11B, XXX)
// 22A = (22B, XXX)
// 22B = (22C, 22C)
// 22C = (22Z, 22Z)
// 22Z = (22B, 22B)
// XXX = (XXX, XXX)".to_string();
    let (directions, locations) = parse_data(&data);

    let answer_one = part_one(directions, &locations);
    println!("Part one: {}", answer_one);
    let answer_two = part_two(directions, &locations);
    println!("Part two: {}", answer_two);
}

fn parse_data (data: &str) -> (&str, DesertMap) {
    let mut lines = data.lines();
    let directions = lines.next().unwrap();

    let loc_re = Regex::new(r"^(?<id>...) = \((?<left>...), (?<right>...)\)$").unwrap();
    let mut locations = DesertMap::new();
    for line in lines.skip(1) {
        let captures = loc_re.captures(line).unwrap();
        let id = captures.name("id").unwrap().as_str();
        let location = Location {
            left: captures.name("left").unwrap().as_str(),
            right: captures.name("right").unwrap().as_str()
        };
        locations.insert(id, location);
    }

    (directions, locations)
}

fn part_one(directions: &str, locations: &DesertMap) -> u32 {
    let mut steps = 0u32;
    let mut current_id = "AAA";
    let mut cyclic_directions = directions.chars().cycle();

    while current_id != "ZZZ" {
        steps += 1;
        let current = locations.get(current_id).unwrap();
        if cyclic_directions.next().unwrap() == 'L' {
            current_id = current.left;
        } else {
            current_id = current.right;
        }
    }

    steps
}

fn part_two(directions: &str, locations: &DesertMap) -> u64 {
    let current_ids = locations
        .keys()
        .filter_map(|k| if k.ends_with('A') {
            Some(*k)
        } else {
            None
        }).collect::<Vec<&str>>();

    current_ids.iter().map(|start| {
        let mut current_id = *start;
        let mut cyclic_directions = directions.chars().cycle();
        let mut steps = 0u64;
        while !current_id.ends_with('Z') {
            steps += 1;
            let current = locations.get(current_id).unwrap();
            if cyclic_directions.next().unwrap() == 'L' {
                current_id = current.left;
            } else {
                current_id = current.right;
            }
        }
        steps
    }).reduce(lcm).unwrap()

}
