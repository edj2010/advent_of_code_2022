use advent_of_code::parse::{parsers, Parser};

macro_rules! parse {
    ($input: ident) => {
        parsers::number()
            .pair("-", parsers::number())
            .pair(",", parsers::number().pair("-", parsers::number()))
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    parse!(input)
        .map(|((al, ar), (bl, br))| {
            if (al <= bl && ar >= br) || (al >= bl && ar <= br) {
                1
            } else {
                0
            }
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    parse!(input)
        .map(|((al, ar), (bl, br))| {
            if al <= br && ar >= bl || al >= br && ar <= bl {
                1
            } else {
                0
            }
        })
        .sum()
}
