use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use advent_of_code_2022::{lines, Cell, Grid};

fn main() {
    let (grid, blizzards) = Grid::parse_with_extra(&lines(), (0, 0));

    let (min, max) = grid.bound_all();

    let at = grid
        .dots
        .iter()
        .find(|c| c.row == min.row && grid.is_dot(c))
        .unwrap();
    let goal = grid
        .dots
        .iter()
        .find(|c| c.row == max.row && grid.is_dot(c))
        .unwrap();

    let part1 = bfs(&grid, &blizzards, 0, at, goal, &min, &max);
    println!("{} ({})", part1, part1 == 286);

    let one = bfs(&grid, &blizzards, part1, goal, at, &min, &max);
    let two = bfs(&grid, &blizzards, one, at, goal, &min, &max);
    println!("{} ({})", two, two == 820);

    // Try making it work fast now (<100ms)?
    // https://www.reddit.com/r/adventofcode/comments/zu28ij/2022_day_24_solutions/
}

fn bfs(
    grid: &Grid,
    blizzards: &[(Cell, char)],
    time: usize,
    from: &Cell,
    goal: &Cell,
    min: &Cell,
    max: &Cell,
) -> usize {
    let mut queue: Vec<(Cell, usize)> = Vec::new();
    queue.push((*from, time));

    let mut seen: HashSet<(Cell, usize)> = HashSet::new();
    seen.insert((*from, time));

    while !queue.is_empty() {
        queue.sort_by_key(|(c, _)| dist(from, c) + dist(c, goal));
        let (cell, time) = queue.remove(0);

        // println!("time: {} (queue: {})", time, queue.len());
        // println!("{}\n", grid.dump_with_extra(as_extra(min, max, blizzards, time, &cell)));

        if goal == &cell {
            return time;
        }

        let time = time + 1;
        let occupied = rounds(min, max, blizzards, time);

        let steps = scan(&cell, grid, min, max, &occupied);
        for next in steps {
            if !seen.contains(&(next, time)) {
                queue.push((next, time));
                seen.insert((next, time));
            }
        }

        if !occupied.contains(&cell) {
            queue.push((cell, time));
            seen.insert((cell, time));
        }
    }

    0
}

fn dist(a: &Cell, b: &Cell) -> i64 {
    (b.row - a.row).abs() + (b.col - a.col).abs()
}

fn scan(cell: &Cell, grid: &Grid, min: &Cell, max: &Cell, occupied: &HashSet<Cell>) -> Vec<Cell> {
    cell.adj4()
        .into_iter()
        .filter(|next| next.fits(min, max))
        .filter(|next| !occupied.contains(next))
        .filter(|next| !grid.pins.contains(next))
        .collect()
}

fn wrap(offset: i64, period: i64, span: i64) -> i64 {
    if offset > 0 {
        (offset - 1 + span) % period + 1
    } else {
        period - (offset + period + span) % period
    }
}

fn rounds(min: &Cell, max: &Cell, blizzards: &[(Cell, char)], time: usize) -> HashSet<Cell> {
    let time = time as i64;
    let rows = max.row - min.row + 1 - 2;
    let cols = max.col - min.col + 1 - 2;

    blizzards
        .iter()
        .map(|(cell, c)| match c {
            '^' => Cell::of(wrap(-cell.row, rows, time), cell.col),
            'v' => Cell::of(wrap(cell.row, rows, time), cell.col),
            '<' => Cell::of(cell.row, wrap(-cell.col, cols, time)),
            '>' => Cell::of(cell.row, wrap(cell.col, cols, time)),
            _ => panic!("fuck off already!"),
        })
        .collect()
}

#[allow(dead_code)]
fn as_extra(
    min: &Cell,
    max: &Cell,
    blizzards: &[(Cell, char)],
    time: usize,
    at: &Cell,
) -> HashMap<Cell, char> {
    rounds(min, max, blizzards, time)
        .into_iter()
        .map(|cell| (cell, '*'))
        .chain(once((*at, 'E')))
        .collect()
}
