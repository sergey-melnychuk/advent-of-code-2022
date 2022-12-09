use std::{
    collections::HashSet,
    ops::{Add, Mul, Sub},
};

use advent_of_code_2022::lines;

type Num = isize;

fn main() {
    let steps = lines()
        .iter()
        .map(|line| Step::from_str(line))
        .collect::<Vec<_>>();

    let mut rope = Rope::new(1);
    for step in &steps {
        rope.step(step);
    }
    println!("{}", rope.seen.len());

    let mut rope = Rope::new(9);
    for step in &steps {
        rope.step(step);
    }
    println!("{}", rope.seen.len());
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct Cell(Num, Num);

impl Cell {
    fn one(&self) -> Self {
        let x = if self.0 == 0 {
            0
        } else {
            self.0 / self.0.abs()
        };
        let y = if self.1 == 0 {
            0
        } else {
            self.1 / self.1.abs()
        };
        Cell(x, y)
    }
}

impl Mul<Num> for Cell {
    type Output = Cell;

    fn mul(self, rhs: Num) -> Self::Output {
        Cell(self.0 * rhs, self.1 * rhs)
    }
}

impl Add<Cell> for Cell {
    type Output = Cell;

    fn add(self, rhs: Cell) -> Self::Output {
        Cell(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub<Cell> for Cell {
    type Output = Cell;

    fn sub(self, rhs: Cell) -> Self::Output {
        Cell(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug)]
struct Rope {
    head: Cell,
    tail: Vec<Cell>,
    seen: HashSet<Cell>,
}

impl Rope {
    fn new(len: usize) -> Self {
        let mut seen = HashSet::new();
        seen.insert(Default::default());
        Self {
            head: Default::default(),
            tail: vec![Default::default(); len],
            seen,
        }
    }

    fn step(&mut self, step: &Step) {
        let dir = step.dir();
        for _ in 0..step.len() {
            self.head = self.head + dir;
            for idx in 0..self.tail.len() {
                let head = if idx == 0 {
                    self.head
                } else {
                    self.tail[idx - 1]
                };
                let tail = self.tail[idx];
                if let Some(tail) = tail_step(&head, &tail) {
                    self.tail[idx] = tail;
                }
            }
            if let Some(last) = self.tail.last().cloned() {
                self.seen.insert(last);
            }
        }
    }
}

fn tail_step(head: &Cell, tail: &Cell) -> Option<Cell> {
    let dist = *head - *tail;
    if dist.0.abs() <= 1 && dist.1.abs() <= 1 {
        return None;
    }
    if (dist.0 == 0 && dist.1.abs() > 1) || (dist.1 == 0 && dist.0.abs() > 1) {
        return Some(*tail + dist.one());
    }
    if dist.0.abs() + dist.1.abs() > 2 {
        return Some(*tail + dist.one());
    }
    None
}

#[derive(Debug)]
enum Step {
    Left(Num),
    Right(Num),
    Up(Num),
    Down(Num),
}

impl Step {
    fn from_str(s: &str) -> Self {
        let mut it = s.split_ascii_whitespace();
        let dir = it.next().unwrap();
        let len: Num = it.next().unwrap().parse().unwrap();

        match dir.chars().next().unwrap() {
            'L' => Self::Left(len),
            'R' => Self::Right(len),
            'U' => Self::Up(len),
            'D' => Self::Down(len),
            _ => panic!("Unexpected direction: {}", dir),
        }
    }

    fn len(&self) -> Num {
        match self {
            Self::Left(n) => *n,
            Self::Right(n) => *n,
            Self::Up(n) => *n,
            Self::Down(n) => *n,
        }
    }

    fn dir(&self) -> Cell {
        match self {
            Self::Left(_) => Cell(-1, 0),
            Self::Right(_) => Cell(1, 0),
            Self::Up(_) => Cell(0, -1),
            Self::Down(_) => Cell(0, 1),
        }
    }
}
