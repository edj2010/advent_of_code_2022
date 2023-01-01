use advent_of_code::parse::{parsers, ParseError, Parser};
use itertools::Itertools;
use std::collections::BTreeSet;

fn score_char(c: u8) -> u32 {
    match c {
        b'a'..=b'z' => (c - b'a') as u32 + 1,
        b'A'..=b'Z' => (c - b'A') as u32 + 27,
        _ => 0_u32,
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    parsers::many_chars(|c| c != '\n')
        .bind(|s: String| {
            let first_half: BTreeSet<u8> = s[..(s.len() / 2)].bytes().collect();
            let second_half: BTreeSet<u8> = s[(s.len() / 2)..].bytes().collect();
            Ok(score_char(
                *(&first_half & &second_half)
                    .first()
                    .ok_or(ParseError::EndOfString)?,
            ))
        })
        .many_lines("\n")
        .parse(input)
        .finish()
        .unwrap()
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    parsers::many_chars(|c| c != '\n')
        .map(|l: String| l.bytes().collect::<BTreeSet<u8>>())
        .many_lines("\n")
        .parse(input)
        .finish()
        .unwrap()
        .tuple_windows::<(_, _, _)>()
        .step_by(3)
        .map(|(a, b, c)| *(&(&a & &b) & &c).first().unwrap())
        .map(score_char)
        .sum()
}
