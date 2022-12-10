use std::collections::BTreeSet;
use itertools::Itertools;

fn score_char(c: u8) -> usize {
    match c {
        b'a'..=b'z' => (c - b'a') as usize + 1,
        b'A'..=b'Z' => (c - b'A') as usize + 27,
        _ => 0_usize,
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let first_half: BTreeSet<u8> = s[..(s.len() / 2)].bytes().collect();
            let second_half: BTreeSet<u8> = s[(s.len() / 2)..].bytes().collect();
            *(&first_half & &second_half).first().unwrap()
        })
        .map(score_char)
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.bytes().collect::<BTreeSet<u8>>())
        .tuple_windows::<(_, _, _)>()
        .step_by(3)
        .map(|(a, b, c)| *(&(&a & &b) & &c).first().unwrap())
        .map(score_char)
        .sum()
}
