#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use advent_of_code::web_api::AdventOfCode;
use std::error::Error;
use std::fs;
use std::path::Path;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() -> Result<(), Box<dyn Error>> {
    let client = AdventOfCode::init(
        "2022",
        &fs::read_to_string("../session.cookie").expect("failed to read session id"),
        Path::new("inputs"),
    )?;
    let input = client.query_question_input(13)?;

    println!("{}", day13::part2(&input));
    Ok(())
}

macro_rules! tests {
    (@test_module) => {};

    (@test_module $day: ident, $day_test: ident, $question: expr, $part1: expr, $part2: expr) => {
        tests!(@test_module $day, $day_test, $question, $part1, $part2;);
    };

    (@test_module $day: ident, $day_test: ident, $question: expr, $part1: expr, $part2: expr; $($rest:tt)*) => {
        mod $day_test {
            use super::*;

            #[test]
            fn part1_test() {
                let client = AdventOfCode::init(
                    "2022",
                    &fs::read_to_string("../session.cookie")
                        .expect("failed to read session id"),
                    Path::new("inputs"),
                )
                .unwrap();
                assert_eq!(
                    $part1,
                    $day::part1(&client.query_question_input($question).unwrap())
                );
            }

            #[test]
            fn part2_test() {
                let client = AdventOfCode::init(
                    "2022",
                    &fs::read_to_string("../session.cookie")
                        .expect("failed to read session id"),
                    Path::new("inputs"),
                )
                .unwrap();
                assert_eq!(
                    $part2,
                    $day::part2(&client.query_question_input($question).unwrap())
                );
            }
        }

        tests!(@test_module $($rest)*);
    };

    ($($list:tt)*) => {
        #[cfg(test)]
        mod test {
            use super::*;

            tests!(@test_module $($list)*);
        }
    };
}

tests!(
day1, day1_test, 1, 67633, 199628;
day2, day2_test, 2, 12276, 9975;
day3, day3_test, 3, 7848, 2616;
day4, day4_test, 4, 534, 841;
day5, day5_test, 5, "FJSRQCFTN", "CJVLJQPHS";
day6, day6_test, 6, 1198, 3120;
day7, day7_test, 7, 1513699, 7991939;
day8, day8_test, 8, 1698, 672280;
day9, day9_test, 9, 6023, 2533;
day10, day10_test, 10, 16880, "
###..#..#..##..####..##....##.###..###..
#..#.#.#..#..#....#.#..#....#.#..#.#..#.
#..#.##...#..#...#..#..#....#.###..#..#.
###..#.#..####..#...####....#.#..#.###..
#.#..#.#..#..#.#....#..#.#..#.#..#.#.#..
#..#.#..#.#..#.####.#..#..##..###..#..#.";
day11, day11_test, 11, 99840, 20683044837;

day12, day12_test, 12, 391, 386;

day13, day13_test, 13, 5825, 24477;);
