use advent_of_code::{
    grid::{
        Direction::{self, East, North, South, West},
        Grid, GridPoint, GridPointDelta, EAST,
    },
    parse::{parsers, Parser},
};

enum Instruction {
    Left,
    Right,
    Move(u32),
}

fn score_orientation(orientation: Direction) -> u32 {
    match orientation {
        Direction::East => 0,
        Direction::North => 3,
        Direction::West => 2,
        Direction::South => 1,
    }
}

macro_rules! parse {
    ($input: ident) => {
        parsers::char(' ')
            .or(parsers::char('.'))
            .or(parsers::char('#'))
            .many_at_least_one()
            .map(|v| v.collect::<Vec<char>>())
            .many_lines("\n")
            .map(|v| {
                let mut vec_of_vecs = v.collect::<Vec<Vec<char>>>();
                let width = vec_of_vecs.iter().map(|l| l.len()).max().unwrap_or(0);
                vec_of_vecs
                    .iter_mut()
                    .for_each(|l| l.extend((0..(width - l.len())).map(|_| ' ')));
                Grid::of_vec_of_vecs(vec_of_vecs).unwrap()
            })
            .skip_tag("\n")
            .and_then(
                parsers::number()
                    .map(|n| Instruction::Move(n))
                    .or(parsers::char('R').map(|_| Instruction::Right))
                    .or(parsers::char('L').map(|_| Instruction::Left))
                    .many(),
            )
            .line("\n")
            .parse($input)
            .finish()
            .unwrap()
    };
}

fn wrap(
    grid: &Grid<char>,
    point: GridPoint<usize>,
    orientation: Direction,
    rows: usize,
    cols: usize,
) -> (GridPoint<usize>, Direction) {
    (
        point
            .traverse_by(GridPointDelta::<isize>::from(-orientation), 0, rows, 0, cols)
            .take_while(|p| grid[*p] != ' ')
            .last()
            .unwrap(),
        orientation,
    )
}

fn wrap_cube(point: GridPoint<usize>, orientation: Direction) -> (GridPoint<usize>, Direction) {
    match (point, orientation) {
        (GridPoint { row: 0, col }, North) if col < 100 => (GridPoint::new(col + 100, 0), East), // a
        (GridPoint { row, col: 0 }, West) if row >= 150 => (GridPoint::new(0, row - 100), South), // a
        (GridPoint { row, col: 0 }, West) if row < 150 => (GridPoint::new(149 - row, 50), East), // b
        (GridPoint { row, col: 50 }, West) if row < 50 => (GridPoint::new(149 - row, 0), East), // b
        (GridPoint { row, col: 99 }, East) if row >= 100 => (GridPoint::new(149 - row, 149), West), // c
        (GridPoint { row, col: 149 }, East) => (GridPoint::new(149 - row, 99), West), // c
        (GridPoint { row: 0, col }, North) if col >= 100 => (GridPoint::new(199, col - 100), North), // d
        (GridPoint { row: 199, col }, South) => (GridPoint::new(0, col + 100), South), // d
        (GridPoint { row: 100, col }, North) => (GridPoint::new(col + 50, 50), East),  // e
        (GridPoint { row, col: 50 }, West) if row >= 50 => (GridPoint::new(100, row - 50), South), // e
        (GridPoint { row, col: 99 }, East) if row < 100 => (GridPoint::new(49, row + 50), North), // f
        (GridPoint { row: 49, col }, South) => (GridPoint::new(col - 50, 99), West), // f
        (GridPoint { row: 149, col }, South) => (GridPoint::new(col + 100, 49), West), // g
        (GridPoint { row, col: 49 }, East) => (GridPoint::new(149, row - 100), North), // g
        _ => panic!(
            "unrecognized point: {} orientation: {:?}",
            point, orientation
        ),
    }
}

fn simluate<
    I: Iterator<Item = Instruction>,
    F: Fn(GridPoint<usize>, Direction) -> (GridPoint<usize>, Direction),
>(
    grid: &Grid<char>,
    instructions: I,
    wrap: F,
) -> u32 {
    let mut curr: GridPoint<usize> = GridPoint::new(0, 0)
        .traverse_by(EAST, 0, grid.rows(), 0, grid.cols())
        .find(|p| grid[*p] != ' ')
        .unwrap();
    let mut orientation: Direction = Direction::East;
    for instr in instructions {
        match instr {
            Instruction::Left => orientation = orientation.rotate_left(),
            Instruction::Right => orientation = orientation.rotate_right(),
            Instruction::Move(n) => {
                for _ in 0..n {
                    let (next, next_orientation) = curr
                        .add_checked(orientation.into(), &0, &grid.rows(), &0, &grid.cols())
                        .filter(|p| grid[*p] != ' ')
                        .map(|p| (p, orientation))
                        .unwrap_or_else(|| wrap(curr, orientation));
                    if grid[next] == '#' {
                        break;
                    } else {
                        curr = next;
                        orientation = next_orientation;
                    }
                }
            }
        }
    }
    (1 + curr.row()) as u32 * 1000 + (1 + curr.col()) as u32 * 4 + score_orientation(orientation)
}

#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let (grid, instructions) = parse!(input);
    simluate(&grid, instructions, |point, orientation| {
        wrap(&grid, point, orientation.into(), grid.rows(), grid.cols())
    })
}

#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let (grid, instructions) = parse!(input);
    simluate(&grid, instructions, |point, orientation| {
        wrap_cube( point, orientation.into())
    })
}

/*
#[allow(dead_code)]
pub fn part1(input: &str) -> u32 {
    let (grid, instructions) = parse!(input);
    let mut curr: GridPoint<usize> = GridPoint::new(0, 0)
        .traverse_by(EAST, 0, grid.rows(), 0, grid.cols())
        .find(|p| grid[*p] != ' ')
        .unwrap();
    let mut orientation: Direction = Direction::East;
    for instr in instructions {
        match instr {
            Instruction::Left => orientation = orientation.rotate_left(),
            Instruction::Right => orientation = orientation.rotate_right(),
            Instruction::Move(n) => {
                for _ in 0..n {
                    let next = curr
                        .add_checked(orientation.into(), &0, &grid.rows(), &0, &grid.cols())
                        .filter(|p| grid[*p] != ' ')
                        .unwrap_or_else(|| {
                            wrap(&grid, curr, orientation.into(), grid.rows(), grid.cols())
                        });
                    if grid[next] == '#' {
                        break;
                    } else {
                        curr = next;
                    }
                }
            }
        }
    }
    (1 + curr.row()) as u32 * 1000 + (1 + curr.col()) as u32 * 4 + score_orientation(orientation)
}
*/

/*
#[allow(dead_code)]
pub fn part2(input: &str) -> u32 {
    let (grid, instructions) = parse!(input);
    let mut curr: GridPoint<usize> = GridPoint::new(0, 0)
        .traverse_by(EAST, 0, grid.rows(), 0, grid.cols())
        .find(|p| grid[*p] != ' ')
        .unwrap();
    let mut orientation = EAST;
    for instr in instructions {
        match instr {
            Instruction::Left => orientation = rotate_left(orientation),
            Instruction::Right => orientation = rotate_right(orientation),
            Instruction::Move(n) => {
                for _ in 0..n {
                    let (next, next_orientation) = curr
                        .add_checked(orientation, &0, &grid.rows(), &0, &grid.cols())
                        .filter(|p| grid[*p] != ' ')
                        .map(|p| (p, orientation))
                        .unwrap_or_else(|| wrap_cube(curr, orientation));
                    if grid[next] == '#' {
                        break;
                    } else {
                        curr = next;
                        orientation = next_orientation;
                    }
                }
            }
        }
    }
    (1 + curr.row()) as u32 * 1000 + (1 + curr.col()) as u32 * 4 + score_orientation(orientation)
}
*/
#[test]
fn part1_test() {
    let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
    assert_eq!(part1(input), 6032);
}
