use advent_of_code::parse::{parsers, Parser};
use std::cmp::max;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

impl Blueprint {
    fn max_ore_cost(self) -> u32 {
        max(
            max(self.ore_robot_ore_cost, self.clay_robot_ore_cost),
            max(self.obsidian_robot_ore_cost, self.geode_robot_ore_cost),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ProblemState {
    ore_robot_count: u32,
    clay_robot_count: u32,
    obsidian_robot_count: u32,
    geode_robot_count: u32,
    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    skipped_ore_build: bool,
    skipped_clay_build: bool,
    skipped_obsidian_build: bool,
    skipped_geode_build: bool,
    time_left: u32,
}

impl ProblemState {
    fn init(time_left: u32) -> Self {
        ProblemState {
            ore_robot_count: 1,
            clay_robot_count: 0,
            obsidian_robot_count: 0,
            geode_robot_count: 0,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            skipped_ore_build: false,
            skipped_clay_build: false,
            skipped_obsidian_build: false,
            skipped_geode_build: false,
            time_left,
        }
    }

    fn mine(&mut self) -> u32 {
        self.ore_count += self.ore_robot_count;
        self.clay_count += self.clay_robot_count;
        self.obsidian_count += self.obsidian_robot_count;
        self.time_left -= 1;
        self.geode_robot_count
    }

    fn can_build_ore(&self, blueprint: Blueprint) -> bool {
        self.ore_count >= blueprint.ore_robot_ore_cost
            && !self.skipped_ore_build
            && self.ore_robot_count < blueprint.max_ore_cost()
    }

    fn can_build_clay(&self, blueprint: Blueprint) -> bool {
        self.ore_count >= blueprint.clay_robot_ore_cost
            && !self.skipped_clay_build
            && self.clay_robot_count < blueprint.obsidian_robot_clay_cost
    }

    fn can_build_obsidian(&self, blueprint: Blueprint) -> bool {
        self.ore_count >= blueprint.obsidian_robot_ore_cost
            && self.clay_count >= blueprint.obsidian_robot_clay_cost
            && !self.skipped_obsidian_build
            && self.obsidian_robot_count < blueprint.geode_robot_obsidian_cost
    }

    fn can_build_geode(&self, blueprint: Blueprint) -> bool {
        self.ore_count >= blueprint.geode_robot_ore_cost
            && self.obsidian_count >= blueprint.geode_robot_obsidian_cost
            && !self.skipped_geode_build
    }

    fn reset_skipped(&mut self) {
        self.skipped_ore_build = false;
        self.skipped_clay_build = false;
        self.skipped_obsidian_build = false;
        self.skipped_geode_build = false;
    }

    fn build_ore(&mut self, blueprint: Blueprint) {
        self.ore_count -= blueprint.ore_robot_ore_cost;
        self.ore_robot_count += 1;
        self.reset_skipped();
    }

    fn build_clay(&mut self, blueprint: Blueprint) {
        self.ore_count -= blueprint.clay_robot_ore_cost;
        self.clay_robot_count += 1;
        self.reset_skipped();
    }

    fn build_obsidian(&mut self, blueprint: Blueprint) {
        self.ore_count -= blueprint.obsidian_robot_ore_cost;
        self.clay_count -= blueprint.obsidian_robot_clay_cost;
        self.obsidian_robot_count += 1;
        self.reset_skipped();
    }

    fn build_geode(&mut self, blueprint: Blueprint) {
        self.ore_count -= blueprint.geode_robot_ore_cost;
        self.obsidian_count -= blueprint.geode_robot_obsidian_cost;
        self.geode_robot_count += 1;
        self.reset_skipped();
    }

    fn next_states(self, blueprint: Blueprint) -> Vec<(Self, u32)> {
        let mut to_ret = vec![];
        let mut no_action = self.clone();
        if self.can_build_ore(blueprint) {
            let mut build_ore = self.clone();
            let geodes = build_ore.mine();
            build_ore.build_ore(blueprint);
            to_ret.push((build_ore, geodes));
            no_action.skipped_ore_build = true;
        }
        if self.can_build_clay(blueprint) {
            let mut build_clay = self.clone();
            let geodes = build_clay.mine();
            build_clay.build_clay(blueprint);
            to_ret.push((build_clay, geodes));
            no_action.skipped_clay_build = true;
        }
        if self.can_build_obsidian(blueprint) {
            let mut build_obsidian = self.clone();
            let geodes = build_obsidian.mine();
            build_obsidian.build_obsidian(blueprint);
            to_ret.push((build_obsidian, geodes));
            no_action.skipped_obsidian_build = true;
        }
        if self.can_build_geode(blueprint) {
            let mut build_geode = self.clone();
            let geodes = build_geode.mine();
            build_geode.build_geode(blueprint);
            to_ret.push((build_geode, geodes));
            no_action.skipped_geode_build = true;
        }
        {
            let geodes = no_action.mine();
            to_ret.push((no_action, geodes));
        }
        to_ret
    }
}

macro_rules! parse {
    ($input: ident) => {
        parsers::tag("Blueprint ")
            .ignore(parsers::number())
            .skip_tag(": Each ore robot costs ")
            .and_then(parsers::number())
            .skip_tag(" ore. Each clay robot costs ")
            .and_then(parsers::number())
            .skip_tag(" ore. Each obsidian robot costs ")
            .and_then(parsers::number())
            .skip_tag(" ore and ")
            .and_then(parsers::number())
            .skip_tag(" clay. Each geode robot costs ")
            .and_then(parsers::number())
            .skip_tag(" ore and ")
            .and_then(parsers::number())
            .skip_tag(" obsidian.")
            .map(|((((((a, b), c), d), e), f), g)| Blueprint {
                id: a,
                ore_robot_ore_cost: b,
                clay_robot_ore_cost: c,
                obsidian_robot_ore_cost: d,
                obsidian_robot_clay_cost: e,
                geode_robot_ore_cost: f,
                geode_robot_obsidian_cost: g,
            })
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .collect::<Vec<Blueprint>>()
    };
}

fn optimize_blueprint(
    known_states: &mut HashMap<ProblemState, u32>,
    state: ProblemState,
    blueprint: Blueprint,
) -> u32 {
    if state.time_left == 1 {
        return state.clone().mine();
    }
    if let Some(ret) = known_states.get(&state) {
        return *ret;
    }
    let ret = state
        .next_states(blueprint)
        .into_iter()
        .map(|(next_state, geodes)| {
            optimize_blueprint(known_states, next_state, blueprint) + geodes
        })
        .max()
        .unwrap_or(0);
    known_states.insert(state, ret);
    ret
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let init_state = ProblemState::init(24);
    parse!(input)
        .into_iter()
        .map(|blueprint| {
            println!("blueprint {}", blueprint.id);
            let mut known_states: HashMap<ProblemState, u32> = HashMap::new();
            optimize_blueprint(&mut known_states, init_state, blueprint) * blueprint.id
        })
        .sum()
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let init_state = ProblemState::init(32);
    parse!(input)
        .into_iter()
        .take(3)
        .map(|blueprint| {
            println!("blueprint {}", blueprint.id);
            let mut known_states: HashMap<ProblemState, u32> = HashMap::new();
            optimize_blueprint(&mut known_states, init_state, blueprint)
        })
        .product()
}

#[test]
fn part1_test() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";
    assert_eq!(part1(input), 33);
}

#[test]
fn part2_test() {
    let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";
    assert_eq!(part2(input), 62 * 56);
}
