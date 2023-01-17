use advent_of_code::{
    grid::{Direction, GridPoint, PLUS_ADJACENT, ZERO},
    parse::{parsers, Parser},
};

use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    direction: Direction,
    location: GridPoint<isize>,
}

impl Blizzard {
    fn step(mut self, min_row: isize, max_row: isize, min_col: isize, max_col: isize) -> Self {
        let next_location = self.location.add_checked(
            self.direction.into(),
            &min_row,
            &max_row,
            &min_col,
            &max_col,
        );
        match next_location {
            Some(location) => self.location = location,
            None => match self.direction {
                Direction::East => self.location.col = min_col,
                Direction::North => self.location.row = max_row - 1,
                Direction::West => self.location.col = max_col - 1,
                Direction::South => self.location.row = min_row,
            },
        };
        self
    }
}

#[derive(Debug)]
struct BlizzardStateCache {
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
    blizzards: Vec<Blizzard>,
    location_states: Vec<HashSet<GridPoint<isize>>>,
}

impl BlizzardStateCache {
    fn new(
        blizzards: Vec<Blizzard>,
        min_row: isize,
        max_row: isize,
        min_col: isize,
        max_col: isize,
    ) -> Self {
        BlizzardStateCache {
            min_row,
            max_row,
            min_col,
            max_col,
            blizzards,
            location_states: Vec::new(),
        }
    }

    fn iter(&mut self) {
        self.location_states.push(
            self.blizzards
                .iter()
                .map(|b| b.location)
                .collect::<HashSet<GridPoint<isize>>>(),
        );
        self.blizzards
            .iter_mut()
            .for_each(|b| *b = b.step(self.min_row, self.max_row, self.min_col, self.max_col));
    }

    fn occupied(&mut self, round: usize, point: GridPoint<isize>) -> bool {
        while self.location_states.len() <= round {
            self.iter();
        }
        self.location_states[round].contains(&point)
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
                    grid.map(|(row_idx, (_, _, l))| {
                        (0_isize, row_idx as isize + 1, 0_isize, l as isize)
                    })
                    .last()
                    .unwrap(),
                )
            })
            .parse($input)
            .finish()
            .unwrap()
    };
}

fn find_exit(
    blizzards: &mut BlizzardStateCache,
    walls: &HashSet<GridPoint<isize>>,
    initial_round: usize,
    start: GridPoint<isize>,
    end: GridPoint<isize>,
    min_row: isize,
    max_row: isize,
    min_col: isize,
    max_col: isize,
) -> usize {
    let mut to_search: VecDeque<(usize, GridPoint<isize>)> = VecDeque::new();
    let mut seen: HashSet<(usize, GridPoint<isize>)> = HashSet::new();
    to_search.push_back((initial_round, start));
    while let Some((round, point)) = to_search.pop_front() {
        let next_round = round + 1;
        let adjacent: Vec<(usize, GridPoint<isize>)> = PLUS_ADJACENT
            .iter()
            .chain(vec![ZERO].iter())
            .filter_map(|step| {
                let next = point.add_checked(*step, &min_row, &max_row, &min_col, &max_col)?;
                if seen.contains(&(next_round, next))
                    || walls.contains(&next)
                    || blizzards.occupied(next_round, next)
                {
                    None
                } else {
                    Some((next_round, next))
                }
            })
            .collect();
        for next in adjacent {
            if next.1 == end {
                return next.0;
            }
            seen.insert(next);
            to_search.push_back(next);
        }
    }
    panic!("failed to find exit");
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let (blizards, walls, (min_row, max_row, min_col, max_col)) = parse!(input);
    let mut blizzard_locations: BlizzardStateCache =
        BlizzardStateCache::new(blizards, min_row + 1, max_row - 1, min_col + 1, max_col - 1);
    find_exit(
        &mut blizzard_locations,
        &walls,
        0,
        GridPoint::new(0, 1),
        GridPoint::new(max_row - 1, max_col - 2),
        min_row,
        max_row,
        min_col,
        max_col,
    )
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let (blizards, walls, (min_row, max_row, min_col, max_col)) = parse!(input);
    let mut blizzard_locations: BlizzardStateCache =
        BlizzardStateCache::new(blizards, min_row + 1, max_row - 1, min_col + 1, max_col - 1);
    let start = GridPoint::new(0, 1);
    let end = GridPoint::new(max_row - 1, max_col - 2);
    let first = find_exit(
        &mut blizzard_locations,
        &walls,
        0,
        start,
        end,
        min_row,
        max_row,
        min_col,
        max_col,
    );
    let second = find_exit(
        &mut blizzard_locations,
        &walls,
        first,
        end,
        start,
        min_row,
        max_row,
        min_col,
        max_col,
    );
    find_exit(
        &mut blizzard_locations,
        &walls,
        second,
        start,
        end,
        min_row,
        max_row,
        min_col,
        max_col,
    )
}

#[test]
fn part1_test() {
    let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
    assert_eq!(part1(input), 18);
}

#[test]
fn part2_test() {
    let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";
    assert_eq!(part2(input), 54);
}
