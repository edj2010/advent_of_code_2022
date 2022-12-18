use advent_of_code::parse::{parsers, Parser};

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    parsers::number()
        .many_lines("\n")
        .list("\n")
        .parse(input)
        .finish()
        .unwrap()
        .map(|l| l.sum())
        .max()
        .unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut calories: Vec<usize> = parsers::number()
        .many_lines("\n")
        .list("\n")
        .parse(input)
        .finish()
        .unwrap()
        .map(|l| l.sum())
        .collect();
    calories.sort();
    calories.pop().unwrap() + calories.pop().unwrap() + calories.pop().unwrap()
}
