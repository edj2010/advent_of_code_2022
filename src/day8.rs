use advent_of_code::{
    grid::{Grid, GridPoint, EAST, NORTH, SOUTH, WEST},
    parse::{parsers, Parser},
};

fn mark_visible<I: Iterator<Item = GridPoint<usize>>>(
    it: I,
    grid: &Grid<char>,
    visible: &mut Grid<u32>,
) {
    let mut peak = ' ';
    for point in it {
        let c = *grid.get(point).unwrap();
        if c > peak {
            peak = c;
            visible.set(point, 1).unwrap();
        }
    }
}

macro_rules! parse {
    ($input: ident) => {
        parsers::chars(|c| c.is_numeric())
            .many()
            .many_lines("\n")
            .parse($input)
            .finish()
            .unwrap()
            .map(|l| l.collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
    };
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let grid = Grid::of_vec_of_vecs(parse!(input)).unwrap();
    let rows = grid.rows();
    let cols = grid.cols();
    let mut visible: Grid<u32> = Grid::init(0, rows, cols);

    GridPoint::new(0, 0)
        .traverse_by(SOUTH, 0, rows, 0, cols)
        .for_each(|start| {
            mark_visible(
                start.traverse_by(EAST, 0, rows, 0, cols),
                &grid,
                &mut visible,
            )
        });
    GridPoint::new(0, 0)
        .traverse_by(EAST, 0, rows, 0, cols)
        .for_each(|start| {
            mark_visible(
                start.traverse_by(SOUTH, 0, rows, 0, cols),
                &grid,
                &mut visible,
            )
        });
    GridPoint::new(rows - 1, 0)
        .traverse_by(EAST, 0, rows, 0, cols)
        .for_each(|start| {
            mark_visible(
                start.traverse_by(NORTH, 0, rows, 0, cols),
                &grid,
                &mut visible,
            )
        });
    GridPoint::new(0, cols - 1)
        .traverse_by(SOUTH, 0, rows, 0, cols)
        .for_each(|start| {
            mark_visible(
                start.traverse_by(WEST, 0, rows, 0, cols),
                &grid,
                &mut visible,
            )
        });

    visible.into_iter().sum()
}

#[derive(Debug, Clone, Copy)]
struct Tree([usize; 4]);

impl Tree {
    fn new() -> Self {
        Tree([0; 4])
    }

    fn product(&self) -> usize {
        self.0.into_iter().product()
    }

    fn set(&mut self, which: usize, value: usize) {
        self.0[which] = value;
    }
}

fn mark_visible_direction<I: Iterator<Item = GridPoint<usize>>>(
    it: I,
    which: usize,
    grid: &Grid<char>,
    visible: &mut Grid<Tree>,
) {
    let mut peaks: Vec<(usize, char)> = Vec::new();
    for (idx, point) in it.enumerate() {
        let c = *grid.get(point).unwrap();
        if let Some((mut last_idx, mut last_peak)) = peaks.pop() {
            while last_peak < c && peaks.len() >= 1 {
                (last_idx, last_peak) = peaks.pop().unwrap();
            }
            if last_peak >= c {
                peaks.push((last_idx, last_peak));
            } else {
                last_idx = 0;
            }
            visible.get_mut(point).unwrap().set(which, idx - last_idx);
        }
        peaks.push((idx, c));
    }
}

#[allow(dead_code)]
pub fn part2(input: &str) -> usize {
    let grid = Grid::of_vec_of_vecs(parse!(input)).unwrap();
    let rows = grid.rows();
    let cols = grid.cols();
    let mut visible: Grid<Tree> = Grid::init(Tree::new(), rows, cols);

    GridPoint::new(0, 0)
        .traverse_by(SOUTH, 0, rows, 0, cols)
        .for_each(|start| {
            mark_visible_direction(
                start.traverse_by(EAST, 0, rows, 0, cols),
                0,
                &grid,
                &mut visible,
            )
        });
    GridPoint::new(0, 0)
        .traverse_by(EAST, 0, rows, 0, cols)
        .for_each(|start| {
            mark_visible_direction(
                start.traverse_by(SOUTH, 0, rows, 0, cols),
                1,
                &grid,
                &mut visible,
            )
        });
    GridPoint::new(rows - 1, 0)
        .traverse_by(EAST, 0, rows, 0, cols)
        .for_each(|start| {
            mark_visible_direction(
                start.traverse_by(NORTH, 0, rows, 0, cols),
                2,
                &grid,
                &mut visible,
            )
        });
    GridPoint::new(0, cols - 1)
        .traverse_by(SOUTH, 0, rows, 0, cols)
        .for_each(|start| {
            mark_visible_direction(
                start.traverse_by(WEST, 0, rows, 0, cols),
                3,
                &grid,
                &mut visible,
            )
        });

    visible
        .into_iter()
        .map(|tree| tree.product())
        .max()
        .unwrap()
}

#[test]
fn test_part1() {
    let input = "30373
25512
65332
33549
35390
";
    assert_eq!(part1(input), 21);
}

#[test]
fn test_part2() {
    let input = "30373
25512
65332
33549
35390
";
    assert_eq!(part2(input), 8);
}
