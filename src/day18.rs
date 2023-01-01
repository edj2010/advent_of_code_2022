use advent_of_code::{
    parse::{parsers, Parser},
    vector::Vector,
};

use std::collections::{HashSet, VecDeque};

macro_rules! parse {
    ($input: ident) => {
        parsers::signed_number()
            .skip_tag(",")
            .and_then(parsers::signed_number())
            .skip_tag(",")
            .and_then(parsers::signed_number())
            .map(|((a, b), c)| Vector::of_raw([a, b, c]))
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .collect::<HashSet<Vector<3, i32>>>()
    };
}

fn is_external(
    external: &mut HashSet<Vector<3, i32>>,
    internal: &HashSet<Vector<3, i32>>,
    point: Vector<3, i32>,
) -> bool {
    let adjacent: Vec<Vector<3, i32>> = vec![
        Vector::of_raw([1, 0, 0]),
        Vector::of_raw([-1, 0, 0]),
        Vector::of_raw([0, 1, 0]),
        Vector::of_raw([0, -1, 0]),
        Vector::of_raw([0, 0, 1]),
        Vector::of_raw([0, 0, -1]),
    ];
    let mut to_search = VecDeque::from([point]);
    let mut seen: HashSet<Vector<3, i32>> = HashSet::new();
    while let Some(next) = to_search.pop_front() {
        if internal.contains(&next) || seen.contains(&next) {
            continue;
        }
        if external.contains(&next) {
            external.extend(seen);
            return true;
        }
        seen.insert(next);
        for delta in adjacent.iter() {
            let adj = next + *delta;
            to_search.push_back(adj);
        }
    }
    false
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let adjacent: Vec<Vector<3, i32>> = vec![
        Vector::of_raw([1, 0, 0]),
        Vector::of_raw([-1, 0, 0]),
        Vector::of_raw([0, 1, 0]),
        Vector::of_raw([0, -1, 0]),
        Vector::of_raw([0, 0, 1]),
        Vector::of_raw([0, 0, -1]),
    ];
    let pixels = parse!(input);
    pixels
        .iter()
        .map(|pixel| {
            adjacent
                .iter()
                .filter(|adj| !pixels.contains(&(*pixel + **adj)))
                .count()
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let adjacent: Vec<Vector<3, i32>> = vec![
        Vector::of_raw([1, 0, 0]),
        Vector::of_raw([-1, 0, 0]),
        Vector::of_raw([0, 1, 0]),
        Vector::of_raw([0, -1, 0]),
        Vector::of_raw([0, 0, 1]),
        Vector::of_raw([0, 0, -1]),
    ];
    let pixels = parse!(input);
    let mut external: HashSet<Vector<3, i32>> = HashSet::from([Vector::of_raw([0, 0, 0])]);
    pixels
        .iter()
        .map(|pixel| {
            adjacent
                .iter()
                .filter(|adj| is_external(&mut external, &pixels, *pixel + **adj))
                .count()
        })
        .sum()
}

#[test]
fn part1_test() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
    assert_eq!(part1(input), 64);
}

#[test]
fn part2_test() {
    let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";
    assert_eq!(part2(input), 58);
}
