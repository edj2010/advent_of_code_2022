use advent_of_code::parse::{parsers, Parser};

macro_rules! parse {
    ($input: ident) => {{
        let (crates, instrs) = parsers::char('[')
            .ignore(parsers::char_any())
            .skip(parsers::char_any())
            .map(|c: char| Some(c))
            .or(parsers::tag("   ").map(|_| Option::<char>::None))
            .list(" ")
            .list("\n")
            .skip(parsers::many_chars(|c| c != '\n').line("\n").repeat(2))
            .pair(
                "\n",
                parsers::tag("move ")
                    .ignore(parsers::number())
                    .skip_tag(" from ")
                    .and_then(parsers::number())
                    .skip_tag(" to ")
                    .and_then(parsers::number())
                    .skip_tag("\n")
                    .map(|((a, b), c)| (a, b - 1, c - 1))
                    .many(),
            )
            .parse($input)
            .finish()
            .unwrap();
        let crates: Vec<Vec<Option<char>>> =
            crates.map(|v| v.collect::<Vec<Option<char>>>()).collect();
        (
            (0..crates[0].len())
                .map(|col_idx| {
                    (0..crates.len())
                        .rev()
                        .filter_map(|row_idx| crates[row_idx][col_idx])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>(),
            instrs,
        )
    }};
}

#[allow(dead_code)]
pub fn part1(input: &str) -> String {
    let (mut crates, instrs) = parse!(input);
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
    let (mut crates, instrs) = parse!(input);
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
fn test_part1() {
    let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
    assert_eq!(part1(&input), "CMZ");
}
