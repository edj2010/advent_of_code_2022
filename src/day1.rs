use advent_of_code::parse::{parsers, Parser};

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
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
pub fn part2(input: &str) -> u32 {
    let mut calories: Vec<u32> = parsers::number()
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
