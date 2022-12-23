use std::collections::HashSet;

use advent_of_code_2022::lines;

fn main() {
    let (grid, path) = parse(lines());

    let mut chip = Chip::new(&grid, grid.start(), Face::East);
    for step in path {
        chip.act(step);
    }

    let part1 = chip.cell.row * 1000 + chip.cell.col * 4 + chip.face.score();
    println!("{}", part1);
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
            self.face = self.face.turn(c);
            // println!("\tcell: {:?}, face: {:?}", self.cell, self.face);
        }

        if let Step::Move(mut n) = step {
            while n > 0 {
                n -= 1;
                if let Some(next) = self.grid.next(self.cell, self.face) {
                    self.cell = next;
                    self.path.push(next);
                    // println!("\tcell: {:?}, face: {:?}", self.cell, self.face);
                } else {
                    break;
                }
            }
        }
    }

    #[allow(dead_code)]
    fn dump(&self) -> String {
        let rows = self
            .grid
            .cells
            .iter()
            .chain(self.grid.walls.iter())
            .map(|cell| cell.row)
            .max()
            .unwrap();
        let cols = self
            .grid
            .cells
            .iter()
            .chain(self.grid.walls.iter())
            .map(|cell| cell.col)
            .max()
            .unwrap();

        let path = self.path.iter().cloned().collect::<HashSet<_>>();
        let last = self.path.last().unwrap();
        (1..=rows)
            .into_iter()
            .map(|row| {
                (1..=cols)
                    .into_iter()
                    .map(|col| {
                        let cell = Cell::of(row, col);
                        if path.contains(&cell) {
                            if &cell == last {
                                '0'
                            } else {
                                'X'
                            }
                        } else if self.grid.cells.contains(&cell) {
                            '.'
                        } else if self.grid.walls.contains(&cell) {
                            '#'
                        } else {
                            ' '
                        }
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect::<String>()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cell {
    row: usize,
    col: usize,
}

impl Cell {
    fn of(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    cells: HashSet<Cell>,
    walls: HashSet<Cell>,
}

impl Grid {
    fn parse(lines: &[String]) -> Self {
        Self {
            cells: list(lines, '.').collect(),
            walls: list(lines, '#').collect(),
        }
    }

    fn start(&self) -> Cell {
        self.cells.iter().min().cloned().unwrap()
    }

    fn next(&self, cell: Cell, face: Face) -> Option<Cell> {
        match face {
            Face::North => {
                let min = self
                    .cells
                    .iter()
                    .chain(self.walls.iter())
                    .filter(|c| c.col == cell.col)
                    .map(|c| c.row)
                    .min()
                    .unwrap();
                let next = if cell.row > min {
                    Cell::of(cell.row - 1, cell.col)
                } else {
                    let max = self
                        .cells
                        .iter()
                        .chain(self.walls.iter())
                        .filter(|c| c.col == cell.col)
                        .map(|c| c.row)
                        .max()
                        .unwrap();
                    Cell::of(max, cell.col)
                };

                if self.walls.contains(&next) {
                    // println!("\t\twall");
                    return None;
                }
                if self.cells.contains(&next) {
                    // println!("\t\tok");
                    Some(next)
                } else {
                    // println!("\t\tempty");
                    None
                }
            }
            Face::South => {
                let max = self
                    .cells
                    .iter()
                    .chain(self.walls.iter())
                    .filter(|c| c.col == cell.col)
                    .map(|c| c.row)
                    .max()
                    .unwrap();

                let next = if cell.row < max {
                    Cell::of(cell.row + 1, cell.col)
                } else {
                    let min = self
                        .cells
                        .iter()
                        .chain(self.walls.iter())
                        .filter(|c| c.col == cell.col)
                        .map(|c| c.row)
                        .min()
                        .unwrap();
                    Cell::of(min, cell.col)
                };
                // println!("\t\tnext: {:?}", next);

                if self.walls.contains(&next) {
                    // println!("\t\twall");
                    return None;
                }
                if self.cells.contains(&next) {
                    // println!("\t\tok");
                    Some(next)
                } else {
                    // println!("\t\tempty");
                    None
                }
            }
            Face::West => {
                let min = self
                    .cells
                    .iter()
                    .chain(self.walls.iter())
                    .filter(|c| c.row == cell.row)
                    .map(|c| c.col)
                    .min()
                    .unwrap();
                let next = if cell.col > min {
                    Cell::of(cell.row, cell.col - 1)
                } else {
                    let max = self
                        .cells
                        .iter()
                        .chain(self.walls.iter())
                        .filter(|c| c.row == cell.row)
                        .map(|c| c.col)
                        .max()
                        .unwrap();
                    Cell::of(cell.row, max)
                };
                // println!("\t\tnext: {:?}", next);

                if self.walls.contains(&next) {
                    // println!("\t\twall");
                    return None;
                }
                if self.cells.contains(&next) {
                    // println!("\t\tok");
                    Some(next)
                } else {
                    // println!("\t\tempty");
                    None
                }
            }
            Face::East => {
                let max = self
                    .cells
                    .iter()
                    .chain(self.walls.iter())
                    .filter(|c| c.row == cell.row)
                    .map(|c| c.col)
                    .max()
                    .unwrap();
                let next = if cell.col < max {
                    Cell::of(cell.row, cell.col + 1)
                } else {
                    let min = self
                        .cells
                        .iter()
                        .chain(self.walls.iter())
                        .filter(|c| c.row == cell.row)
                        .map(|c| c.col)
                        .min()
                        .unwrap();
                    Cell::of(cell.row, min)
                };
                // println!("\tnext: {:?}", next);

                if self.walls.contains(&next) {
                    // println!("\t\twall");
                    return None;
                }
                if self.cells.contains(&next) {
                    // println!("\t\tok");
                    Some(next)
                } else {
                    // println!("\t\tempty");
                    None
                }
            }
        }
    }
}

fn list(lines: &[String], chr: char) -> impl Iterator<Item = Cell> + '_ {
    lines.iter().enumerate().flat_map(move |(row, line)| {
        line.chars()
            .enumerate()
            .filter(move |(_, c)| *c == chr)
            .map(move |(col, _)| Cell::of(row + 1, col + 1))
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Move(usize),
    Turn(char),
}

#[derive(Debug, Clone, Copy)]
enum Face {
    North,
    South,
    East,
    West,
}

impl Face {
    fn turn(self, c: char) -> Self {
        match (self, c) {
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

    fn score(self) -> usize {
        match self {
            Face::North => 3,
            Face::South => 1,
            Face::East => 0,
            Face::West => 2,
        }
    }
}

fn parse(lines: Vec<String>) -> (Grid, Vec<Step>) {
    let mut it = lines.split(|line| line.is_empty());
    let grid = Grid::parse(it.next().unwrap());
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
