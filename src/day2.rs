use advent_of_code::simple_parse::parse_lines;

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
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
pub fn part2(input: &str) -> usize {
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
