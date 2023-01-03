use advent_of_code::{
    grid::{Direction, GridPoint},
    parse::{parsers, Parser},
};

use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    direction: Direction,
    location: GridPoint<isize>,
}

impl Blizzard {
    fn step(self, min_row: isize, max_row: isize, min_col: isize, max_col: isize) -> Self {
        self.location = (self.location + self.direction.into()).unwrap();
        self
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Wall,
    Empty,
    Blizard(Direction),
}

macro_rules! parse {
    ($input: ident) => {
        parsers::char('.')
            .map(|_| Cell::Empty)
            .or(parsers::char('#').map(|_| Cell::Wall))
            .or(parsers::char('>').map(|_| Cell::Blizard(Direction::East)))
            .or(parsers::char('v').map(|_| Cell::Blizard(Direction::South)))
            .or(parsers::char('<').map(|_| Cell::Blizard(Direction::West)))
            .or(parsers::char('^').map(|_| Cell::Blizard(Direction::North)))
            .many()
            .map(|r| {
                (
                    r.clone().enumerate().filter_map(|(idx, cell)| match cell {
                        Cell::Blizard(d) => Some((idx, d)),
                        _ => None,
                    }),
                    r.clone().enumerate().filter_map(|(idx, cell)| match cell {
                        Cell::Wall => Some(idx),
                        _ => None,
                    }),
                    r.count(),
                )
            })
            .many_lines("\n")
            .map(|g| {
                let grid = g.enumerate();
                (
                    grid.clone()
                        .flat_map(|(row_idx, (r, _, _))| {
                            r.map(move |(col_idx, direction)| Blizzard {
                                location: GridPoint::new(row_idx as isize, col_idx as isize),
                                direction,
                            })
                        })
                        .collect::<Vec<Blizzard>>(),
                    grid.clone()
                        .flat_map(|(row_idx, (_, w, _))| {
                            w.map(move |col_idx| GridPoint::new(row_idx as isize, col_idx as isize))
                        })
                        .collect::<HashSet<GridPoint<isize>>>(),
                    grid.map(|(row_idx, (_, _, l))| (row_idx + 1, l))
                        .last()
                        .unwrap(),
                )
            })
            .parse($input)
            .finish()
            .unwrap()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let (blizards, walls, (rows, cols)) = parse!(input);
    0
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    todo!()
}

#[test]
fn part1_test() {
    let input = "";
    assert_eq!(part1(input), 0);
}

#[test]
fn part2_test() {
    let input = "";
    assert_eq!(part2(input), 0);
}
