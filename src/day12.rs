use advent_of_code::{
    grid::{Grid, GridPoint, PLUS_ADJACENT},
    parse::{parsers, Parser},
    search::WeightedGraph,
};

macro_rules! parser {
    ($input: ident) => {{
        let vec_of_vecs = parsers::chars(|c| c.is_alphabetic())
            .map(|c| u8::try_from(c).unwrap())
            .many()
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .map(|v| v.collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        Grid::of_vec_of_vecs(vec_of_vecs).unwrap()
    }};
}

struct Mountain<F: Fn(u8, u8) -> bool> {
    map: Grid<u8>,
    height_map: Grid<u8>,
    adjacent_filter: F,
}

impl<F: Fn(u8, u8) -> bool> WeightedGraph<GridPoint<usize>, u32> for Mountain<F> {
    fn adjacent(&self, a: &GridPoint<usize>) -> Option<impl Iterator<Item = GridPoint<usize>>> {
        Some(
            PLUS_ADJACENT
                .into_iter()
                .filter_map(move |delta| {
                    a.add_checked(delta, &0, &self.map.rows(), &0, &self.map.cols())
                })
                .filter(move |other| {
                    (self.adjacent_filter)(self.height_map[*a], self.height_map[*other])
                })
                .collect::<Vec<GridPoint<usize>>>()
                .into_iter(),
        )
    }

    fn weight(&self, _: &GridPoint<usize>, _: &GridPoint<usize>) -> Option<u32> {
        Some(1)
    }
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let map: Grid<u8> = parser!(input);
    let height_map = Grid::from(
        map.clone().into_iter().map(|c| {
            if c == b'S' {
                b'a'
            } else if c == b'E' {
                b'z'
            } else {
                c
            }
        }),
        map.rows(),
        map.cols(),
    )
    .unwrap();
    let start_idx = map.find(&b'S').unwrap();
    let graph = Mountain {
        map,
        height_map,
        adjacent_filter: |a, b| a + 1 >= b,
    };
    graph
        .shortest_distance(start_idx, |end| graph.map[*end] == b'E', 0)
        .unwrap()
        .1
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let map: Grid<u8> = parser!(input);
    let height_map = Grid::from(
        map.clone().into_iter().map(|c| {
            if c == b'S' {
                b'a'
            } else if c == b'E' {
                b'z'
            } else {
                c
            }
        }),
        map.rows(),
        map.cols(),
    )
    .unwrap();
    let start_idx = map.find(&b'E').unwrap();
    let graph = Mountain {
        map,
        height_map,
        adjacent_filter: |a, b| a <= b + 1,
    };
    graph
        .shortest_distance(start_idx, |end| graph.map[*end] == b'a', 0)
        .unwrap()
        .1
}

#[test]
fn part1_test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    assert_eq!(31, part1(input));
}

#[test]
fn part2_test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";
    assert_eq!(29, part2(input));
}
