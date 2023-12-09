use input_curler::input_for;

fn main() {
    let data = input_for(9).unwrap();

    let sequences = data.lines().map(|line| line.split_whitespace().map(|n| n.parse::<i32>().unwrap()));

    let answer_one = part_one(sequences.clone());
    println!("Part one: {}", answer_one);
    let answer_two = part_two(sequences.clone());
    println!("Part two: {}", answer_two);
}

fn part_one<Iter, InnerIter>(sequences: Iter) -> i32
where Iter: Iterator<Item = InnerIter>,
    InnerIter: Iterator<Item =i32> + Clone
{
    sequences.map(|seq| next_value(seq)).sum()
}

fn next_value<Iter>(sequence: Iter) -> i32
where Iter: Iterator<Item = i32> + Clone
{
    let mut lasts = vec![sequence.clone().last().unwrap()];

    let mut working_seq = sequence.collect::<Vec<i32>>();
    loop {
        let diffs = working_seq.windows(2).map(|pair| pair[1] - pair[0]);
        if diffs.clone().all(|n| n == 0) {
            break;
        }
        working_seq = diffs.collect();
        lasts.push(*working_seq.last().unwrap());
    }

    lasts.iter().sum()
}

fn part_two<Iter, InnerIter>(sequences: Iter) -> i32
where Iter: Iterator<Item = InnerIter>,
    InnerIter: Iterator<Item =i32> + Clone
{
    sequences.map(|seq| prev_value(seq)).sum()
}

fn prev_value<Iter>(sequence: Iter) -> i32
where Iter: Iterator<Item = i32> + Clone
{
    let mut firsts = vec![sequence.clone().next().unwrap()];

    let mut working_seq = sequence.collect::<Vec<i32>>();
    loop {
        let diffs = working_seq.windows(2).map(|pair| pair[1] - pair[0]);
        if diffs.clone().all(|n| n == 0) {
            break;
        }
        working_seq = diffs.collect();
        firsts.push(*working_seq.first().unwrap());
    }

    firsts.iter().enumerate().map(|(ix, n)|
        if ix % 2 == 0 { *n } else { -n }
    ).sum()
}