use advent_of_code::{
    grid::GridPoint,
    parse::{parsers, Parser},
};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    ops::AddAssign,
};

macro_rules! parse {
    ($input: ident) => {
        parsers::tag("Sensor at x=")
            .ignore(
                parsers::signed_number()
                    .pair(", y=", parsers::signed_number())
                    .map(|(x, y)| GridPoint::new(y as i64, x as i64)),
            )
            .skip_tag(": closest beacon is at x=")
            .and_then(
                parsers::signed_number()
                    .pair(", y=", parsers::signed_number())
                    .map(|(x, y)| GridPoint::new(y as i64, x as i64)),
            )
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .collect::<HashMap<GridPoint<i64>, GridPoint<i64>>>()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    part1_inner(input, 2_000_000)
}

fn part1_inner(input: &str, row: i64) -> usize {
    let sensors = parse!(input);
    let mut row_data: BTreeMap<i64, i64> = BTreeMap::new();
    sensors.iter().for_each(|(sensor, beacon)| {
        let dist = beacon.sub::<i64>(*sensor).unwrap().l1_norm();
        let y_dist = (row - sensor.row()).abs();
        if y_dist > dist {
            return;
        }
        let left_edge = sensor.col() - (dist - y_dist);
        let right_edge = sensor.col() + (dist - y_dist) + 1;
        row_data.entry(left_edge).or_insert(0).add_assign(1);
        row_data.entry(right_edge).or_insert(0).add_assign(-1);
    });
    row_data
        .iter()
        .fold((i64::MIN, 0, 0), |(last_col, acc, sum), (col, adj)| {
            (
                *col,
                acc + adj,
                sum + (if acc > 0 { col - last_col } else { 0 }),
            )
        })
        .2 as usize
        - sensors
            .iter()
            .filter_map(|(_, beacon)| {
                if *beacon.row() == row {
                    Some(*beacon.col())
                } else {
                    None
                }
            })
            .collect::<BTreeSet<i64>>()
            .len()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    part2_inner(input, 0, 4_000_000)
}

fn part2_inner(input: &str, min: i64, max: i64) -> i64 {
    let sensors = parse!(input);
    let mut all_row_data: BTreeMap<i64, BTreeMap<i64, i64>> = BTreeMap::new();
    sensors.iter().for_each(|(sensor, beacon)| {
        let dist = beacon.sub::<i64>(*sensor).unwrap().l1_norm();
        ((sensor.row() - dist)..=(sensor.row() + dist)).for_each(|row| {
            let y_dist = (row - sensor.row()).abs();
            if y_dist > dist {
                return;
            }
            let left_edge = sensor.col() - (dist - y_dist);
            let right_edge = sensor.col() + (dist - y_dist) + 1;
            let row_data = all_row_data.entry(row).or_insert_with(BTreeMap::new);
            row_data.entry(left_edge).or_insert(0).add_assign(1);
            row_data.entry(right_edge).or_insert(0).add_assign(-1);
        })
    });
    all_row_data
        .range(min..=max)
        .find_map(|(row, row_data)| {
            row_data
                .into_iter()
                .fold((0, None), |(mut acc, found), (col, adj)| {
                    acc += adj;
                    if acc == 0 && *col >= min && *col <= max {
                        (acc, Some(col * 4_000_000 + row))
                    } else {
                        (acc, found)
                    }
                })
                .1
        })
        .unwrap()
}

#[test]
fn part1_test() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
    assert_eq!(part1_inner(input, 10), 26);
}

#[test]
fn part2_test() {
    let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
    assert_eq!(part2_inner(input, 0, 20), 56000011);
}
