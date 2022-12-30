use advent_of_code::{
    grid::{Block, GridPoint, GridPointDelta, Lattice, EAST, SOUTH, WEST},
    parse::{parsers, Parser},
};

use std::{cmp::max, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Left,
    Right,
}

macro_rules! parse {
    ($input: ident) => {
        parsers::char('>')
            .map(|_| Move::Right)
            .or(parsers::char('<').map(|_| Move::Left))
            .many()
            .line("\n")
            .parse($input)
            .finish()
            .unwrap()
            .collect::<Vec<Move>>()
    };
}

fn slice_hash(
    lattice: &Lattice<char>,
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
) -> u64 {
    let mut hash: u64 = 0;
    for row in min_row..max_row {
        for col in min_col..max_col {
            hash *= 2;
            if lattice.contains(GridPoint::new(row, col)) {
                hash += 1;
            }
        }
    }
    hash
}

#[allow(dead_code)]
pub fn part1(input: &str) -> isize {
    let mut blocks = vec![
        Block::from(vec![
            GridPoint::new(0, 0),
            GridPoint::new(0, 1),
            GridPoint::new(0, 2),
            GridPoint::new(0, 3),
        ]),
        Block::from(vec![
            GridPoint::new(-1, 0),
            GridPoint::new(0, 1),
            GridPoint::new(-1, 1),
            GridPoint::new(-1, 2),
            GridPoint::new(-2, 1),
        ]),
        Block::from(vec![
            GridPoint::new(0, 0),
            GridPoint::new(0, 1),
            GridPoint::new(0, 2),
            GridPoint::new(-1, 2),
            GridPoint::new(-2, 2),
        ]),
        Block::from(vec![
            GridPoint::new(0, 0),
            GridPoint::new(-1, 0),
            GridPoint::new(-2, 0),
            GridPoint::new(-3, 0),
        ]),
        Block::from(vec![
            GridPoint::new(0, 0),
            GridPoint::new(-1, 0),
            GridPoint::new(0, 1),
            GridPoint::new(-1, 1),
        ]),
    ]
    .into_iter()
    .cycle();
    let mut moves = parse!(input).into_iter().cycle();
    let mut game: Lattice<char> = Lattice::empty();
    let mut height = 0;
    for _ in 0..2022 {
        let mut rock = (blocks.next().unwrap() + GridPointDelta::new(-(height + 3), 2)).unwrap();
        loop {
            let moved_rock = match moves.next() {
                None => panic!("no more moves"),
                Some(Move::Left) => {
                    if *rock.min_col().unwrap() > 0 {
                        (rock.clone() + WEST).unwrap()
                    } else {
                        rock.clone()
                    }
                }
                Some(Move::Right) => {
                    if *rock.max_col().unwrap() < 6 {
                        (rock.clone() + EAST).unwrap()
                    } else {
                        rock.clone()
                    }
                }
            };
            if !game.intersects_block(&moved_rock) {
                rock = moved_rock;
            }
            let lowered_rock = (rock.clone() + SOUTH).unwrap();
            if game.intersects_block(&lowered_rock) || *lowered_rock.max_row().unwrap() > 0 {
                height = max(height, 1 - *rock.min_row().unwrap());
                game.set_block(rock, '#');
                break;
            } else {
                rock = lowered_rock;
            }
        }
    }
    height
}

#[allow(dead_code)]
pub fn part2(input: &str) -> isize {
    let rocks = vec![
        Block::from(vec![
            GridPoint::new(0, 0),
            GridPoint::new(0, 1),
            GridPoint::new(0, 2),
            GridPoint::new(0, 3),
        ]),
        Block::from(vec![
            GridPoint::new(-1, 0),
            GridPoint::new(0, 1),
            GridPoint::new(-1, 1),
            GridPoint::new(-1, 2),
            GridPoint::new(-2, 1),
        ]),
        Block::from(vec![
            GridPoint::new(0, 0),
            GridPoint::new(0, 1),
            GridPoint::new(0, 2),
            GridPoint::new(-1, 2),
            GridPoint::new(-2, 2),
        ]),
        Block::from(vec![
            GridPoint::new(0, 0),
            GridPoint::new(-1, 0),
            GridPoint::new(-2, 0),
            GridPoint::new(-3, 0),
        ]),
        Block::from(vec![
            GridPoint::new(0, 0),
            GridPoint::new(-1, 0),
            GridPoint::new(0, 1),
            GridPoint::new(-1, 1),
        ]),
    ];
    let mut moves = parse!(input).into_iter().enumerate().cycle();
    let mut seen: HashMap<(usize, u64), (isize, isize)> = HashMap::new();
    let mut round_idx = 0;
    let mut game: Lattice<char> = Lattice::empty();
    let mut height = 0;
    loop {
        let mut last_move_idx = 0;
        for rock_idx in 0..5 {
            round_idx += 1;
            let mut rock =
                (rocks[rock_idx].clone() + GridPointDelta::new(-(height + 3), 2)).unwrap();
            loop {
                let moved_rock = match moves.next() {
                    None => panic!("no more moves"),
                    Some((move_idx, Move::Left)) => {
                        last_move_idx = move_idx;
                        if *rock.min_col().unwrap() > 0 {
                            (rock.clone() + WEST).unwrap()
                        } else {
                            rock.clone()
                        }
                    }
                    Some((move_idx, Move::Right)) => {
                        last_move_idx = move_idx;
                        if *rock.max_col().unwrap() < 6 {
                            (rock.clone() + EAST).unwrap()
                        } else {
                            rock.clone()
                        }
                    }
                };
                if !game.intersects_block(&moved_rock) {
                    rock = moved_rock;
                }
                let lowered_rock = (rock.clone() + SOUTH).unwrap();
                if game.intersects_block(&lowered_rock) || *lowered_rock.max_row().unwrap() > 0 {
                    height = max(height, 1 - *rock.min_row().unwrap());
                    game.set_block(rock, '#');
                    break;
                } else {
                    rock = lowered_rock;
                }
            }
        }
        let key = (
            last_move_idx,
            slice_hash(&game, 1 - height, 8 - height, 0, 7),
        );
        let value = (round_idx, height);
        if let Some((last_round_idx, last_height)) = seen.get(&key) {
            if (1_000_000_000_000 - last_round_idx) % (round_idx - last_round_idx) != 0 {
                continue;
            }
            return (1_000_000_000_000 - last_round_idx) / (round_idx - last_round_idx)
                * (height - last_height)
                + last_height;
        } else {
            seen.insert(key, value);
        }
    }
}

#[test]
fn part1_test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";
    assert_eq!(part1(input), 3068);
}

#[test]
fn part2_test() {
    let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
";
    assert_eq!(part2(input), 1_514_285_714_288);
}
