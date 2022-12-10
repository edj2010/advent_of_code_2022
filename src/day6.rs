use std::collections::BTreeSet;

fn distinct(s: &str) -> bool {
    let unique = s.chars().collect::<BTreeSet<char>>();
    unique.len() == s.len()
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    (0..input.len())
        .find_map(|idx| {
            if distinct(&input[idx..(idx + 4)]) {
                Some(idx + 4)
            } else {
                None
            }
        })
        .unwrap()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    (0..input.len())
        .find_map(|idx| {
            if distinct(&input[idx..(idx + 14)]) {
                Some(idx + 14)
            } else {
                None
            }
        })
        .unwrap()
}
