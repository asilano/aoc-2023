use std::ops::Range;

use input_curler::input_for;

#[derive(Debug)]
struct MapSegment {
    source: Range<i64>,
    difference: i64
}
impl MapSegment {
    fn new(dest_start: i64, source_start: i64, range_len: i64) -> Self {
        Self {
            source: source_start..(source_start + range_len),
            difference: dest_start - source_start
        }
    }

    fn apply(&self, input: i64) -> Option<i64> {
        if self.source.contains(&input) {
            Some(input + self.difference)
        } else {
            None
        }
    }
}
type ConditionMap = Vec<MapSegment>;

fn main() {
    let data = input_for(5).unwrap();

    {
        let (seeds, condition_maps) = parse_data_one(&data);
        let answer_one = part_one(&seeds, &condition_maps);
        println!("Part one: {}", answer_one);
    }

    {
        let (seeds, condition_maps) = parse_data_two(&data);
        let answer_two = part_two(&seeds, &condition_maps);
        println!("Part two: {}", answer_two);
    }
}

fn parse_data_one(data: &String) -> (Vec<i64>, Vec<ConditionMap>) {
    let mut lines = data.lines().filter(|l| !l.is_empty());

    let seeds_line = lines.next().unwrap();
    let seeds = seeds_line
        .split_whitespace()
        .skip(1) // The word "seeds:"
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let condition_maps = parse_condition_maps(&mut lines);

    (seeds, condition_maps)
}

fn parse_data_two(data: &String) -> (Vec<Range<i64>>, Vec<ConditionMap>) {
    let mut lines = data.lines().filter(|l| !l.is_empty());

    let seeds_line = lines.next().unwrap();
    let seeds = seeds_line
        .split_whitespace()
        .skip(1) // The word "seeds:"
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
        .chunks(2)
        .map(|pair| pair[0]..(pair[0]+pair[1]))
        .collect::<Vec<Range<i64>>>();
    let condition_maps = parse_condition_maps(&mut lines);

    (seeds, condition_maps)
}

fn parse_condition_maps(lines: &mut dyn Iterator<Item = &str>) -> Vec<ConditionMap> {
    let mut condition_maps = Vec::<ConditionMap>::new();
    let mut current_map = ConditionMap::new();
    let mut first = true;
    for line in lines {
        if line.ends_with("map:") {
            if !first { condition_maps.push(current_map) }
            current_map = ConditionMap::new();
            first = false;
            continue;
        }

        let mut numbers = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
        current_map.push(MapSegment::new(numbers.next().unwrap(), numbers.next().unwrap(), numbers.next().unwrap()));
    }
    condition_maps.push(current_map);

    condition_maps
}

fn part_one(seeds: &[i64], condition_maps: &[ConditionMap]) -> i64 {
    let locations = seeds.iter().map(|seed| {
        let mut output = *seed;

        for condition_map in condition_maps {
            output = condition_map
                .iter()
                .filter_map(|segment| segment.apply(output))
                .next()
                .unwrap_or(output);
        }

        output
    }).collect::<Vec<i64>>();
    *locations.iter().min().unwrap()
}

fn part_two(seeds: &[Range<i64>], condition_maps: &[ConditionMap]) -> i64 {
    let mut working_copy = seeds.iter().map(|r| r.clone()).collect::<Vec<Range<i64>>>();
    let mut next_ranges = Vec::<Range<i64>>::new();

    for condition_map in condition_maps {
        for range in &working_copy {
            let mut maybe_working_range = Some(range.clone());

            while let Some(ref working_range) = maybe_working_range {
                let maybe_segment = condition_map
                    .iter()
                    .find(|segment| segment.apply(working_range.start).is_some());

                if let Some(segment) = maybe_segment {
                    // Need to find the last entry in the range that is handled by the segment
                    if segment.source.end >= working_range.end {
                        // All covered by the segment
                        next_ranges.push((working_range.start + segment.difference)..(working_range.end + segment.difference));
                        maybe_working_range = None;
                    } else {
                        next_ranges.push((working_range.start + segment.difference)..(segment.source.end + segment.difference));
                        maybe_working_range = Some(segment.source.end..working_range.end);
                    }
                } else {
                    // Need to find the first entry in the range that is actually mapped
                    let maybe_subrange_start = condition_map
                        .iter()
                        .find(|segment| working_range.contains(&segment.source.start))
                        .map(|segment| segment.source.start);

                    if let Some(subrange_start) = maybe_subrange_start {
                        next_ranges.push(working_range.start..subrange_start);
                        maybe_working_range = Some(subrange_start..working_range.end);
                    } else {
                        next_ranges.push(working_range.clone());
                        maybe_working_range = None;
                    }
                }
            }
        }
        working_copy = next_ranges;
        next_ranges = vec![];
    }

    working_copy.iter().map(|r| r.start).min().unwrap()
}