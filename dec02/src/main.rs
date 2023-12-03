use input_curler::input_for;
use regex::Regex;

struct Game {
    id: u32,
    pulls: Vec<(u32, u32, u32)>
}

fn main() {
    let data = input_for(2).unwrap();
    let games = parse_data(&data);

    let answer_one = part_one(&games);
    println!("Part one: {}", answer_one);
    let answer_two = part_two(&games);
    println!("Part two: {}", answer_two);
}

fn parse_data(data: &String) -> Vec<Game> {
    let red_rex = Regex::new(r"(\d+) red").unwrap();
    let green_rex = Regex::new(r"(\d+) green").unwrap();
    let blue_rex = Regex::new(r"(\d+) blue").unwrap();

    data.lines().map(|line| {
        let (name_part, pulls_part) = line.split_once(':').unwrap();
        let (_, id_part) = name_part.split_once(' ').unwrap();
        let id = id_part.parse::<u32>().unwrap();
        let pulls = pulls_part.split(';').map(|pull| {
            let red = red_rex.captures(pull).map_or(0, |b| b.get(1).map(|m| m.as_str()).map(|v| v.parse::<u32>().unwrap()).unwrap());
            let green = green_rex.captures(pull).map_or(0, |b| b.get(1).map(|m| m.as_str()).map(|v| v.parse::<u32>().unwrap()).unwrap());
            let blue = blue_rex.captures(pull).map_or(0, |b| b.get(1).map(|m| m.as_str()).map(|v| v.parse::<u32>().unwrap()).unwrap());
            (red, green, blue)
        }).collect();
        Game {
            id,
            pulls
        }
    }).collect()
}

fn part_one(games: &Vec<Game>) -> u32 {
    games.iter().filter(|game| {
        game.pulls.iter().all(|pull| {
            pull.0 <= 12 &&
            pull.1 <= 13 &&
            pull.2 <= 14
        })
    }).map(|game| game.id).sum()
}

fn part_two(games: &Vec<Game>) -> u32 {
    games.iter().map(|game| {
        let max_red = game.pulls.iter().map(|p| p.0).max().unwrap();
        let max_green = game.pulls.iter().map(|p| p.1).max().unwrap();
        let max_blue = game.pulls.iter().map(|p| p.2).max().unwrap();
        max_red * max_green * max_blue
    }).sum()
}
