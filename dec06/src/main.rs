use input_curler::input_for;
use itertools::Itertools;

struct Race {
    time: u64,
    record: u64
}

fn main() {
    let data = input_for(6).unwrap();
// let data = "Time:      7  15   30
// Distance:  9  40  200".to_string();

    let races = parse_data(&data, false);
    let answer_one = part_one(&races);
    println!("Part one: {}", answer_one);

    let races = parse_data(&data, true);
    let answer_two = part_one(&races);
    println!("Part two: {}", answer_two);
}

fn parse_data(data: &str, combine: bool) -> Vec<Race> {
    let mut times_part = data.lines().next().unwrap().split_whitespace().skip(1);
    let mut records_part = data.lines().nth(1).unwrap().split_whitespace().skip(1);

    if combine {
        vec![
            Race {
                time: times_part.join("").parse::<u64>().unwrap(),
                record: records_part.join("").parse::<u64>().unwrap()
            }
        ]
    } else {
        let times = times_part.map(|t| t.parse::<u64>().unwrap());
        let records = records_part.map(|t| t.parse::<u64>().unwrap());

        times.zip(records).map(|(time, record)| Race { time, record } ).collect()
    }

}

fn part_one(races: &[Race]) -> u64 {
    races.iter().map(|race| {
        let min_hold = (0..race.time).find(|hold_time|
            hold_time * (race.time - hold_time) > race.record
        ).unwrap();
        race.time + 1 - min_hold * 2
    }).product()
}