use advent_of_code::parse::{parsers, Parser};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add(usize),
    Multiply(usize),
    Square,
}

impl Operation {
    fn apply(self, item: usize) -> usize {
        match self {
            Self::Add(n) => item + n,
            Self::Multiply(n) => item * n,
            Self::Square => item * item,
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    divisor: usize,
    true_target_id: usize,
    false_target_id: usize,
}

macro_rules! parse {
    ($input: ident) => {
        parsers::tag("Monkey ")
            .ignore(parsers::number())
            .skip(parsers::tag(":\n  Starting items: "))
            .and_then(parsers::number().list(", ").line("\n"))
            .skip(parsers::tag("  Operation: new = old "))
            .and_then(
                parsers::tag("+ ")
                    .ignore(parsers::number())
                    .map(|n| Operation::Add(n))
                    .or(parsers::tag("* ")
                        .ignore(parsers::number())
                        .map(|n| Operation::Multiply(n)))
                    .or(parsers::tag("* old").map(|_| Operation::Square))
                    .line("\n"),
            )
            .skip(parsers::tag("  Test: divisible by "))
            .and_then(parsers::number().line("\n"))
            .skip(parsers::tag("    If true: throw to monkey "))
            .and_then(parsers::number().line("\n"))
            .skip(parsers::tag("    If false: throw to monkey "))
            .and_then(parsers::number().line("\n"))
            .map(
                |(((((_id, list), operation), divisor), true_target_id), false_target_id)| Monkey {
                    items: list.collect(),
                    operation,
                    divisor,
                    true_target_id,
                    false_target_id,
                },
            )
            .list("\n")
            .parse($input)
            .finish()
            .unwrap()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = parse!(input).collect();
    let monkey_count = monkeys.len();
    let mut inspections: Vec<usize> = vec![0; monkey_count];
    for _ in 0..20 {
        for monkey_idx in 0..monkey_count {
            let items = monkeys[monkey_idx].items.clone();
            monkeys[monkey_idx].items = vec![];
            let monkey = monkeys[monkey_idx].clone();
            for item in items {
                inspections[monkey_idx] += 1;
                let new_item = monkey.operation.apply(item) / 3;
                if new_item % monkey.divisor == 0 {
                    monkeys[monkey.true_target_id].items.push(new_item);
                } else {
                    monkeys[monkey.false_target_id].items.push(new_item);
                }
            }
        }
    }
    inspections.sort();
    inspections[monkey_count - 1] * inspections[monkey_count - 2]
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = parse!(input).collect();
    let modulus: usize = monkeys.iter().map(|monkey| monkey.divisor).product();
    let monkey_count = monkeys.len();
    let mut inspections: Vec<usize> = vec![0; monkey_count];
    for _ in 0..10000 {
        for monkey_idx in 0..monkey_count {
            let items = monkeys[monkey_idx].items.clone();
            monkeys[monkey_idx].items = vec![];
            let monkey = monkeys[monkey_idx].clone();
            for item in items {
                inspections[monkey_idx] += 1;
                let new_item = monkey.operation.apply(item) % modulus;
                if new_item % monkey.divisor == 0 {
                    monkeys[monkey.true_target_id].items.push(new_item);
                } else {
                    monkeys[monkey.false_target_id].items.push(new_item);
                }
            }
        }
    }
    inspections.sort();
    inspections[monkey_count - 1] * inspections[monkey_count - 2]
}

#[test]
fn part1_test() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    assert_eq!(part1(input), 10605);
}

#[test]
fn part2_test() {
    let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";
    assert_eq!(part2(input), 2713310158);
}
