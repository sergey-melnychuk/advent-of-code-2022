use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2022::lines;

fn main() {
    let grid = Grid::from_lines(lines());

    let start = grid.find('S')[0];
    let end = grid.find('E')[0];
    println!("{}", bfs(&grid, &start)[&end]);

    let min = grid
        .find('a')
        .into_iter()
        .filter_map(|cell| {
            let step = bfs(&grid, &cell);
            step.get(&end).cloned()
        })
        .min()
        .unwrap_or_default();
    println!("{}", min);
}

fn bfs(grid: &Grid, cell: &Cell) -> HashMap<Cell, usize> {
    let mut seen: HashSet<Cell> = HashSet::new();
    let mut step: HashMap<Cell, usize> = HashMap::new();

    step.insert(*cell, 0);

    let mut queue = VecDeque::new();
    queue.push_back(*cell);

    while !queue.is_empty() {
        let cell = queue.pop_front().unwrap();
        let a = grid.cell(&cell).unwrap();

        for next in grid.adj(&cell) {
            if seen.contains(&next) {
                continue;
            }
            if let Some(b) = grid.cell(&next) {
                let d = dist(a, b);
                if d <= 1 {
                    if step.get(&next).cloned().unwrap_or(usize::MAX) > step[&cell] + 1 {
                        step.insert(next, step[&cell] + 1);
                    }

                    if b == 'E' {
                        return step;
                    }

                    seen.insert(next);
                    queue.push_back(next);
                }
            }
        }
    }

    step
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Cell {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn from_lines(lines: Vec<String>) -> Self {
        let rows = lines.len();
        let cols = lines[0].len();
        let grid = lines
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();
        Self { rows, cols, grid }
    }

    fn find(&self, c: char) -> Vec<Cell> {
        let mut cells = Vec::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = Cell { row, col };
                if self.cell(&cell) == Some(c) {
                    cells.push(cell);
                }
            }
        }
        cells
    }

    fn cell(&self, cell: &Cell) -> Option<char> {
        if cell.row >= self.rows || cell.col >= self.cols {
            return None;
        }
        self.grid
            .get(cell.row)
            .and_then(|row| row.get(cell.col))
            .cloned()
    }

    fn adj(&self, cell: &Cell) -> Vec<Cell> {
        let mut adj = Vec::with_capacity(4);
        if cell.row > 0 {
            adj.push(Cell {
                row: cell.row - 1,
                col: cell.col,
            });
        }
        if cell.col > 0 {
            adj.push(Cell {
                row: cell.row,
                col: cell.col - 1,
            });
        }
        if cell.row < self.rows - 1 {
            adj.push(Cell {
                row: cell.row + 1,
                col: cell.col,
            });
        }
        if cell.col < self.cols - 1 {
            adj.push(Cell {
                row: cell.row,
                col: cell.col + 1,
            });
        }
        adj
    }
}

fn dist(a: char, b: char) -> isize {
    let a = if a == 'S' { 'a' } else { a };
    let b = if b == 'S' { 'a' } else { b };

    let a = if a == 'E' { 'z' } else { a };
    let b = if b == 'E' { 'z' } else { b };

    let a = a as isize;
    let b = b as isize;
    b - a
}
