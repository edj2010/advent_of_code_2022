use advent_of_code::parse::{parsers, Parser};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

impl Operation {
    fn apply(self, a: i64, b: i64) -> i64 {
        match self {
            Self::Add => a + b,
            Self::Sub => a - b,
            Self::Mul => a * b,
            Self::Div => a / b,
        }
    }

    fn solve_left(self, b: i64, eq: i64) -> i64 {
        match self {
            Self::Add => eq - b,
            Self::Sub => eq + b,
            Self::Mul => eq / b,
            Self::Div => eq * b,
        }
    }

    fn solve_right(self, a: i64, eq: i64) -> i64 {
        match self {
            Self::Add => eq - a,
            Self::Sub => a - eq,
            Self::Mul => eq / a,
            Self::Div => a / eq,
        }
    }
}

#[derive(Clone, Debug)]
enum Monkey {
    Target,
    Resolved(i64),
    LeftResolved(Operation, i64, String),
    RightResolved(Operation, String, i64),
    Unresolved(Operation, String, String),
}

macro_rules! parse {
    ($input: ident) => {{
        let parse_monkey_id = parsers::char_any().repeat(4).map(|n| n.collect::<String>());
        parse_monkey_id
            .skip_tag(": ")
            .and_then(
                parsers::signed_number()
                    .map(|n| Monkey::Resolved(n as i64))
                    .or(parse_monkey_id
                        .skip_tag(" + ")
                        .and_then(parse_monkey_id)
                        .map(|(a, b)| Monkey::Unresolved(Operation::Add, a, b)))
                    .or(parse_monkey_id
                        .skip_tag(" - ")
                        .and_then(parse_monkey_id)
                        .map(|(a, b)| Monkey::Unresolved(Operation::Sub, a, b)))
                    .or(parse_monkey_id
                        .skip_tag(" * ")
                        .and_then(parse_monkey_id)
                        .map(|(a, b)| Monkey::Unresolved(Operation::Mul, a, b)))
                    .or(parse_monkey_id
                        .skip_tag(" / ")
                        .and_then(parse_monkey_id)
                        .map(|(a, b)| Monkey::Unresolved(Operation::Div, a, b))),
            )
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .collect::<HashMap<String, Monkey>>()
    }};
}

fn resolve_monkey(monkeys: &mut HashMap<String, Monkey>, monkey_id: &str) -> Monkey {
    let res = match monkeys.get(monkey_id).cloned().expect("missing monkey!") {
        s @ Monkey::Target => s,
        s @ Monkey::Resolved(_) => s,
        Monkey::LeftResolved(operation, n, id) => {
            if let Monkey::Resolved(m) = resolve_monkey(monkeys, &id) {
                Monkey::Resolved(operation.apply(n, m))
            } else {
                Monkey::LeftResolved(operation, n, id)
            }
        }
        Monkey::RightResolved(operation, id, m) => {
            if let Monkey::Resolved(n) = resolve_monkey(monkeys, &id) {
                Monkey::Resolved(operation.apply(n, m))
            } else {
                Monkey::RightResolved(operation, id, m)
            }
        }
        Monkey::Unresolved(operation, key_a, key_b) => {
            let a = resolve_monkey(monkeys, &key_a);
            let b = resolve_monkey(monkeys, &key_b);
            match (a, b) {
                (Monkey::Resolved(n), Monkey::Resolved(m)) => {
                    Monkey::Resolved(operation.apply(n, m))
                }
                (Monkey::Resolved(n), _) => Monkey::LeftResolved(operation, n, key_b),
                (_, Monkey::Resolved(m)) => Monkey::RightResolved(operation, key_a, m),
                (_, _) => Monkey::Unresolved(operation, key_a, key_b),
            }
        }
    };
    monkeys.insert(monkey_id.to_string(), res.clone());
    res
}

#[allow(dead_code)]
pub fn part1(input: &str) -> i64 {
    let mut monkeys = parse!(input);
    if let Monkey::Resolved(n) = resolve_monkey(&mut monkeys, "root") {
        n
    } else {
        panic!("failed to resolve root");
    }
}

fn attempt_solve(monkeys: &HashMap<String, Monkey>, monkey_id: &str, eq: i64) -> i64 {
    match monkeys[monkey_id].clone() {
        Monkey::Target => eq,
        Monkey::LeftResolved(operation, a, id) => {
            attempt_solve(monkeys, &id, operation.solve_right(a, eq))
        }
        Monkey::RightResolved(operation, id, b) => {
            attempt_solve(monkeys, &id, operation.solve_left(b, eq))
        }
        _ => panic!("unsolvable, failed at {}", monkey_id),
    }
}

#[allow(dead_code)]
pub fn part2(input: &str) -> i64 {
    let mut monkeys = parse!(input);
    monkeys.insert("humn".to_string(), Monkey::Target);
    match resolve_monkey(&mut monkeys, "root") {
        Monkey::LeftResolved(_, n, id) | Monkey::RightResolved(_, id, n) => {
            attempt_solve(&monkeys, &id, n)
        }
        v => panic!("root was unexpected value {:?}", v),
    }
}

#[test]
fn part1_test() {
    let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    assert_eq!(part1(input), 152);
}

#[test]
fn part2_test() {
    let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
";
    assert_eq!(part2(input), 301);
}
