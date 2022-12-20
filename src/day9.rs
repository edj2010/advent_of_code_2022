use std::collections::HashSet;

use advent_of_code::grid::{GridPoint, GridPointDelta, ADJACENT, EAST, NORTH, SOUTH, WEST, ZERO};
use advent_of_code::parse::{parsers, Parser};

macro_rules! parse {
    ($input: ident) => {
        parsers::char_any()
            .map(|c| match c {
                'U' => NORTH,
                'R' => EAST,
                'D' => SOUTH,
                'L' => WEST,
                _ => ZERO,
            })
            .pair(" ", parsers::number())
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
    };
}

fn normalized_delta(d: GridPointDelta<isize>) -> GridPointDelta<isize> {
    let row_delta = if d.row_delta == 0 {
        0
    } else {
        d.row_delta / d.row_delta.abs()
    };
    let col_delta = if d.col_delta == 0 {
        0
    } else {
        d.col_delta / d.col_delta.abs()
    };
    GridPointDelta::new(row_delta, col_delta)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut head: GridPoint<isize, isize> = GridPoint::new(0, 0);
    let mut tail: GridPoint<isize, isize> = GridPoint::new(0, 0);
    let mut seen: HashSet<GridPoint<isize, isize>> = HashSet::from([tail]);
    parse!(input).for_each(|(dir, count)| {
        (0..count).for_each(|_| {
            head = (head + dir).unwrap();
            let delta = (head - tail).unwrap();
            if !(delta == ZERO || ADJACENT.contains(&delta)) {
                tail = (tail + normalized_delta(delta)).unwrap();
                seen.insert(tail);
            }
        })
    });
    seen.len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut rope: [GridPoint<isize, isize>; 10] = [GridPoint::new(0, 0); 10];
    let mut seen: HashSet<GridPoint<isize, isize>> = HashSet::from([rope[9]]);
    parse!(input).for_each(|(dir, count)| {
        (0..count).for_each(|_| {
            rope[0] = (rope[0] + dir).unwrap();
            for idx in 1..rope.len() {
                let delta = (rope[idx - 1] - rope[idx]).unwrap();
                if !(delta == ZERO || ADJACENT.contains(&delta)) {
                    rope[idx] = (rope[idx] + normalized_delta(delta)).unwrap();
                }
            }
            seen.insert(rope[9]);
        })
    });
    seen.len()
}

#[test]
fn part1_test() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";
    assert_eq!(part1(input), 13);
}
