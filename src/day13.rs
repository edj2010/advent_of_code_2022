use advent_of_code::parse::{parsers, Parser};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Input {
    LeftParen,
    RightParen,
    Number(u32),
}

macro_rules! parse {
    ($input: ident) => {
        parsers::char('[')
            .map(|_| Input::LeftParen)
            .or(parsers::char(']').map(|_| Input::RightParen))
            .or(parsers::number().map(|n| Input::Number(n)))
            .skip(parsers::char(',').maybe())
            .many()
            .line("\n")
            .repeat(2)
            .map(|mut v| {
                (
                    v.next().unwrap().collect::<VecDeque<Input>>(),
                    v.next().unwrap().collect::<VecDeque<Input>>(),
                )
            })
            .list("\n")
            .parse($input)
            .finish()
            .unwrap()
    };
}

fn compare_packets(left: &VecDeque<Input>, right: &VecDeque<Input>) -> Ordering {
    let mut a = left.clone();
    let mut b = right.clone();
    loop {
        match (a.get(0).map(|v| *v), b.get(0).map(|v| *v)) {
            (None, None) => return Ordering::Equal,
            (None, _) => return Ordering::Less,
            (_, None) => return Ordering::Greater,
            (Some(Input::LeftParen), Some(Input::LeftParen)) => {
                a.pop_front();
                b.pop_front();
            }
            (Some(Input::LeftParen), Some(Input::Number(n))) => {
                b.pop_front();
                b.push_front(Input::RightParen);
                b.push_front(Input::Number(n));
                b.push_front(Input::LeftParen);
            }
            (Some(Input::Number(n)), Some(Input::LeftParen)) => {
                a.pop_front();
                a.push_front(Input::RightParen);
                a.push_front(Input::Number(n));
                a.push_front(Input::LeftParen);
            }
            (Some(Input::RightParen), Some(Input::RightParen)) => {
                a.pop_front();
                b.pop_front();
            }
            (Some(Input::Number(n)), Some(Input::Number(m))) => match n.cmp(&m) {
                Ordering::Equal => {
                    a.pop_front();
                    b.pop_front();
                }
                ordering => return ordering,
            },
            (Some(Input::RightParen), _) => return Ordering::Less,
            (_, Some(Input::RightParen)) => return Ordering::Greater,
        }
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    parse!(input)
        .enumerate()
        .filter_map(|(idx, (a, b))| {
            if compare_packets(&a, &b) == Ordering::Less {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum()
}
#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut packets: Vec<VecDeque<Input>> = parse!(input)
        .flat_map(|(a, b)| vec![a, b].into_iter())
        .collect();
    let div_2 = VecDeque::from([
        Input::LeftParen,
        Input::LeftParen,
        Input::Number(2),
        Input::RightParen,
        Input::RightParen,
    ]);
    let div_6 = VecDeque::from([
        Input::LeftParen,
        Input::LeftParen,
        Input::Number(6),
        Input::RightParen,
        Input::RightParen,
    ]);
    packets.push(div_2.clone());
    packets.push(div_6.clone());
    packets.sort_by(compare_packets);
    (1 + packets.iter().find_position(|p| p == &&div_2).unwrap().0)
        * (1 + packets.iter().find_position(|p| p == &&div_6).unwrap().0)
}

#[test]
fn part1_test() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    assert_eq!(part1(input), 13);
}

#[test]
fn part2_test() {
    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
    assert_eq!(part2(input), 140);
}
