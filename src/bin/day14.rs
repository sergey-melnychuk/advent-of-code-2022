use std::{collections::HashSet, env::args, iter::once};

use advent_of_code_2022::lines;

fn main() {
    let verbose = args().nth(1).map(|arg| arg == "--dump").unwrap_or_default();
    let mut grid = Grid::from_lines(lines());

    let src = Cell::at(0, 500);

    let (min, max) = grid.bounds(&src);
    while let Some(cell) = drop(&grid, src, |next| next.fits(&min, &max)) {
        grid.sand.insert(cell);
    }
    println!("{}", grid.sand.len());

    if verbose {
        println!("{}", grid.dump());
    }

    let floor = max.row + 2;
    grid.floor = Some(floor);

    while let Some(cell) = drop(&grid, src, |_| true) {
        grid.sand.insert(cell);
        if cell == src {
            break;
        }
    }
    println!("{}", grid.sand.len());

    if verbose {
        println!("{}", grid.dump());
    }
}

fn drop<F: Fn(&Cell) -> bool>(grid: &Grid, mut cell: Cell, test: F) -> Option<Cell> {
    while let Some(next) = cell.next().into_iter().find(|next| grid.is_empty(next)) {
        if !test(&next) {
            return None;
        }
        cell = next;
    }

    Some(cell)
}

struct Grid {
    rock: HashSet<Cell>,
    sand: HashSet<Cell>,
    floor: Option<isize>,
}

impl Grid {
    fn from_lines(lines: Vec<String>) -> Self {
        let rock = lines
            .into_iter()
            .flat_map(|line| {
                let dots = parse(&line);
                dots.iter()
                    .zip(dots.iter().skip(1))
                    .into_iter()
                    .flat_map(|(a, b)| unfold(a, b))
                    .collect::<Vec<_>>()
            })
            .collect();

        Self {
            rock,
            sand: HashSet::new(),
            floor: None,
        }
    }

    fn bounds(&self, src: &Cell) -> (Cell, Cell) {
        let rows = bounds(self.rock.iter().chain(once(src)).map(|cell| cell.row));
        let cols = bounds(self.rock.iter().chain(once(src)).map(|cell| cell.col));

        (Cell::at(rows.0, cols.0), Cell::at(rows.1, cols.1))
    }

    fn is_empty(&self, cell: &Cell) -> bool {
        !self
            .floor
            .as_ref()
            .map(|floor| floor == &cell.row)
            .unwrap_or_default()
            && !self.rock.contains(cell)
            && !self.sand.contains(cell)
    }

    fn dump(&self) -> String {
        let rows = bounds(
            self.rock
                .iter()
                .chain(self.sand.iter())
                .map(|cell| cell.row),
        );
        let cols = bounds(
            self.rock
                .iter()
                .chain(self.sand.iter())
                .map(|cell| cell.col),
        );

        (rows.0..=rows.1)
            .into_iter()
            .map(move |row| {
                (cols.0..cols.1)
                    .into_iter()
                    .map(move |col| {
                        let cell = Cell::at(row, col);
                        if self.rock.contains(&cell) {
                            '#'
                        } else if self.sand.contains(&cell) {
                            'o'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct Cell {
    row: isize,
    col: isize,
}

impl Cell {
    fn at(row: isize, col: isize) -> Self {
        Self { row, col }
    }

    fn next(&self) -> Vec<Cell> {
        vec![
            Cell {
                row: self.row + 1,
                col: self.col,
            },
            Cell {
                row: self.row + 1,
                col: self.col - 1,
            },
            Cell {
                row: self.row + 1,
                col: self.col + 1,
            },
        ]
    }

    fn fits(&self, min: &Cell, max: &Cell) -> bool {
        self.row >= min.row && self.row <= max.row && self.col >= min.col && self.col <= max.col
    }
}

fn parse(line: &str) -> Vec<Cell> {
    line.split(" -> ")
        .into_iter()
        .map(|cell| {
            let mut it = cell.split(',');
            let col = it.next().unwrap().parse().unwrap();
            let row = it.next().unwrap().parse().unwrap();
            Cell { row, col }
        })
        .collect()
}

fn seq(a: isize, b: isize) -> impl Iterator<Item = isize> {
    let len = b - a;
    let dir = len / len.abs();
    (0..=len.abs()).into_iter().map(move |i| a + i * dir)
}

fn unfold(a: &Cell, b: &Cell) -> Vec<Cell> {
    if a.row == b.row {
        seq(a.col, b.col)
            .map(|col| Cell { row: a.row, col })
            .collect()
    } else if a.col == b.col {
        seq(a.row, b.row)
            .map(|row| Cell { row, col: a.col })
            .collect()
    } else {
        panic!("Diagonal line: {:?} -> {:?}", a, b);
    }
}

fn bounds<T: Sized + Ord + Default + Clone>(it: impl Iterator<Item = T> + Clone) -> (T, T) {
    let min = it
        .clone()
        .reduce(|acc, val| acc.min(val))
        .unwrap_or_default();
    let max = it.reduce(|acc, val| acc.max(val)).unwrap_or_default();
    (min, max)
}
