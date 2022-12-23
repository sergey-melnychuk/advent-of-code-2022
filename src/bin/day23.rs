use std::collections::HashMap;

use advent_of_code_2022::{lines, Cell, Face, Grid};

fn main() {
    let mut grid = Grid::parse(&lines(), (0, 0));

    let mut step: usize = 0;
    for _ in 0..10 {
        let fs = faces(step);
        round(&mut grid, &fs);
        step += 1;
    }

    let part1 = {
        let mut empty: usize = 0;
        let (min, max) = grid.bound_pin();
        for row in min.row..=max.row {
            for col in min.col..=max.col {
                let cell = Cell::of(row, col);
                if !grid.pins.contains(&cell) {
                    empty += 1;
                }
            }
        }
        empty
    };
    println!("{}", part1);

    let part2 = loop {
        let fs = faces(step);
        let n = round(&mut grid, &fs);
        step += 1;
        if n == 0 {
            break step;
        }
    };
    println!("{}", part2);
}

const FACES: [Face; 4] = [Face::North, Face::South, Face::West, Face::East];

fn faces(offset: usize) -> Vec<Face> {
    let mut ret = Vec::with_capacity(4);
    for i in 0..FACES.len() {
        let face = FACES[(offset + i) % FACES.len()];
        ret.push(face);
    }
    ret
}

fn can_move(cell: &Cell, face: &Face, grid: &Grid) -> bool {
    cell.next3(face).iter().all(|next| grid.is_dot(next))
}

fn can_skip(cell: &Cell, grid: &Grid) -> bool {
    cell.adj8().iter().all(|next| grid.is_dot(next))
}

fn round(grid: &mut Grid, faces: &[Face]) -> usize {
    let mut steps: HashMap<Cell, Vec<Cell>> = HashMap::new();
    grid.pins
        .iter()
        .filter(|cell| !can_skip(cell, grid))
        .for_each(|cell| {
            faces
                .iter()
                .filter(|face| can_move(cell, face, grid))
                .take(1)
                .map(|face| cell.next(face))
                .for_each(|next| steps.entry(next).or_default().push(*cell))
        });

    let ret = steps.len();

    steps
        .into_iter()
        .filter(|(_, from)| from.len() == 1)
        .map(|(next, from)| (next, from[0]))
        .for_each(|(next, from)| {
            grid.pins.remove(&from);
            grid.dots.remove(&next);

            grid.dots.insert(from);
            grid.pins.insert(next);
        });

    ret
}
