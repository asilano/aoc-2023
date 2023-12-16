use std::collections::HashMap;

use input_curler::input_for;

fn main() {
    // let data = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let data = input_for(15).unwrap();

    let answer_one = part_one(&data);
    println!("Part one: {}", answer_one);
    let answer_two = part_two(&data);
    println!("Part two: {}", answer_two);
}

fn part_one(data: &str) -> u32 {
    data
        .trim_end()
        .split(',')
        .map(reindeer_hash)
        .sum()
}

type Catalogue<'a> = HashMap<&'a str, u32>;
type LensBox<'a> = Vec<&'a str>;

fn part_two(data: &str) -> u32 {
    let mut system: Vec<LensBox> = vec![vec![]; 256];
    let mut catalogue = Catalogue::new();

    for step in data.trim_end().split(',') {
        let mut parts = step.split(&['=', '-']).filter(|p| !p.is_empty());
        let label = parts.next().unwrap();
        let power = parts.next().map(|num| num.parse::<u32>().unwrap());
        let op_add = step.contains('=');

        let box_num = reindeer_hash(label);
        let lens_box = system.get_mut(box_num as usize).unwrap();
        if op_add {
            if !lens_box.iter().any(|&lbl| lbl == label) {
                lens_box.push(label);
            }
            catalogue.insert(label, power.unwrap());
        } else if let Some(pos) = lens_box.iter().position(|&lbl| lbl == label) {
            lens_box.remove(pos);
        }
    }

    system.iter().enumerate().map(|(ix, lens_box)|
        lens_box.iter().enumerate().map(|(slot, label)|
            (ix as u32 + 1) * (slot as u32 + 1) * catalogue.get(label).unwrap()
        ).sum::<u32>()
    ).sum()
}

fn reindeer_hash(step: &str) -> u32 {
    let mut value = 0;
    for c in step.chars() {
        value += c as u32;
        value *= 17;
        value %= 256;
    }

    value
}