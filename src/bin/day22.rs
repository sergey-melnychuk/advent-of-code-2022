use advent_of_code_2022::{lines, Cell, Face, Grid};

fn main() {
    let (grid, path) = parse(lines());

    // Check that the input data is aligned as expected.
    // If any cell is not aligned, `side` will panic.
    let ok = grid
        .dots
        .iter()
        .chain(grid.pins.iter())
        .all(|cell| side(cell) > 0);
    if !ok {
        panic!("Input is not aligned as expected");
    }

    let mut chip = Chip::new(&grid, start(&grid), Face::East);
    for step in path.iter() {
        chip.act(step, next, turn);
    }
    let part1 = chip.cell.row * 1000 + chip.cell.col * 4 + score(chip.face);
    println!("{}", part1);

    // println!("{}", grid.dump_with_extra(chip.path.into_iter().collect()));

    let mut chip = Chip::new(&grid, start(&grid), Face::East);
    for step in path.iter() {
        chip.act(step, next2, turn2);
    }
    let part2 = chip.cell.row * 1000 + chip.cell.col * 4 + score(chip.face);
    println!("{}", part2);

    // println!("{}", grid.dump_with_extra(chip.path.into_iter().collect()));
}

#[derive(Debug)]
struct Chip<'a> {
    grid: &'a Grid,
    cell: Cell,
    face: Face,
    path: Vec<Cell>,
}

impl<'a> Chip<'a> {
    fn new(grid: &'a Grid, cell: Cell, face: Face) -> Self {
        let mut path = Vec::with_capacity(1024);
        path.push(cell);
        Self {
            grid,
            cell,
            face,
            path,
        }
    }

    fn act<F, G>(&mut self, step: &Step, next: F, turn: G)
    where
        F: Fn(&Grid, Cell, Face) -> Option<(Cell, Face)>,
        G: Fn(Cell, Face, &char) -> Face,
    {
        // println!("step: {:?}", step);

        if let Step::Turn(c) = step {
            self.face = turn(self.cell, self.face, c);
            //println!("\tcell: {:?}, face: {:?}", self.cell, self.face);
        }

        if let Step::Move(mut n) = step {
            while n > 0 {
                n -= 1;
                if let Some((next, face)) = next(self.grid, self.cell, self.face) {
                    self.cell = next;
                    self.face = face;
                    self.path.push(next);
                    // println!("\tcell: {:?}, face: {:?}", self.cell, self.face);
                } else {
                    break;
                }
            }
        }
    }
}

fn start(grid: &Grid) -> Cell {
    grid.dots.iter().min().cloned().unwrap()
}

fn next(grid: &Grid, cell: Cell, face: Face) -> Option<(Cell, Face)> {
    match face {
        Face::North => {
            let min = grid
                .dots
                .iter()
                .chain(grid.pins.iter())
                .filter(|c| c.col == cell.col)
                .map(|c| c.row)
                .min()
                .unwrap();
            let next = if cell.row > min {
                Cell::of(cell.row - 1, cell.col)
            } else {
                let max = grid
                    .dots
                    .iter()
                    .chain(grid.pins.iter())
                    .filter(|c| c.col == cell.col)
                    .map(|c| c.row)
                    .max()
                    .unwrap();
                Cell::of(max, cell.col)
            };

            if grid.pins.contains(&next) {
                // println!("\t\twall");
                return None;
            }
            if grid.dots.contains(&next) {
                // println!("\t\tok");
                Some((next, face))
            } else {
                // println!("\t\tempty");
                None
            }
        }
        Face::South => {
            let max = grid
                .dots
                .iter()
                .chain(grid.pins.iter())
                .filter(|c| c.col == cell.col)
                .map(|c| c.row)
                .max()
                .unwrap();

            let next = if cell.row < max {
                Cell::of(cell.row + 1, cell.col)
            } else {
                let min = grid
                    .dots
                    .iter()
                    .chain(grid.pins.iter())
                    .filter(|c| c.col == cell.col)
                    .map(|c| c.row)
                    .min()
                    .unwrap();
                Cell::of(min, cell.col)
            };
            // println!("\t\tnext: {:?}", next);

            if grid.pins.contains(&next) {
                // println!("\t\twall");
                return None;
            }
            if grid.dots.contains(&next) {
                // println!("\t\tok");
                Some((next, face))
            } else {
                // println!("\t\tempty");
                None
            }
        }
        Face::West => {
            let min = grid
                .dots
                .iter()
                .chain(grid.pins.iter())
                .filter(|c| c.row == cell.row)
                .map(|c| c.col)
                .min()
                .unwrap();
            let next = if cell.col > min {
                Cell::of(cell.row, cell.col - 1)
            } else {
                let max = grid
                    .dots
                    .iter()
                    .chain(grid.pins.iter())
                    .filter(|c| c.row == cell.row)
                    .map(|c| c.col)
                    .max()
                    .unwrap();
                Cell::of(cell.row, max)
            };
            // println!("\t\tnext: {:?}", next);

            if grid.pins.contains(&next) {
                // println!("\t\twall");
                return None;
            }
            if grid.dots.contains(&next) {
                // println!("\t\tok");
                Some((next, face))
            } else {
                // println!("\t\tempty");
                None
            }
        }
        Face::East => {
            let max = grid
                .dots
                .iter()
                .chain(grid.pins.iter())
                .filter(|c| c.row == cell.row)
                .map(|c| c.col)
                .max()
                .unwrap();
            let next = if cell.col < max {
                Cell::of(cell.row, cell.col + 1)
            } else {
                let min = grid
                    .dots
                    .iter()
                    .chain(grid.pins.iter())
                    .filter(|c| c.row == cell.row)
                    .map(|c| c.col)
                    .min()
                    .unwrap();
                Cell::of(cell.row, min)
            };
            // println!("\tnext: {:?}", next);

            if grid.pins.contains(&next) {
                // println!("\t\twall");
                return None;
            }
            if grid.dots.contains(&next) {
                // println!("\t\tok");
                Some((next, face))
            } else {
                // println!("\t\tempty");
                None
            }
        }
    }
}

fn turn(_: Cell, face: Face, chr: &char) -> Face {
    match (face, chr) {
        (Face::North, 'L') => Face::West,
        (Face::South, 'L') => Face::East,
        (Face::East, 'L') => Face::North,
        (Face::West, 'L') => Face::South,

        (Face::North, 'R') => Face::East,
        (Face::South, 'R') => Face::West,
        (Face::East, 'R') => Face::South,
        (Face::West, 'R') => Face::North,

        _ => panic!("Unsupported turn: {}", chr),
    }
}

fn next2(grid: &Grid, cell: Cell, face: Face) -> Option<(Cell, Face)> {
    match face {
        Face::North => {
            let min = grid
                .dots
                .iter()
                .chain(grid.pins.iter())
                .filter(|c| c.col == cell.col)
                .min()
                .unwrap();
            let (next, face) = if cell.row > min.row {
                (Cell::of(cell.row - 1, cell.col), face)
            } else {
                match side(&cell) {
                    2 => {
                        let next = Cell::of(cell.col + SIDE * 2, 1);
                        let face = Face::East;
                        (next, face)
                    }
                    3 => {
                        let next = Cell::of(SIDE * 4, cell.col - SIDE * 2);
                        let face = Face::North;
                        (next, face)
                    }
                    5 => {
                        let next = Cell::of(cell.col + SIDE, SIDE + 1);
                        let face = Face::East;
                        (next, face)
                    }
                    x => panic!("Move north from {:?} (side: {})", cell, x),
                }
            };

            if grid.pins.contains(&next) {
                // println!("\t\twall");
                return None;
            }
            if grid.dots.contains(&next) {
                // println!("\t\tok");
                Some((next, face))
            } else {
                // println!("\t\tempty");
                None
            }
        }
        Face::South => {
            let max = grid
                .dots
                .iter()
                .chain(grid.pins.iter())
                .filter(|c| c.col == cell.col)
                .max()
                .unwrap();

            let (next, face) = if cell.row < max.row {
                (Cell::of(cell.row + 1, cell.col), face)
            } else {
                match side(&cell) {
                    6 => {
                        let next = Cell::of(1, cell.col + SIDE * 2);
                        let face = Face::South;
                        (next, face)
                    }
                    4 => {
                        let next = Cell::of(cell.col + SIDE * 2, SIDE);
                        let face = Face::West;
                        (next, face)
                    }
                    3 => {
                        let next = Cell::of(cell.col - SIDE, SIDE * 2);
                        let face = Face::West;
                        (next, face)
                    }
                    x => panic!("Move south from {:?} (side: {})", cell, x),
                }
            };
            // println!("\t\tnext: {:?}", next);

            if grid.pins.contains(&next) {
                // println!("\t\twall");
                return None;
            }
            if grid.dots.contains(&next) {
                // println!("\t\tok");
                Some((next, face))
            } else {
                // println!("\t\tempty");
                None
            }
        }
        Face::West => {
            let min = grid
                .dots
                .iter()
                .chain(grid.pins.iter())
                .filter(|c| c.row == cell.row)
                .map(|c| c.col)
                .min()
                .unwrap();
            let (next, face) = if cell.col > min {
                (Cell::of(cell.row, cell.col - 1), face)
            } else {
                match side(&cell) {
                    2 => {
                        let next = Cell::of(SIDE * 3 - cell.row + 1, 1);
                        let face = Face::East;
                        (next, face)
                    }
                    1 => {
                        let next = Cell::of(SIDE * 2 + 1, cell.row - SIDE);
                        let face = Face::South;
                        (next, face)
                    }
                    5 => {
                        let next = Cell::of(SIDE - (cell.row - SIDE * 2) + 1, SIDE + 1);
                        let face = Face::East;
                        (next, face)
                    }
                    6 => {
                        let next = Cell::of(1, cell.row - SIDE * 2);
                        let face = Face::South;
                        (next, face)
                    }
                    x => panic!("Move west from {:?} (side: {})", cell, x),
                }
            };
            // println!("\t\tnext: {:?}", next);

            if grid.pins.contains(&next) {
                // println!("\t\twall");
                return None;
            }
            if grid.dots.contains(&next) {
                // println!("\t\tok");
                Some((next, face))
            } else {
                // println!("\t\tempty");
                None
            }
        }
        Face::East => {
            let max = grid
                .dots
                .iter()
                .chain(grid.pins.iter())
                .filter(|c| c.row == cell.row)
                .map(|c| c.col)
                .max()
                .unwrap();
            let (next, face) = if cell.col < max {
                (Cell::of(cell.row, cell.col + 1), face)
            } else {
                match side(&cell) {
                    3 => {
                        let next = Cell::of(SIDE * 3 - cell.row + 1, SIDE * 2);
                        let face = Face::West;
                        (next, face)
                    }
                    1 => {
                        let next = Cell::of(SIDE, cell.row + SIDE);
                        let face = Face::North;
                        (next, face)
                    }
                    4 => {
                        let next = Cell::of(SIDE * 3 - cell.row + 1, SIDE * 3);
                        let face = Face::West;
                        (next, face)
                    }
                    6 => {
                        let next = Cell::of(SIDE * 3, cell.row - SIDE * 2);
                        let face = Face::North;
                        (next, face)
                    }
                    x => panic!("Move east from {:?} (side: {})", cell, x),
                }
            };
            // println!("\tnext: {:?}", next);

            if grid.pins.contains(&next) {
                // println!("\t\twall");
                return None;
            }
            if grid.dots.contains(&next) {
                // println!("\t\tok");
                Some((next, face))
            } else {
                // println!("\t\tempty");
                None
            }
        }
    }
}

fn turn2(_: Cell, face: Face, chr: &char) -> Face {
    match (face, chr) {
        (Face::North, 'L') => Face::West,
        (Face::South, 'L') => Face::East,
        (Face::East, 'L') => Face::North,
        (Face::West, 'L') => Face::South,

        (Face::North, 'R') => Face::East,
        (Face::South, 'R') => Face::West,
        (Face::East, 'R') => Face::South,
        (Face::West, 'R') => Face::North,

        _ => panic!("Unsupported turn: {}", chr),
    }
}

const SIDE: i64 = 50;

///    222333
///    222333
///    222333
///    111
///    111
///    111
/// 555444
/// 555444
/// 666
/// 666
/// 666
fn side(cell: &Cell) -> i8 {
    let (row, col) = ((cell.row - 1) / SIDE, (cell.col - 1) / SIDE);
    match (row, col) {
        (0, 1) => 2,
        (0, 2) => 3,
        (1, 1) => 1,
        (2, 0) => 5,
        (2, 1) => 4,
        (3, 0) => 6,
        _ => -1,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Move(usize),
    Turn(char),
}

fn score(f: Face) -> i64 {
    match f {
        Face::North => 3,
        Face::South => 1,
        Face::East => 0,
        Face::West => 2,
    }
}

fn parse(lines: Vec<String>) -> (Grid, Vec<Step>) {
    let mut it = lines.split(|line| line.is_empty());
    let grid = Grid::parse(it.next().unwrap(), (1, 1));
    let path = it.next().unwrap().iter().next().unwrap();
    let path = parse_path(path);

    (grid, path)
}

fn parse_path(path: &str) -> Vec<Step> {
    fn next(mut seq: Vec<char>) -> (Step, Vec<char>) {
        if seq[0].is_ascii_alphabetic() {
            let dir = seq.remove(0);
            (Step::Turn(dir), seq)
        } else {
            let mut buf = Vec::new();
            while !seq.is_empty() && seq[0].is_ascii_digit() {
                let d = seq.remove(0);
                buf.push(d);
            }
            let num = buf.into_iter().collect::<String>();
            let num: usize = num.parse().unwrap();
            (Step::Move(num), seq)
        }
    }

    let mut seq = path.chars().collect::<Vec<_>>();
    let mut path = Vec::new();
    while !seq.is_empty() {
        let (step, rem) = next(seq);
        path.push(step);
        seq = rem;
    }

    path
}

#[cfg(test)]
mod day22 {
    use super::*;

    #[test]
    fn test_parse_path() {
        let input = "10R5L5R10L4R5L5";
        assert_eq!(
            parse_path(input),
            vec![
                Step::Move(10),
                Step::Turn('R'),
                Step::Move(5),
                Step::Turn('L'),
                Step::Move(5),
                Step::Turn('R'),
                Step::Move(10),
                Step::Turn('L'),
                Step::Move(4),
                Step::Turn('R'),
                Step::Move(5),
                Step::Turn('L'),
                Step::Move(5),
            ]
        );
    }
}
