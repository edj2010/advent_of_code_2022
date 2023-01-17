#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]
#![feature(test)]

use advent_of_code::web_api::AdventOfCode;
use std::error::Error;
use std::fs;
use std::path::Path;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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
    let input = client.query_question_input(25)?;

    println!("{}", day25::part1(&input));
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
day13, day13_test, 13, 5825, 24477;
day14, day14_test, 14, 618, 26358;
day15, day15_test, 15, 5083287, 13134039205729;
day16, day16_test, 16, 1617, 2171;
day17, day17_test, 17, 3130, 1556521739139;
day18, day18_test, 18, 3498, 2008;
day19, day19_test, 19, 1981, 10962;
day20, day20_test, 20, 2215, 8927480683;
day21, day21_test, 21, 194501589693264, 3887609741189;
day22, day22_test, 22, 126350, 129339;
day23, day23_test, 23, 3815, 893;
day24, day24_test, 24, 225, 711;
day25, day25_test, 25, "", 0;);
