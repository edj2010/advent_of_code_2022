use advent_of_code::{
    grid::{
        GridPoint, GridPointDelta, ADJACENT, EAST, NORTH, NORTHEAST, NORTHWEST, SOUTH, SOUTHEAST,
        SOUTHWEST, WEST, ZERO,
    },
    parse::{parsers, Parser},
};
use std::collections::{HashMap, HashSet};

macro_rules! parse {
    ($input: ident) => {
        parsers::char('.')
            .or(parsers::char('#'))
            .many()
            .map(|r| r.enumerate().filter(|(_, c)| *c == '#'))
            .many_lines("\n")
            .map(|g| {
                g.enumerate()
                    .flat_map(|(row_idx, r)| {
                        r.map(move |(col_idx, _)| {
                            GridPoint::new(row_idx as isize, col_idx as isize)
                        })
                    })
                    .collect::<HashSet<GridPoint<isize>>>()
            })
            .parse($input)
            .finish()
            .unwrap()
    };
}

fn desired_spot(
    elves: &HashSet<GridPoint<isize>>,
    search_pattern: &Vec<(Vec<GridPointDelta<isize>>, GridPointDelta<isize>)>,
    elf: GridPoint<isize>,
) -> GridPoint<isize> {
    search_pattern
        .iter()
        .find_map(|(to_check, delta)| {
            if to_check
                .iter()
                .all(|next_to_check| !elves.contains(&(elf + *next_to_check).unwrap()))
            {
                Some((elf + *delta).unwrap())
            } else {
                None
            }
        })
        .unwrap_or(elf)
}

fn round(
    elves: &HashSet<GridPoint<isize>>,
    search_pattern: &Vec<(Vec<GridPointDelta<isize>>, GridPointDelta<isize>)>,
) -> (HashSet<GridPoint<isize>>, bool) {
    let mut next_elves: HashMap<GridPoint<isize>, GridPoint<isize>> = HashMap::new();
    let mut finished = true;
    for elf in elves.iter() {
        let desired_move = desired_spot(&elves, &search_pattern, *elf);
        finished &= desired_move == *elf;
        if next_elves.contains_key(&desired_move) {
            let old = next_elves.remove(&desired_move).unwrap();
            next_elves.insert(old, old);
            next_elves.insert(*elf, *elf);
        } else {
            next_elves.insert(desired_move, *elf);
        }
    }
    (next_elves.keys().copied().collect(), finished)
}

fn score(elves: &HashSet<GridPoint<isize>>) -> isize {
    let min_row = elves.iter().map(|p| p.row).min().unwrap();
    let max_row = elves.iter().map(|p| p.row).max().unwrap() + 1;
    let min_col = elves.iter().map(|p| p.col).min().unwrap();
    let max_col = elves.iter().map(|p| p.col).max().unwrap() + 1;
    (max_col - min_col) * (max_row - min_row) - elves.len() as isize
}

#[allow(dead_code)]
pub fn part1(input: &str) -> isize {
    let mut elves: HashSet<GridPoint<isize>> = parse!(input);
    let mut search_pattern = vec![
        (Vec::from(ADJACENT), ZERO),
        (vec![NORTHWEST, NORTH, NORTHEAST], NORTH),
        (vec![SOUTHWEST, SOUTH, SOUTHEAST], SOUTH),
        (vec![NORTHWEST, WEST, SOUTHWEST], WEST),
        (vec![NORTHEAST, EAST, SOUTHEAST], EAST),
    ];
    for _ in 0..10 {
        score(&elves);
        (elves, _) = round(&elves, &search_pattern);
        let front = search_pattern.remove(1);
        search_pattern.push(front);
    }
    score(&elves)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let mut elves: HashSet<GridPoint<isize>> = parse!(input);
    let mut search_pattern = vec![
        (Vec::from(ADJACENT), ZERO),
        (vec![NORTHWEST, NORTH, NORTHEAST], NORTH),
        (vec![SOUTHWEST, SOUTH, SOUTHEAST], SOUTH),
        (vec![NORTHWEST, WEST, SOUTHWEST], WEST),
        (vec![NORTHEAST, EAST, SOUTHEAST], EAST),
    ];
    let mut finished = false;
    let mut idx = 0;
    while !finished {
        idx += 1;
        score(&elves);
        (elves, finished) = round(&elves, &search_pattern);
        let front = search_pattern.remove(1);
        search_pattern.push(front);
    }
    idx
}

#[test]
fn part1_test() {
    let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
    assert_eq!(part1(input), 110);
}

#[test]
fn part2_test() {
    let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";
    assert_eq!(part2(input), 20);
}
