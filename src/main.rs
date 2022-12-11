use advent_of_code::web_api::AdventOfCode;
use std::error::Error;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() -> Result<(), Box<dyn Error>> {
    let client = AdventOfCode::init(
        "2022",
        &fs::read_to_string("../session.cookie").expect("failed to read session id"),
    )?;
    let input = client.query_question_input(8)?;

    println!("{}", day8::part2(&input));
    Ok(())
}
