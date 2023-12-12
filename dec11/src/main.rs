use input_curler::input_for;

fn main() {
    let data = input_for(11).unwrap();
//     let data = "...#......
// .......#..
// #.........
// ..........
// ......#...
// .#........
// .........#
// ..........
// .......#..
// #...#.....".to_string();
    let galaxies = parse_data(&data);

    let answer_one = total_distance(&galaxies, 1);
    println!("Part one: {}", answer_one);
    let answer_one = total_distance(&galaxies, 999999);
    println!("Part one: {}", answer_one);
}

fn parse_data(data: &str) -> Vec<(u64, u64)> {
    let mut galaxies = vec![];

    for (y, line) in data.lines().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '#' {
                galaxies.push((y as u64, x as u64));
            }
        }
    }

    galaxies
}

fn total_distance(galaxies: &[(u64, u64)], expansion: u64) -> u64 {
    let mut empty_cols = Vec::<u64>::new();
    for col in 0..galaxies.iter().map(|&(_, x)| x).max().unwrap() {
        if !galaxies.iter().any(|&(_, x)| x == col) {
            empty_cols.push(col);
        }
    }
    let mut empty_rows = Vec::<u64>::new();
    for row in 0..galaxies.iter().map(|&(y, _)| y).max().unwrap() {
        if !galaxies.iter().any(|&(y, _)| y == row) {
            empty_rows.push(row);
        }
    }

    let mut total = 0;
    for (ix, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(ix) {
            total += g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1);
            let empties = empty_cols.iter().filter(|col| (g1.1..g2.1).contains(col) || (g2.1..g1.1).contains(col)).count() +
                empty_rows.iter().filter(|row| (g1.0..g2.0).contains(row) || (g2.0..g1.0).contains(row)).count();
            total += empties as u64 * expansion;
        }
    }
    total
}
