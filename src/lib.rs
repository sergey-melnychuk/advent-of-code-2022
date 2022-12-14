use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
    ops::Add,
};

pub fn lines() -> Vec<String> {
    let stdin = io::stdin();
    stdin.lock().lines().map(|line| line.unwrap()).collect()
}

#[derive(Debug, Clone, Copy)]
pub enum Face {
    North,
    South,
    East,
    West,
}

impl Face {
    pub fn as_cell(&self) -> Cell {
        match self {
            Face::North => Cell::of(-1, 0),
            Face::South => Cell::of(1, 0),
            Face::West => Cell::of(0, -1),
            Face::East => Cell::of(0, 1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Cell {
    pub row: i32,
    pub col: i32,
}

impl Cell {
    pub fn of(row: i32, col: i32) -> Self {
        Self { row, col }
    }

    pub fn next(&self, face: &Face) -> Self {
        *self + face.as_cell()
    }

    pub fn next3(&self, face: &Face) -> [Self; 3] {
        let mid = self.next(face);
        match face {
            Face::South | Face::North => [mid.next(&Face::East), mid, mid.next(&Face::West)],
            Face::East | Face::West => [mid.next(&Face::North), mid, mid.next(&Face::South)],
        }
    }

    pub fn adj4(&self) -> Vec<Self> {
        vec![
            self.next(&Face::North),
            self.next(&Face::East),
            self.next(&Face::South),
            self.next(&Face::West),
        ]
    }

    pub fn adj8(&self) -> Vec<Self> {
        let mut ret = Vec::with_capacity(8);
        for r in -1..=1 {
            for c in -1..=1 {
                if r == 0 && c == 0 {
                    continue;
                }
                ret.push(Cell::of(self.row + r, self.col + c));
            }
        }
        ret
    }

    pub fn lo(&self, that: &Cell) -> Cell {
        Cell::of(self.row.min(that.row), self.col.min(that.col))
    }

    pub fn hi(&self, that: &Cell) -> Cell {
        Cell::of(self.row.max(that.row), self.col.max(that.col))
    }

    pub fn fits(&self, min: &Cell, max: &Cell) -> bool {
        self.row >= min.row && self.col >= min.col && self.row <= max.row && self.col <= max.col
    }
}

impl Add for Cell {
    type Output = Cell;

    fn add(self, rhs: Self) -> Self::Output {
        Self::of(self.row + rhs.row, self.col + rhs.col)
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub dots: HashSet<Cell>,
    pub pins: HashSet<Cell>,
}

impl Grid {
    pub fn parse(lines: &[String], offset: (i32, i32)) -> Self {
        Self {
            dots: list(lines, '.', offset).collect(),
            pins: list(lines, '#', offset).collect(),
        }
    }

    pub fn parse_with_extra(lines: &[String], offset: (i32, i32)) -> (Self, Vec<(Cell, char)>) {
        let parsed = Self::parse(lines, offset);
        let extra = extra(lines, &|c| c == '#' || c == '.', offset);
        (parsed, extra.collect())
    }

    pub fn is_dot(&self, cell: &Cell) -> bool {
        !self.pins.contains(cell)
    }

    pub fn bound_all(&self) -> (Cell, Cell) {
        let min = self
            .pins
            .iter()
            .chain(self.dots.iter())
            .copied()
            .reduce(|a, b| a.lo(&b))
            .unwrap();

        let max = self
            .pins
            .iter()
            .chain(self.dots.iter())
            .copied()
            .reduce(|a, b| a.hi(&b))
            .unwrap();

        (min, max)
    }

    pub fn bound_pin(&self) -> (Cell, Cell) {
        let min = self.pins.iter().copied().reduce(|a, b| a.lo(&b)).unwrap();

        let max = self.pins.iter().copied().reduce(|a, b| a.hi(&b)).unwrap();

        (min, max)
    }

    pub fn dump(&self) -> String {
        self.dump_with_extra(Default::default())
    }

    pub fn dump_with_extra(&self, extra: HashMap<Cell, char>) -> String {
        let (min, max) = self.bound_all();

        (min.row..=max.row)
            .into_iter()
            .map(|row| {
                (min.col..=max.col)
                    .into_iter()
                    .map(|col| {
                        let cell = Cell::of(row, col);
                        if extra.contains_key(&cell) {
                            extra[&cell]
                        } else if self.dots.contains(&cell) {
                            '.'
                        } else if self.pins.contains(&cell) {
                            '#'
                        } else {
                            ' '
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

fn list(lines: &[String], chr: char, offset: (i32, i32)) -> impl Iterator<Item = Cell> + '_ {
    lines.iter().enumerate().flat_map(move |(row, line)| {
        line.chars()
            .enumerate()
            .filter(move |(_, c)| *c == chr)
            .map(move |(col, _)| Cell::of(row as i32 + offset.0, col as i32 + offset.1))
    })
}

fn extra<'a, F: Fn(char) -> bool>(
    lines: &'a [String],
    skip: &'a F,
    offset: (i32, i32),
) -> impl Iterator<Item = (Cell, char)> + 'a {
    lines.iter().enumerate().flat_map(move |(row, line)| {
        line.chars()
            .enumerate()
            .filter(move |(_, c)| !skip(*c))
            .map(move |(col, chr)| {
                let cell = Cell::of(row as i32 + offset.0, col as i32 + offset.1);
                (cell, chr)
            })
    })
}
