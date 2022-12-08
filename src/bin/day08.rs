use std::collections::HashSet;

use advent_of_code_2022::lines;

fn main() {
    let grid = Grid::from(lines());
    println!("{}", part1(&grid));
    println!("{}", part2(&grid));
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<Vec<usize>>,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Cell {
    row: usize,
    col: usize,
}

impl Cell {
    fn of(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Grid {
    fn from(lines: Vec<String>) -> Self {
        let cells: Vec<Vec<usize>> = lines
            .into_iter()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|n| n.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        Self {
            rows: cells.len(),
            cols: cells[0].len(),
            cells,
        }
    }

    fn at(&self, cell: &Cell) -> usize {
        self.cells[cell.row][cell.col]
    }

    fn row(&self, row: usize) -> Vec<(Cell, usize)> {
        (0..self.cols)
            .into_iter()
            .map(|col| {
                let cell = Cell::of(row, col);
                let val = self.at(&cell);
                (cell, val)
            })
            .collect()
    }

    fn col(&self, col: usize) -> Vec<(Cell, usize)> {
        (0..self.rows)
            .into_iter()
            .map(|row| {
                let cell = Cell::of(row, col);
                let val = self.at(&cell);
                (cell, val)
            })
            .collect()
    }

    fn score(&self, at: &Cell) -> usize {
        let row = self
            .row(at.row)
            .into_iter()
            .map(|(_, val)| val)
            .collect::<Vec<_>>();
        let col = self
            .col(at.col)
            .into_iter()
            .map(|(_, val)| val)
            .collect::<Vec<_>>();
        let val = self.at(at);

        let mut n: usize = 0;
        for r in (0..at.row).rev() {
            n += 1;
            if col[r] >= val {
                break;
            }
        }

        let mut s: usize = 0;
        for x in col.iter().skip(at.row + 1) {
            s += 1;
            if x >= &val {
                break;
            }
        }

        let mut e: usize = 0;
        for c in (0..at.col).rev() {
            e += 1;
            if row[c] >= val {
                break;
            }
        }

        let mut w: usize = 0;
        for x in row.iter().skip(at.col + 1) {
            w += 1;
            if x >= &val {
                break;
            }
        }

        n * s * e * w
    }
}

fn part1(grid: &Grid) -> usize {
    let mut seen: HashSet<Cell> = HashSet::new();

    grid.row(0).into_iter().for_each(|(cell, _)| {
        seen.insert(cell);
    });
    grid.col(0).into_iter().for_each(|(cell, _)| {
        seen.insert(cell);
    });
    grid.row(grid.rows - 1).into_iter().for_each(|(cell, _)| {
        seen.insert(cell);
    });
    grid.col(grid.cols - 1).into_iter().for_each(|(cell, _)| {
        seen.insert(cell);
    });

    for row in 0..grid.rows {
        let mut max = grid.at(&Cell::of(row, 0));
        for (cell, val) in grid.row(row).into_iter().skip(1) {
            if val > max {
                seen.insert(cell);
                max = val;
            }
        }

        let mut max = grid.at(&Cell::of(row, grid.cols - 1));
        for (cell, val) in grid.row(row).into_iter().rev().skip(1) {
            if val > max {
                seen.insert(cell);
                max = val;
            }
        }
    }

    for col in 0..grid.cols {
        let mut max = grid.at(&Cell::of(0, col));
        for (cell, val) in grid.col(col).into_iter().skip(1) {
            if val > max {
                seen.insert(cell);
                max = val;
            }
        }

        let mut max = grid.at(&Cell::of(grid.rows - 1, col));
        for (cell, val) in grid.col(col).into_iter().rev().skip(1) {
            if val > max {
                seen.insert(cell);
                max = val;
            }
        }
    }

    seen.len()
}

fn part2(grid: &Grid) -> usize {
    let mut max = 0;
    for row in 1..grid.rows - 1 {
        for col in 1..grid.cols - 1 {
            let cell = Cell::of(row, col);
            let score = grid.score(&cell);
            if score > max {
                max = score;
            }
        }
    }
    max
}
