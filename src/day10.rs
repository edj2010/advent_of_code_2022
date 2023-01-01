use advent_of_code::parse::{parsers, Parser};

#[derive(Debug, Clone, Copy)]
enum Command {
    Noop,
    Addx(i32),
}

macro_rules! parse {
    ($input: ident) => {
        parsers::tag("addx ")
            .ignore(parsers::signed_number())
            .map(|number| vec![Command::Noop, Command::Addx(number)])
            .or(parsers::tag("noop").map(|_| vec![Command::Noop]))
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .flat_map(|v| v.into_iter())
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i32 {
    parse!(input)
        .fold((1, 1, 0), |(x, idx, mut sum), instr| {
            if (idx - 20) % 40 == 0 {
                sum += idx * x;
            }
            match instr {
                Command::Noop => (x, idx + 1, sum),
                Command::Addx(n) => (x + n, idx + 1, sum),
            }
        })
        .2
}

#[allow(dead_code)]
pub fn part2(input: &str) -> String {
    let mut display: String = String::new();
    parse!(input).enumerate().fold(1_i32, |x, (idx, instr)| {
        let pos = (idx % 40) as i32;
        if pos == 0 {
            display.push('\n');
        }
        if x - 1 <= pos && x + 1 >= pos {
            display.push('#');
        } else {
            display.push('.');
        }
        match instr {
            Command::Noop => x,
            Command::Addx(n) => x + n,
        }
    });
    display
}

#[test]
fn part1_test() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
    assert_eq!(part1(input), 13140);
}

#[test]
fn part2_test() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
    assert_eq!(
        part2(input),
        "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
            .to_owned()
    );
}
