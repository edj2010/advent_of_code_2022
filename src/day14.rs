use advent_of_code::{
    grid::{GridPoint, GridPointDelta, Lattice, SOUTH, SOUTHEAST, SOUTHWEST},
    parse::{parsers, Parser},
};
use std::cmp::max;
const DOWN: [GridPointDelta<isize>; 3] = [SOUTH, SOUTHWEST, SOUTHEAST];

macro_rules! parse {
    ($input: ident) => {
        parsers::signed_number()
            .pair(",", parsers::signed_number())
            .map(|(a, b)| GridPoint::new(a, b))
            .list(" -> ")
            .map(|v| v.collect::<Vec<GridPoint<isize>>>())
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut grid: Lattice<char> = Lattice::empty();
    let mut max_y: isize = 0;
    parse!(input).for_each(|line| {
        for idx in 1..line.len() {
            let start = line[idx - 1];
            let finish = line[idx];
            start.traverse_to(finish).unwrap().for_each(|idx| {
                max_y = max(*idx.col(), max_y);
                grid.set(idx, '#');
            })
        }
    });
    (0..)
        .find(|_| {
            let mut sand = GridPoint::new(500, 0);
            while let Some(next_sand) = DOWN.into_iter().find_map(|delta| {
                let next_sand = (sand + delta)?;
                if grid.get(next_sand).is_none() && *next_sand.col() <= max_y {
                    Some(next_sand)
                } else {
                    None
                }
            }) {
                sand = next_sand;
            }
            grid.set(sand, 'o');
            *sand.col() >= max_y
        })
        .unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut grid: Lattice<char> = Lattice::empty();
    let mut max_y: isize = 0;
    parse!(input).for_each(|line| {
        for idx in 1..line.len() {
            let start = line[idx - 1];
            let finish = line[idx];
            start.traverse_to(finish).unwrap().for_each(|idx| {
                max_y = max(*idx.col(), max_y);
                grid.set(idx, '#');
            })
        }
    });
    (1..)
        .find(|_| {
            let mut sand = GridPoint::new(500, 0);
            while let Some(next_sand) = DOWN.into_iter().find_map(|delta| {
                let next_sand = (sand + delta)?;
                if grid.get(next_sand).is_none() && *next_sand.col() < max_y + 2 {
                    Some(next_sand)
                } else {
                    None
                }
            }) {
                sand = next_sand;
            }
            grid.set(sand, 'o');
            if *sand.col() == 0 {
                return true;
            } else {
                return false;
            }
        })
        .unwrap()
}

#[test]
fn part1_test() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
    assert_eq!(part1(input), 24);
}

#[test]
fn part2_test() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";
    assert_eq!(part2(input), 93);
}
