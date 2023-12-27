use std::ops::{Add, RangeInclusive};

use input_curler::input_for;
use nalgebra::{Matrix6, Vector6};
use regex::Regex;


#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}
impl Add<&Point> for Point {
    type Output = Point;
    fn add(self, rhs: &Point) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        self.add(&rhs)
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct Line {
    point_a: Point,
    point_b: Point,
    vel: Point
}

fn main() {
    let data = input_for(24).unwrap();

    let line_re = Regex::new(r"^(?<x>-?\d+),\s+(?<y>-?\d+),\s+(?<z>-?\d+)\s+@\s+(?<dx>-?\d+),\s+(?<dy>-?\d+),\s+(?<dz>-?\d+)$").unwrap();
    let lines = data.lines().map(|data_line| {
        let captures = line_re.captures(data_line).unwrap();
        let point_a = Point {
            x: captures.name("x").unwrap().as_str().parse().unwrap(),
            y: captures.name("y").unwrap().as_str().parse().unwrap(),
            z: captures.name("z").unwrap().as_str().parse().unwrap(),
        };
        let vel = Point {
            x: captures.name("dx").unwrap().as_str().parse().unwrap(),
            y: captures.name("dy").unwrap().as_str().parse().unwrap(),
            z: captures.name("dz").unwrap().as_str().parse().unwrap(),
        };
        let point_b = point_a + vel;
        Line {
            point_a, point_b, vel
        }
    }).collect::<Vec<Line>>();

    let answer_one = part_one(&lines, 200000000000000f64..=400000000000000f64);
    println!("Part one: {}", answer_one);

    let answer_two = part_two(&lines);
    println!("Part two: {} (approx, for some reason)", answer_two);
}

fn part_one(lines: &[Line], window: RangeInclusive<f64>) -> usize {
    let mut count = 0;
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
    let denom = (line_a.point_a.x - line_a.point_b.x)*(line_b.point_a.y - line_b.point_b.y) -
        (line_a.point_a.y - line_a.point_b.y)*(line_b.point_a.x - line_b.point_b.x);
    let numer_t = (line_a.point_a.x - line_b.point_a.x)*(line_b.point_a.y - line_b.point_b.y) -
        (line_a.point_a.y - line_b.point_a.y)*(line_b.point_a.x - line_b.point_b.x);
    let numer_u = (line_a.point_a.x - line_b.point_a.x)*(line_a.point_a.y - line_a.point_b.y) -
        (line_a.point_a.y - line_b.point_a.y)*(line_a.point_a.x - line_a.point_b.x);

    if denom == 0.0 || (numer_t < 0.0) != (denom < 0.0) || (numer_u < 0.0) != (denom < 0.0) {
        return None;
    }

    let t = numer_t / denom;
    let isect = Point {
        x: line_a.point_a.x + t * (line_a.point_b.x - line_a.point_a.x),
        y: line_a.point_a.y + t * (line_a.point_b.y - line_a.point_a.y),
        z: 0.0
    };
    Some(isect)
}


fn part_two(lines: &[Line]) -> i64 {
    // We have 6 unknowns - position x,y,z and velocity u,v,w - plus an additional unknown - intersection ti - per
    // hailstone used to calculate. So for n hailstones, we have 6 + n unknowns and 3n equations. So n = 3
    // hailstones can solve the system. Call those hailstone A,B,C. Call my throw position/velocity P.
    //
    // Then:
    // Ax + Au*t0 = Px + Pu*t0
    // Ay + Av*t0 = Py + Pv*t0
    // Az + Aw*t0 = Pz + Pw*t0 (and similarly for B & C)
    //
    // So:
    // [t0 =] (Ax - Px) / (Pu - Au) = (Ay - Py) / (Pv - Av) = (Az - Pz) / (Pw - Aw)
    // [t1 =] (Bx - Px) / (Pu - Bu) = (By - Py) / (Pv - Bv) = (Bz - Pz) / (Pw - Bw)
    // [t2 =] (Cx - Px) / (Pu - Cu) = (Cy - Py) / (Pv - Cv) = (Cz - Pz) / (Pw - Cw)
    //
    // Considering the Px <-> Py pairs:
    // (Ax - Px)(Pv - Av) = (Ay - Py)(Pu - Au)
    // => AxPv - AxAv - PxPv + PxAv = AyPu - AyAu - PyPu + PyAu
    // => PxPv - PyPu = AxPv + AvPx - AyPu - AuPy + AyAu - AxAv {1}
    // where the RHS is linear in Px, Py, Pu, Pv. Also:
    // => PxPv - PyPu = BxPv + BvPx - ByPu - BuPy + ByBu - BxBv {2}
    // => PxPv - PyPu = CxPv + CvPx - CyPu - CuPy + CyCu - CxCv {3}
    //
    // So {1} = {2}:
    // AxPv + AvPx - AyPu - AuPy + AyAu - AxAv = BxPv + BvPx - ByPu - BuPy + ByBu - BxBv
    // => (Av - Bv)Px + (Bu - Au)Py + (By - Ay)Pu + (Ax - Bx)Pv = AxAv - AyAu + ByBu - BxBv
    //
    // And similarly for A/C-x/y; A/B-x/z; A/C-x/z; A/B-y/z; A/C-y/z:
    // (Av - Bv)Px + (Bu - Au)Py + (By - Ay)Pu + (Ax - Bx)Pv = AxAv - AyAu + ByBu - BxBv
    // (Av - Cv)Px + (Cu - Au)Py + (Cy - Ay)Pu + (Ax - Cx)Pv = AxAv - AyAu + CyCu - CxCv
    // (Aw - Bw)Px + (Bu - Au)Pz + (Bz - Az)Pu + (Ax - Bx)Pw = AxAw - AzAu + BzBu - BxBw
    // (Aw - Cw)Px + (Cu - Au)Pz + (Cz - Az)Pu + (Ax - Cx)Pw = AxAw - AzAu + CzCu - CxCw
    // (Aw - Bw)Py + (Bv - Av)Pz + (Bz - Az)Pv + (Ay - By)Pw = AyAw - AzAv + BzBv - ByBw
    // (Aw - Cw)Py + (Cv - Av)Pz + (Cz - Az)Pv + (Ay - Cy)Pw = AyAw - AzAv + CzCv - CyCw
    //
    // ... which is 6 linear equations in 6 unknowns. From here I could manipulate symbols further, but it'll be
    // easier to throw it to a matrix solver.
    let (line_a, line_b, line_c) = (lines[0], lines[1], lines[2]);
    let (ax, ay, az) = (line_a.point_a.x, line_a.point_a.y, line_a.point_a.z);
    let (au, av, aw) = (line_a.vel.x, line_a.vel.y, line_a.vel.z);
    let (bx, by, bz) = (line_b.point_a.x, line_b.point_a.y, line_b.point_a.z);
    let (bu, bv, bw) = (line_b.vel.x, line_b.vel.y, line_b.vel.z);
    let (cx, cy, cz) = (line_c.point_a.x, line_c.point_a.y, line_c.point_a.z);
    let (cu, cv, cw) = (line_c.vel.x, line_c.vel.y, line_c.vel.z);
    let coeff_matrix = Matrix6::<f64>::new(
        av - bv, bu - au,     0.0, by - ay, ax - bx,     0.0,
        av - cv, cu - au,     0.0, cy - ay, ax - cx,     0.0,
        aw - bw,     0.0, bu - au, bz - az,     0.0, ax - bx,
        aw - cw,     0.0, cu - au, cz - az,     0.0, ax - cx,
            0.0, aw - bw, bv - av,     0.0, bz - az, ay - by,
            0.0, aw - cw, cv - av,     0.0, cz - az, ay - cy
    );
    let scalar = Vector6::<f64>::new(
        ax*av - ay*au + by*bu - bx*bv,
        ax*av - ay*au + cy*cu - cx*cv,
        ax*aw - az*au + bz*bu - bx*bw,
        ax*aw - az*au + cz*cu - cx*cw,
        ay*aw - az*av + bz*bv - by*bw,
        ay*aw - az*av + cz*cv - cy*cw,
    );
    let inverse = coeff_matrix.try_inverse().unwrap();
    let answer = inverse * scalar;
    println!("Answer: {:?}", answer);
    (answer[(0, 0)].round() as i64) + (answer[(1, 0)].round() as i64) + (answer[(2, 0)].round() as i64)
}
