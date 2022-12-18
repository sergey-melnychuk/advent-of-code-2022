use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

use advent_of_code_2022::lines;

fn main() {
    let at = Instant::now();
    let dots = lines()
        .into_iter()
        .map(|line| Dot::parse(&line))
        .collect::<HashSet<_>>();

    let part1 = dots
        .iter()
        .map(|dot| {
            adj()
                .iter()
                .filter(|side| !dots.contains(&dot.add(side)))
                .count()
        })
        .sum::<usize>();
    println!("{} ({} ms)", part1, at.elapsed().as_millis());

    let at = Instant::now();
    let min = min(&dots).add(&Dot::of(-1, -1, -1));
    let max = max(&dots).add(&Dot::of(1, 1, 1));
    let part2 = traverse(&dots, &min, &max);
    println!("{} ({} ms)", part2, at.elapsed().as_millis());
}

fn traverse(dots: &HashSet<Dot>, min: &Dot, max: &Dot) -> usize {
    let mut count = 0;
    let mut seen: HashSet<Dot> = HashSet::new();
    let mut queue: VecDeque<Dot> = VecDeque::new();

    seen.insert(min.clone());
    queue.push_back(min.clone());
    while !queue.is_empty() {
        let dot = queue.pop_front().unwrap();

        for next in adj() {
            let next = dot.add(&next);
            if seen.contains(&next) {
                continue;
            }
            if dots.contains(&next) {
                count += 1;
                continue;
            }
            if fits(&next, min, max) {
                queue.push_back(next.clone());
                seen.insert(next);
            }
        }
    }

    count
}

fn fits(dot: &Dot, min: &Dot, max: &Dot) -> bool {
    dot.x >= min.x
        && dot.x <= max.x
        && dot.y >= min.y
        && dot.y <= max.y
        && dot.z >= min.z
        && dot.z <= max.z
}

fn min(dots: &HashSet<Dot>) -> Dot {
    dots.iter()
        .cloned()
        .reduce(|acc, dot| Dot::of(acc.x.min(dot.x), acc.y.min(dot.y), acc.z.min(dot.z)))
        .unwrap()
}

fn max(dots: &HashSet<Dot>) -> Dot {
    dots.iter()
        .cloned()
        .reduce(|acc, dot| Dot::of(acc.x.max(dot.x), acc.y.max(dot.y), acc.z.max(dot.z)))
        .unwrap()
}

fn adj() -> Vec<Dot> {
    vec![
        Dot::of(-1, 0, 0),
        Dot::of(1, 0, 0),
        Dot::of(0, -1, 0),
        Dot::of(0, 1, 0),
        Dot::of(0, 0, -1),
        Dot::of(0, 0, 1),
    ]
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Dot {
    x: isize,
    y: isize,
    z: isize,
}

impl Dot {
    fn of(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    fn parse(line: &str) -> Self {
        let mut it = line.split(',');
        let x = it.next().unwrap().parse().unwrap();
        let y = it.next().unwrap().parse().unwrap();
        let z = it.next().unwrap().parse().unwrap();
        Self { x, y, z }
    }

    fn add(&self, rhs: &Dot) -> Dot {
        Dot::of(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
