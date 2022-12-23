use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    time::Instant,
};

use advent_of_code_2022::lines;

const PIECES: &str = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
";

fn main() {
    let at = Instant::now();
    let steam = &lines()[0];

    let pieces = PIECES
        .lines()
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(Piece::parse)
        .collect::<Vec<_>>();

    let mut screen = Screen::new(7, steam.chars().collect());
    for i in 0..2022 {
        let piece = pieces[i % pieces.len()].clone();
        screen.piece(piece);
    }

    let part1 = screen.max();
    println!("{} ({} ms)", part1, at.elapsed().as_millis());

    let n: isize = 1000000000000;

    let at = Instant::now();
    let mut seen: HashMap<State, (isize, isize)> = HashMap::new();
    let mut screen = Screen::new(7, steam.chars().collect());
    let mut m: isize = 0;
    let (_, (n2, max2), (n1, max1), _, mut screen) = loop {
        let piece = (m as usize) % pieces.len();
        let steam = screen.counter % steam.len();
        screen.piece(pieces[piece].clone());
        let (offset, head) = screen.head();
        let state = State { head, piece, steam };
        m += 1;
        if seen.contains_key(&state) {
            break (
                state.clone(),
                (m, screen.max() as isize),
                seen[&state],
                offset,
                screen,
            );
        }
        seen.insert(state.clone(), (m, screen.max() as isize));
    };

    let full_cycles = (n - n1) / (n2 - n1);
    let remaining_steps = (n - n1) % (n2 - n1);

    for i in 0..remaining_steps {
        let piece = (n1 as usize + i as usize) % pieces.len();
        screen.piece(pieces[piece].clone());
    }
    let part2 = max1 + full_cycles * (max2 - max1) + (screen.max() as isize) - max2;
    println!("{} ({} ms)", part2, at.elapsed().as_millis());
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Dot {
    x: isize,
    y: isize,
}

impl Dot {
    fn of(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl Add<Dot> for Dot {
    type Output = Dot;

    fn add(self, that: Dot) -> Self::Output {
        Dot::of(self.x + that.x, self.y + that.y)
    }
}

#[derive(Debug, Clone)]
struct Piece {
    height: isize,
    dots: Vec<Dot>,
}

impl Piece {
    fn parse(lines: &[&str]) -> Self {
        let dots: Vec<Dot> = lines
            .iter()
            .enumerate()
            .rev()
            .flat_map(|(y, row)| {
                row.chars()
                    .into_iter()
                    .enumerate()
                    .filter(|(_, c)| c == &'#')
                    .map(move |(x, _)| Dot::of(x as isize, -(y as isize)))
            })
            .collect();

        let min = dots
            .iter()
            .cloned()
            .reduce(|acc, dot| Dot::of(acc.x.min(dot.x), acc.y.min(dot.y)))
            .unwrap();
        let max = dots
            .iter()
            .cloned()
            .reduce(|acc, dot| Dot::of(acc.x.max(dot.x), acc.y.max(dot.y)))
            .unwrap();

        Self {
            // width: max.x - min.x + 1,
            height: max.y - min.y + 1,
            dots,
        }
    }

    fn dots(&self) -> Vec<Dot> {
        self.dots.clone()
    }

    fn add(&mut self, one: &Dot) {
        self.dots.iter_mut().for_each(|dot| *dot = *dot + *one);
    }

    fn lshift(&mut self) {
        let one = Dot::of(-1, 0);
        self.add(&one);
    }

    fn rshift(&mut self) {
        let one = Dot::of(1, 0);
        self.add(&one);
    }

    fn down(&mut self) {
        let one = Dot::of(0, -1);
        self.add(&one);
    }
}

struct Screen {
    len: usize,
    steam: Vec<char>,
    counter: usize,
    cells: HashSet<Dot>,
}

impl Screen {
    fn new(len: usize, steam: Vec<char>) -> Self {
        Self {
            len,
            steam,
            counter: 0,
            cells: HashSet::new(),
        }
    }

    fn max(&self) -> usize {
        self.cells.iter().map(|dot| dot.y).max().unwrap_or_default() as usize
    }

    fn blow(&mut self) -> char {
        let c = self.steam[self.counter % self.steam.len()];
        self.counter += 1;
        c
    }

    fn fits(&self, piece: &Piece) -> bool {
        piece.dots.iter().all(|dot| {
            dot.x >= 0 && dot.x < self.len as isize && dot.y > 0 && !self.cells.contains(dot)
        })
    }

    fn piece(&mut self, mut piece: Piece) {
        let at: Dot = Dot::of(2, self.max() as isize + piece.height + 3);
        piece.add(&at);

        let rest = loop {
            let wind = self.blow();
            if wind == '<' {
                let mut next = piece.clone();
                next.lshift();
                if self.fits(&next) {
                    piece = next;
                }
            } else if wind == '>' {
                let mut next = piece.clone();
                next.rshift();
                if self.fits(&next) {
                    piece = next;
                }
            }

            let mut next = piece.clone();
            next.down();
            if self.fits(&next) {
                piece = next;
            } else {
                break piece;
            }
        };

        for dot in rest.dots() {
            self.cells.insert(dot);
        }
    }

    fn column(&self, idx: usize) -> impl Iterator<Item = isize> + '_ {
        self.cells
            .iter()
            .filter(move |dot| dot.x == idx as isize)
            .map(|dot| dot.y)
    }

    fn head(&mut self) -> (isize, [isize; 7]) {
        let mut head: [isize; 7] = Default::default();

        let min = (0..self.len)
            .into_iter()
            .map(|idx| self.column(idx).max().unwrap_or_default())
            .min()
            .unwrap_or_default();

        (0..self.len)
            .into_iter()
            .map(|idx| (idx, self.column(idx).max().unwrap_or_default()))
            .for_each(|(idx, max)| head[idx] = max - min);

        (min, head)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    head: [isize; 7],
    piece: usize,
    steam: usize,
}
