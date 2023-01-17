use advent_of_code::parse::{parsers, Parser};
use std::{fmt::Debug, ops::Neg};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Digit {
    DoubleMinus,
    Minus,
    Zero,
    One,
    Two,
}

const DIGITS: [Digit; 5] = [
    Digit::Two,
    Digit::One,
    Digit::Zero,
    Digit::Minus,
    Digit::DoubleMinus,
];

impl Digit {
    fn to_num(self) -> i64 {
        match self {
            Digit::Two => 2,
            Digit::One => 1,
            Digit::Zero => 0,
            Digit::Minus => -1,
            Digit::DoubleMinus => -2,
        }
    }

    #[allow(dead_code)]
    fn of_char(c: char) -> Option<Self> {
        match c {
            '2' => Some(Digit::Two),
            '1' => Some(Digit::One),
            '0' => Some(Digit::Zero),
            '-' => Some(Digit::Minus),
            '=' => Some(Digit::DoubleMinus),
            _ => None,
        }
    }
}

impl ToString for Digit {
    fn to_string(&self) -> String {
        String::from(match self {
            Digit::Two => "2",
            Digit::One => "1",
            Digit::Zero => "0",
            Digit::Minus => "-",
            Digit::DoubleMinus => "=",
        })
    }
}

impl Debug for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Neg for Digit {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Digit::Two => Digit::DoubleMinus,
            Digit::One => Digit::Minus,
            Digit::Zero => Digit::Zero,
            Digit::Minus => Digit::One,
            Digit::DoubleMinus => Digit::Two,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Snafu(Vec<Digit>);

impl ToString for Snafu {
    fn to_string(&self) -> String {
        self.0.iter().map(|d| d.to_string()).collect()
    }
}

impl Debug for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Neg for Snafu {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Snafu(self.0.into_iter().map(|d| d.neg()).collect())
    }
}

impl Snafu {
    fn to_num(self) -> i64 {
        self.0.iter().fold(0, |acc, d| acc * 5 + d.to_num())
    }

    fn of_num(mut n: i64) -> Self {
        if n == 0 {
            return Snafu(vec![Digit::Zero]);
        } else if n < 0 {
            return -Snafu::of_num(-n);
        }
        let mut divisor = 1;
        let mut max_val = 2;
        while max_val < n {
            max_val = 5 * max_val + 2;
            divisor *= 5;
        }
        let mut digits = vec![];
        while divisor > 0 {
            let (_, next_digit) = DIGITS
                .iter()
                .map(|d| ((n - d.to_num() * divisor).abs(), *d))
                .min()
                .unwrap();
            digits.push(next_digit);
            n -= next_digit.to_num() * divisor;
            divisor /= 5;
        }
        Snafu(digits)
    }
}

macro_rules! parse {
    ($input: ident) => {
        parsers::char('2')
            .map(|_| Digit::Two)
            .or(parsers::char('1').map(|_| Digit::One))
            .or(parsers::char('0').map(|_| Digit::Zero))
            .or(parsers::char('-').map(|_| Digit::Minus))
            .or(parsers::char('=').map(|_| Digit::DoubleMinus))
            .many()
            .map(|v| Snafu(v.collect::<Vec<Digit>>()))
            .many_lines("\n")
            .map(|v| v.collect::<Vec<Snafu>>())
            .parse($input)
            .finish()
            .unwrap()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> String {
    let snafus = parse!(input);
    let total: i64 = snafus.into_iter().map(|v| v.to_num()).sum();
    Snafu::of_num(total).to_string()
}

#[allow(dead_code)]
pub fn part2(_: &str) -> u32 {
    0
}

#[test]
fn snafu_to_num() {
    let snafus: Vec<i64> = [
        "1",
        "2",
        "1=",
        "1-",
        "10",
        "11",
        "12",
        "2=",
        "2-",
        "20",
        "1=0",
        "1-0",
        "1=11-2",
        "1-0---0",
        "1121-1110-1=0",
    ]
    .into_iter()
    .map(|s| Snafu(s.chars().filter_map(Digit::of_char).collect()).to_num())
    .collect();
    assert_eq!(
        snafus,
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265]
    );
}

#[test]
fn snafu_of_num() {
    let snafus: Vec<Snafu> = [
        "1",
        "2",
        "1=",
        "1-",
        "10",
        "11",
        "12",
        "2=",
        "2-",
        "20",
        "1=0",
        "1-0",
        "1=11-2",
        "1-0---0",
        "1121-1110-1=0",
    ]
    .into_iter()
    .map(|s| Snafu(s.chars().filter_map(Digit::of_char).collect()))
    .collect();
    let computed_snafus: Vec<Snafu> = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15, 20, 2022, 12345, 314159265,
    ]
    .into_iter()
    .map(Snafu::of_num)
    .collect();
    assert_eq!(snafus, computed_snafus);
}

#[test]
fn part1_test() {
    let input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";
    assert_eq!(part1(input), "2=-1=0");
}

#[test]
fn part2_test() {
    let input = "";
    assert_eq!(part2(input), 0);
}
