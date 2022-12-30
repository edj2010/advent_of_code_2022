use advent_of_code::{
    parse::{parsers, Parser},
    search::WeightedGraph,
};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

macro_rules! parse {
    ($input: ident) => {
        parsers::tag("Valve ")
            .ignore(parsers::char_any().repeat(2).map(|s| s.collect::<String>()))
            .skip_tag(" has flow rate=")
            .and_then(parsers::number())
            .skip_tag("; ")
            .skip(
                parsers::tag("tunnels lead to valves ").or(parsers::tag("tunnel leads to valve ")),
            )
            .and_then(
                parsers::char_any()
                    .repeat(2)
                    .map(|s| s.collect::<String>())
                    .list(", ")
                    .map(|v| v.collect::<Vec<String>>()),
            )
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .collect::<Vec<((String, usize), Vec<String>)>>()
    };
}

fn simplify_graph(graph: &HashMap<String, Vec<String>>) -> HashMap<String, HashMap<String, usize>> {
    let weighted_graph = graph
        .into_iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.into_iter()
                    .map(|k2| (k2.clone(), 1))
                    .collect::<HashMap<String, usize>>(),
            )
        })
        .collect::<HashMap<String, HashMap<String, usize>>>();
    weighted_graph.all_pairs_shortest_paths(weighted_graph.keys().cloned(), 0)
}

fn filter_graph(
    graph: &HashMap<String, HashMap<String, usize>>,
    desired_nodes: HashSet<String>,
) -> HashMap<String, HashMap<String, usize>> {
    graph
        .into_iter()
        .filter_map(|(k, vs)| {
            if desired_nodes.contains(k) {
                Some((
                    k.clone(),
                    vs.into_iter()
                        .filter_map(|(k, v)| {
                            if desired_nodes.contains(k) {
                                Some((k.clone(), *v))
                            } else {
                                None
                            }
                        })
                        .collect(),
                ))
            } else {
                None
            }
        })
        .collect()
}

fn explore(
    graph: &HashMap<String, HashMap<String, usize>>,
    valves: &HashMap<String, usize>,
    on: HashSet<String>,
    node: String,
    time_left: usize,
) -> usize {
    match graph.get(&node) {
        None => 0,
        Some(adjacent) => adjacent
            .iter()
            .filter_map(|(next, distance)| {
                if distance + 1 < time_left && !on.contains(next) {
                    let mut new_on = on.clone();
                    new_on.insert(next.clone());
                    let additional_weight = (time_left - distance - 1) * valves.get(next)?;
                    Some(
                        additional_weight
                            + explore(
                                graph,
                                valves,
                                new_on,
                                next.clone(),
                                time_left - distance - 1,
                            ),
                    )
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0),
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> usize {
    let data = parse!(input);
    let valves = data
        .iter()
        .filter_map(|((k, v), _)| if *v > 0 { Some((k.clone(), *v)) } else { None })
        .collect::<HashMap<String, usize>>();
    let graph = filter_graph(
        &simplify_graph(
            &data
                .into_iter()
                .map(|((k, _), v)| (k, v))
                .collect::<HashMap<String, Vec<String>>>(),
        ),
        valves
            .keys()
            .cloned()
            .chain(vec!["AA".to_owned()].into_iter())
            .collect(),
    );
    explore(&graph, &valves, HashSet::new(), "AA".to_owned(), 30)
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let data = parse!(input);
    let valves = data
        .iter()
        .filter_map(|((k, v), _)| if *v > 0 { Some((k.clone(), *v)) } else { None })
        .collect::<HashMap<String, usize>>();
    let graph = filter_graph(
        &simplify_graph(
            &data
                .into_iter()
                .map(|((k, _), v)| (k, v))
                .collect::<HashMap<String, Vec<String>>>(),
        ),
        valves
            .keys()
            .cloned()
            .chain(vec!["AA".to_owned()].into_iter())
            .collect(),
    );
    let all_valves = valves.keys().cloned().collect::<Vec<String>>();
    (0..=(valves.len() + 1) / 2)
        .filter_map(|comb| {
            println!("{:?}", comb);
            (0..valves.len())
                .combinations(comb)
                .map(|subset_idx| {
                    let my_graph = filter_graph(
                        &graph,
                        all_valves
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, v)| {
                                if subset_idx.contains(&idx) {
                                    Some(v)
                                } else {
                                    None
                                }
                            })
                            .cloned()
                            .chain(vec!["AA".to_owned()].into_iter())
                            .collect(),
                    );
                    let elephant_graph = filter_graph(
                        &graph,
                        all_valves
                            .iter()
                            .enumerate()
                            .filter_map(|(idx, v)| {
                                if subset_idx.contains(&idx) {
                                    None
                                } else {
                                    Some(v)
                                }
                            })
                            .cloned()
                            .chain(vec!["AA".to_owned()].into_iter())
                            .collect(),
                    );
                    explore(&my_graph, &valves, HashSet::new(), "AA".to_owned(), 26)
                        + explore(
                            &elephant_graph,
                            &valves,
                            HashSet::new(),
                            "AA".to_owned(),
                            26,
                        )
                })
                .max()
        })
        .max()
        .unwrap()
}

#[test]
fn part1_test() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
    assert_eq!(part1(input), 1651);
}

#[test]
fn part2_test() {
    let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";
    assert_eq!(part2(input), 1707);
}
