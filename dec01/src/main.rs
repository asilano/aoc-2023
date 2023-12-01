use input_curler::input_for;
use regex::Regex;

fn main() {
    let data = input_for(1).unwrap();

    let answer_one = part_one(&data);
    println!("Part one: {}", answer_one);

    let answer_two = part_two(&data);
    println!("Part two: {}", answer_two);
}

fn part_one(data: &String) -> u32 {
    data.lines().map(|line| {
        let first = line.chars().find(char::is_ascii_digit).unwrap();
        let last = line.chars().rfind(char::is_ascii_digit).unwrap();
        first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
    }).sum()
}

fn part_two(data: &String) -> u32 {
    let number_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let number_regex = Regex::new(
        format!(r"({}|\d)", number_words.join("|")).as_str()
    ).unwrap();
    let reverse_regex = Regex::new(
        format!(r"({}|\d)", number_words.map(|s| s.chars().rev().collect::<String>()).join("|")).as_str()
    ).unwrap();
    data.lines().map(|line| {
        let first_cap = number_regex.find(line).unwrap();
        let line_rev = line.chars().rev().collect::<String>();
        let last_cap = reverse_regex.find(line_rev.as_str()).unwrap();

        let first = if let Ok(num) = first_cap.as_str().parse::<u32>() {
            num
        } else {
            number_words.iter().position(|&w| w == first_cap.as_str()).unwrap() as u32 + 1
        };
        let last = if let Ok(num) = last_cap.as_str().parse::<u32>() {
            num
        } else {
            number_words.iter().position(|&w| w.chars().rev().collect::<String>() == last_cap.as_str()).unwrap() as u32 + 1
        };

        first * 10 + last
    }).sum()

}