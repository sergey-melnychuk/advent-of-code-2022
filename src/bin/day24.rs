use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use advent_of_code_2022::{lines, Cell, Grid};

fn main() {
    let t = Instant::now();
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
    println!(
        "{} ({}) [{} ms]",
        part1,
        part1 == 286,
        t.elapsed().as_millis()
    );

    // let one = bfs(&grid, &blizzards, part1, goal, at, &min, &max);
    // let two = bfs(&grid, &blizzards, one, at, goal, &min, &max);
    // println!("{} ({})", two, two == 820);

    // Try making it work fast now (<100ms)?
    // https://www.reddit.com/r/adventofcode/comments/zu28ij/2022_day_24_solutions/
    /*

    time cargo run --release --bin day24 < txt/day24.txt
    cargo flamegraph --bin day24 < txt/day24.txt
    python -m http.server
    http://192.168.1.103:8000/flamegraph.svg

    */
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
    let mut queue: VecDeque<(usize, Cell)> = VecDeque::new();
    queue.push_back((time, *from));

    let mut seen: HashSet<(usize, Cell)> = HashSet::new();
    seen.insert((time, *from));

    while let Some((time, cell)) = queue.pop_front() {
        // println!("time: {} (queue: {})", time, queue.len());
        // println!("{}\n", grid.dump_with_extra(as_extra(min, max, blizzards, time, &cell)));

        if goal == &cell {
            return time;
        }

        let time = time + 1;

        let vec = rounds_vec(min, max, blizzards, time);

        for next in cell.adj4() {
            if seen.contains(&(time, next)) {
                continue;
            }
            if !next.fits(min, max) {
                continue;
            }
            if hits(&next, &vec) {
                continue;
            }
            if grid.pins.contains(&next) {
                continue;
            }

            queue.push_back((time, next));
            seen.insert((time, next));
        }

        if !hits(&cell, &vec) {
            queue.push_back((time, cell));
            seen.insert((time, cell));
        }
    }

    0
}

fn wrap(offset: i32, period: i32, span: i32) -> i32 {
    if offset > 0 {
        (offset - 1 + span) % period + 1
    } else {
        period - (offset + period + span) % period
    }
}

// hot spot: .for_each takes ~90% of time
fn rounds_vec(min: &Cell, max: &Cell, blizzards: &[(Cell, char)], time: usize) -> Vec<Vec<u8>> {
    let time = time as i32;
    let rows = max.row - min.row + 1 - 2;
    let cols = max.col - min.col + 1 - 2;

    let mut ret = vec![vec![0_u8; cols as usize]; rows as usize];

    blizzards
        .iter()
        .map(|(cell, c)| match c {
            '^' => Cell::of(wrap(-cell.row, rows, time), cell.col),
            'v' => Cell::of(wrap(cell.row, rows, time), cell.col),
            '<' => Cell::of(cell.row, wrap(-cell.col, cols, time)),
            '>' => Cell::of(cell.row, wrap(cell.col, cols, time)),
            _ => panic!("fuck off already!"),
        })
        .for_each(|cell| {
            let row = cell.row as usize - 1;
            let col = cell.col as usize - 1;
            ret[row][col] = 1;
        });

    ret
}

fn hits(cell: &Cell, vec: &[Vec<u8>]) -> bool {
    if cell.row == 0 || cell.col == 0 {
        return false;
    }
    let row = cell.row as usize - 1;
    let col = cell.col as usize - 1;
    if row > vec.len() - 1 || col > vec[0].len() - 1 {
        return false;
    }
    vec[row][col] > 0
}
