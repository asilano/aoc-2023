use input_curler::input_for;

fn main() {
//     let data = "#.##..##.
// ..#.##.#.
// ##......#
// ##......#
// ..#.##.#.
// ..##..##.
// #.#.##.#.

// #...##..#
// #....#..#
// ..##..###
// #####.##.
// #####.##.
// ..##..###
// #....#..#".to_string();
    let data = input_for(13).unwrap();

    let patterns = data.split("\n\n");

    let answer_one = part_one(patterns.clone());
    println!("Part one: {}", answer_one);

    let answer_two = part_two(patterns.clone());
    println!("Part two: {}", answer_two);
}

enum MirrorLoc {
    Row(u32),
    Column(u32)
}

fn part_one<'a, Iter>(patterns: Iter) -> u32
where Iter: Iterator<Item = &'a str>
{
    patterns.map(|pattern| {
        match find_reflection(pattern, false) {
            MirrorLoc::Column(x) => x,
            MirrorLoc::Row(y) => 100 * y
        }
    }).sum()
}

fn part_two<'a, Iter>(patterns: Iter) -> u32
where Iter: Iterator<Item = &'a str>
{
    patterns.map(|pattern| {
        match find_reflection(pattern, true) {
            MirrorLoc::Column(x) => x,
            MirrorLoc::Row(y) => 100 * y
        }
    }).sum()
}

fn find_reflection(pattern: &str, smudge_allowed: bool) -> MirrorLoc {
    if let Some(x) = find_col_reflection(pattern, smudge_allowed) {
        MirrorLoc::Column(x as u32 + 1)
    } else if let Some(y) = find_row_reflection(pattern, smudge_allowed) {
        MirrorLoc::Row(y as u32 + 1)
    } else {
        panic!()
    }
}

fn find_row_reflection(pattern: &str, smudge_allowed: bool) -> Option<usize> {
    let num_lines = pattern.lines().count();

    (0..num_lines - 1).find(|&after_row| {
        let mut inner_smudge_allowed = smudge_allowed;
        (0..=after_row).all(|check_row| {
            let matching_row = 2 * after_row - check_row + 1;
            matching_row >= num_lines ||
                patterns_match(
                    pattern.lines().nth(check_row).unwrap(),
                    pattern.lines().nth(matching_row).unwrap(),
                    &mut inner_smudge_allowed)
        }) && !inner_smudge_allowed
    })
}

fn find_col_reflection(pattern: &str, smudge_allowed: bool) -> Option<usize> {
    let num_cols = pattern.lines().next().unwrap().len();

    (0..num_cols - 1).find(|&after_col| {
        let mut inner_smudge_allowed = smudge_allowed;
        (0..=after_col).all(|check_col| {
            let matching_col = 2 * after_col - check_col + 1;
            matching_col >= num_cols ||
                patterns_match(
                    &pattern.lines().map(|row| row.chars().nth(check_col).unwrap()).collect::<String>(),
                    &pattern.lines().map(|row| row.chars().nth(matching_col).unwrap()).collect::<String>(),
                    &mut inner_smudge_allowed)
        }) && !inner_smudge_allowed
    })
}

fn patterns_match(left: &str, right: &str, smudge_allowed: &mut bool) -> bool {
    let errors = if *smudge_allowed { 1 } else { 0 };
    let err_count = left.chars().zip(right.chars()).filter(|(l, r)| l != r).count();

    if err_count > errors {
        false
    } else {
        if err_count == 1 { *smudge_allowed = false }
        true
    }
}