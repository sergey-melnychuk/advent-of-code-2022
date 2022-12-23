use advent_of_code_2022::{lines, Cell, Face, Grid};

fn main() {
    let (grid, path) = parse(lines());

    let mut chip = Chip::new(&grid, start(&grid), Face::East);
    for step in path {
        chip.act(step);
    }

    //println!("cell: {:?}", chip.cell);
    let part1 = chip.cell.row * 1000 + chip.cell.col * 4 + score(chip.face);
    println!("{}", part1);

    //println!("{}", grid.dump(chip.path.into_iter().collect()));
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

    fn act(&mut self, step: Step) {
        // println!("step: {:?}", step);

        if let Step::Turn(c) = step {
            self.face = turn(self.face, c);
            // println!("\tcell: {:?}, face: {:?}", self.cell, self.face);
        }

        if let Step::Move(mut n) = step {
            while n > 0 {
                n -= 1;
                if let Some(next) = next(self.grid, self.cell, self.face) {
                    self.cell = next;
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

fn next(grid: &Grid, cell: Cell, face: Face) -> Option<Cell> {
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
                Some(next)
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
                Some(next)
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
                Some(next)
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
                Some(next)
            } else {
                // println!("\t\tempty");
                None
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Move(usize),
    Turn(char),
}

fn turn(f: Face, c: char) -> Face {
    match (f, c) {
        (Face::North, 'L') => Face::West,
        (Face::South, 'L') => Face::East,
        (Face::East, 'L') => Face::North,
        (Face::West, 'L') => Face::South,

        (Face::North, 'R') => Face::East,
        (Face::South, 'R') => Face::West,
        (Face::East, 'R') => Face::South,
        (Face::West, 'R') => Face::North,

        _ => panic!("Unsupported turn: {}", c),
    }
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
