fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut iter = input.split("\n\n").map(|s| s.lines());
    let crates = iter.next().unwrap().collect::<Vec<&str>>();
    let instrs = iter.next().unwrap();

    (
        (1_usize..crates[0].len())
            .step_by(4)
            .map(|crate_idx| {
                (0..crates.len())
                    .rev()
                    .skip(1)
                    .map(|row_idx| crates[row_idx].chars().nth(crate_idx).unwrap())
                    .filter(|c| *c != ' ')
                    .collect::<Vec<char>>()
            })
            .collect(),
        instrs
            .map(|s| {
                let (_, r) = s.split_once("move ").unwrap();
                let (a, r) = r.split_once(" from ").unwrap();
                let (b, c) = r.split_once(" to ").unwrap();
                (
                    a.parse::<usize>().unwrap(),
                    b.parse::<usize>().unwrap() - 1,
                    c.parse::<usize>().unwrap() - 1,
                )
            })
            .collect(),
    )
}

#[allow(dead_code)]
pub fn part1(input: &str) -> String {
    let (mut crates, instrs) = parse(input);
    for (count, from, to) in instrs {
        for _ in 0..count {
            let popped = crates[from].pop().unwrap();
            crates[to].push(popped);
        }
    }

    crates
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> String {
    let (mut crates, instrs) = parse(input);
    for (count, from, to) in instrs {
        let at = crates[from].len() - count;
        let popped = crates[from].split_off(at);
        crates[to].extend(popped);
    }

    crates
        .into_iter()
        .map(|stack| stack.last().unwrap().clone())
        .collect()
}

#[test]
fn part1_test() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    assert_eq!(part1(&input), "CMZ");
}
