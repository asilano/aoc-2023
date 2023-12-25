use std::ops::{Add, RangeInclusive};

use input_curler::input_for;
use malachite::Rational;
use regex::Regex;


#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Point {
    x: Rational,
    y: Rational,
    z: Rational
}
impl Add<&Point> for Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x.clone(),
            y: self.y + rhs.y.clone(),
            z: self.z + rhs.z.clone(),
        }
    }
}
impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        self.add(&rhs)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Line {
    point_a: Point,
    point_b: Point
}

fn main() {
    let data = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3".to_string();
    let data = input_for(24).unwrap();

    let line_re = Regex::new(r"^(?<x>-?\d+),\s+(?<y>-?\d+),\s+(?<z>-?\d+)\s+@\s+(?<dx>-?\d+),\s+(?<dy>-?\d+),\s+(?<dz>-?\d+)$").unwrap();
    let lines = data.lines().map(|data_line| {
        let captures = line_re.captures(data_line).unwrap();
        let point_a = Point {
            x: captures.name("x").unwrap().as_str().parse().unwrap(),
            y: captures.name("y").unwrap().as_str().parse().unwrap(),
            z: captures.name("z").unwrap().as_str().parse().unwrap(),
        };
        let point_b = point_a.clone() + Point {
            x: captures.name("dx").unwrap().as_str().parse().unwrap(),
            y: captures.name("dy").unwrap().as_str().parse().unwrap(),
            z: captures.name("dz").unwrap().as_str().parse().unwrap(),
        };
        Line {
            point_a, point_b
        }
    }).collect::<Vec<Line>>();

    let answer_one = part_one(&lines, Rational::from(200000000000000i128)..=Rational::from(400000000000000i128));
    println!("Part one: {}", answer_one);
}

fn part_one(lines: &[Line], window: RangeInclusive<Rational>) -> u64 {
    let mut count = 0u64;
    for a in 0..lines.len() {
        for b in a..lines.len() {
            if let Some(isect) = lines_intersect_2d(&lines[a], &lines[b]) {
                if window.contains(&isect.x) && window.contains(&isect.y) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn lines_intersect_2d(line_a: &Line, line_b: &Line) -> Option<Point> {
    let denom = (&line_a.point_a.x - &line_a.point_b.x)*(&line_b.point_a.y - &line_b.point_b.y) -
        (&line_a.point_a.y - &line_a.point_b.y)*(&line_b.point_a.x - &line_b.point_b.x);
    let numer_t = (&line_a.point_a.x - &line_b.point_a.x)*(&line_b.point_a.y - &line_b.point_b.y) -
        (&line_a.point_a.y - &line_b.point_a.y)*(&line_b.point_a.x - &line_b.point_b.x);
    let numer_u = (&line_a.point_a.x - &line_b.point_a.x)*(&line_a.point_a.y - &line_a.point_b.y) -
        (&line_a.point_a.y - &line_b.point_a.y)*(&line_a.point_a.x - &line_a.point_b.x);

    if denom == 0 || numer_t < 0 || numer_u < 0 {
        return None;
    }

    let t = numer_t / denom;
    let isect = Point {
        x: &line_a.point_a.x + &t * (&line_a.point_b.x - &line_a.point_a.x),
        y: &line_a.point_a.y + &t * (&line_a.point_b.y - &line_a.point_a.y),
        z: Rational::from(0)
    };
    Some(isect)
}