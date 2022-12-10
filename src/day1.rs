use advent_of_code::simple_parse::parse_chunks;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    parse_chunks(input, |s| s.parse::<usize>().ok())
        .map(|l| l.sum())
        .max()
        .unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut calories: Vec<usize> = parse_chunks(input, |s| s.parse::<usize>().ok())
        .map(|l| l.sum())
        .collect();
    calories.sort();
    calories.pop().unwrap() + calories.pop().unwrap() + calories.pop().unwrap()
}
