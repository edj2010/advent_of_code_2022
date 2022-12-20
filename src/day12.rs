use std::collections::VecDeque;

use advent_of_code::{
    grid::{Grid, GridPoint, PLUS_ADJACENT},
    parse::{parsers, Parser},
};

macro_rules! parser {
    ($input: ident) => {{
        let vec_of_vecs = parsers::chars(|c| c.is_alphabetic())
            .map(|c| u8::try_from(c).unwrap())
            .many()
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .map(|v| v.collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        Grid::of_vec_of_vecs(vec_of_vecs).unwrap()
    }};
}

fn shortest_distance<F: Fn(GridPoint<usize, isize>, GridPoint<usize, isize>) -> bool>(
    map: Grid<u8>,
    connected: F,
    start_idx: GridPoint<usize, isize>,
    end: u8,
) -> usize {
    let mut distance: Grid<Option<usize>> = Grid::init(None, map.rows(), map.cols());
    distance.set(start_idx, Some(0)).unwrap();
    let mut to_search: VecDeque<GridPoint<usize, isize>> = VecDeque::from([start_idx]);
    while let Some(next) = to_search.pop_front() {
        for adjacent in PLUS_ADJACENT
            .into_iter()
            .filter_map(|delta| next.add_checked(delta, &0, &map.rows(), &0, &map.cols()))
        {
            if !connected(next, adjacent) || distance.get(adjacent).unwrap().is_some() {
                continue;
            }
            distance
                .set(adjacent, Some(distance.get(next).unwrap().unwrap() + 1))
                .unwrap();
            if map[adjacent] == end {
                return distance.get(adjacent).unwrap().unwrap();
            }
            to_search.push_back(adjacent);
        }
    }
    0
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let map: Grid<u8> = parser!(input);
    let height_map = Grid::from(
        map.clone().into_iter().map(|c| {
            if c == b'S' {
                b'a'
            } else if c == b'E' {
                b'z'
            } else {
                c
            }
        }),
        map.rows(),
        map.cols(),
    )
    .unwrap();
    let start_idx = map.find(&b'S').unwrap();
    shortest_distance(
        map,
        |next, adjacent| height_map[next] + 1 >= height_map[adjacent],
        start_idx,
        b'E',
    )
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let map: Grid<u8> = parser!(input);
    let height_map = Grid::from(
        map.clone().into_iter().map(|c| {
            if c == b'S' {
                b'a'
            } else if c == b'E' {
                b'z'
            } else {
                c
            }
        }),
        map.rows(),
        map.cols(),
    )
    .unwrap();
    let start_idx = map.find(&b'E').unwrap();
    shortest_distance(
        map,
        |next, adjacent| height_map[next] <= height_map[adjacent] + 1,
        start_idx,
        b'a',
    )
}

#[test]
fn part1_test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    assert_eq!(31, part1(input));
}

#[test]
fn part2_test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    assert_eq!(29, part2(input));
}
