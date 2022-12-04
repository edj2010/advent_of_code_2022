use advent_of_code::simple_parse::{parse_chunks, parse_lines};
use advent_of_code::web_api::AdventOfCode;
use std::error::Error;
use std::fs;

#[allow(dead_code)]
fn day1_part1(input: &str) -> usize {
    parse_chunks(input, |s| s.parse::<usize>().ok())
        .map(|l| l.sum())
        .max()
        .unwrap()
}

#[allow(dead_code)]
fn day1_part2(input: &str) -> usize {
    let mut calories: Vec<usize> = parse_chunks(input, |s| s.parse::<usize>().ok())
        .map(|l| l.sum())
        .collect();
    calories.sort();
    calories.pop().unwrap() + calories.pop().unwrap() + calories.pop().unwrap()
}

#[allow(dead_code)]
fn day2_part1(input: &str) -> usize {
    parse_lines(input, |s| {
        Some(match s {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("Failed to parse {}", s),
        })
    })
    .sum()
}

#[allow(dead_code)]
fn day2_part2(input: &str) -> usize {
    parse_lines(input, |s| {
        Some(match s {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("Failed to parse {}", s),
        })
    })
    .sum()
}

#[allow(dead_code)]
fn day3_part1(input: &str) -> usize {
    use std::collections::BTreeSet;
    input
        .lines()
        .map(|s| {
            let first_half: BTreeSet<u8> = s[..(s.len() / 2)].bytes().collect();
            let second_half: BTreeSet<u8> = s[(s.len() / 2)..].bytes().collect();
            first_half
                .intersection(&second_half)
                .cloned()
                .min()
                .unwrap()
        })
        .map(|c| match c {
            b'a'..=b'z' => (c - b'a') as usize + 1,
            b'A'..=b'Z' => (c - b'A') as usize + 27,
            _ => 0_usize,
        })
        .sum()
}

#[allow(dead_code)]
fn day3_part2(input: &str) -> usize {
    use itertools::Itertools;
    use std::collections::BTreeSet;
    input
        .lines()
        .tuple_windows::<(_, _, _)>()
        .step_by(3)
        .map(|(a, b, c)| {
            a.bytes()
                .collect::<BTreeSet<u8>>()
                .intersection(&b.bytes().collect::<BTreeSet<u8>>())
                .cloned()
                .collect::<BTreeSet<u8>>()
                .intersection(&c.bytes().collect::<BTreeSet<u8>>())
                .cloned()
                .min()
                .unwrap()
        })
        .map(|c| match c {
            b'a'..=b'z' => (c - b'a') as usize + 1,
            b'A'..=b'Z' => (c - b'A') as usize + 27,
            _ => 0_usize,
        })
        .sum()
}

#[allow(dead_code)]
fn day4_part1(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let v = s
                .split(",")
                .map(|s| {
                    s.split("-")
                        .filter_map(|v| v.parse::<usize>().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            if v[0][0] <= v[1][0] && v[0][1] >= v[1][1] || v[0][0] >= v[1][0] && v[0][1] <= v[1][1]
            {
                1
            } else {
                0
            }
        })
        .sum()
}

#[allow(dead_code)]
fn day4_part2(input: &str) -> usize {
    input
        .lines()
        .map(|s| {
            let v = s
                .split(",")
                .map(|s| {
                    s.split("-")
                        .filter_map(|v| v.parse::<usize>().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            if v[0][0] <= v[1][1] && v[0][1] >= v[1][0] || v[0][0] >= v[1][1] && v[0][1] <= v[1][0]
            {
                1
            } else {
                0
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = AdventOfCode::init(
        "2022",
        &fs::read_to_string("session.cookie").expect("failed to read session id"),
    )?;
    let input = client.query_question_input(4)?;

    println!("{}", day4_part2(&input));
    Ok(())
}
