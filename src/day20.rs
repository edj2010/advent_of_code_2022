use advent_of_code::parse::{parsers, Parser};

macro_rules! parse {
    ($input: ident) => {
        parsers::signed_number()
            .map(|n| n as isize)
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .collect::<Vec<isize>>()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> isize {
    let offsets = parse!(input);
    let mut array = (0..offsets.len()).collect::<Vec<usize>>();
    let mut next_to_move = 0;
    let mut idx = 0;
    while next_to_move < array.len() {
        while array[idx] != next_to_move {
            idx = (idx + 1) % array.len();
        }
        let item = array.remove(idx);
        let new_idx = (((idx as isize) + offsets[item]).rem_euclid(array.len() as isize)) as usize;
        array.insert(new_idx, item);
        next_to_move += 1;
    }
    let zero_idx = (0..array.len())
        .find(|idx| offsets[array[*idx]] == 0)
        .unwrap();
    (1000..=3000)
        .step_by(1000)
        .map(|idx| offsets[array[(idx + zero_idx) % array.len()]])
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> isize {
    let decryption_key = 811589153;
    let offsets = parse!(input)
        .into_iter()
        .map(|v| v * decryption_key)
        .collect::<Vec<isize>>();
    let mut array = (0..offsets.len()).collect::<Vec<usize>>();
    for _ in 0..10 {
        let mut next_to_move = 0;
        let mut idx = 0;
        while next_to_move < array.len() {
            while array[idx] != next_to_move {
                idx = (idx + 1) % array.len();
            }
            let item = array.remove(idx);
            let new_idx =
                (((idx as isize) + offsets[item]).rem_euclid(array.len() as isize)) as usize;
            array.insert(new_idx, item);
            next_to_move += 1;
        }
    }
    let zero_idx = (0..array.len())
        .find(|idx| offsets[array[*idx]] == 0)
        .unwrap();
    (1000..=3000)
        .step_by(1000)
        .map(|idx| offsets[array[(idx + zero_idx) % array.len()]])
        .sum()
}

#[test]
fn part1_test() {
    let input = "1
2
-3
3
-2
0
4
";
    assert_eq!(part1(input), 3);
}

#[test]
fn part2_test() {
    let input = "1
2
-3
3
-2
0
4
";
    assert_eq!(part2(input), 1623178306);
}
