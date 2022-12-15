use std::collections::HashSet;

use advent_of_code_2022::lines;

fn main() {
    let all: Vec<(Sensor, Beacon)> = lines().into_iter().map(|line| parse(&line)).collect();

    let (min, max) = all
        .iter()
        .map(|(sensor, beacon)| bound(sensor, beacon))
        .reduce(|acc, (min, max)| {
            let min = Dot::of(acc.0.x.min(min.x), acc.0.y.min(min.y));
            let max = Dot::of(acc.1.x.max(max.x), acc.1.y.max(max.y));
            (min, max)
        })
        .unwrap();

    let y = 2000000;
    let part1 = (min.x..=max.x)
        .into_iter()
        .map(|x| Dot::of(x, y))
        .filter(|dot| hits(dot, &all))
        .count();
    println!("{}", part1);

    let part2 = all
        .iter()
        .flat_map(|(sensor, beacon)| frontier(sensor, beacon))
        .filter(|dot| dot.x >= 0 && dot.x <= 4000000 && dot.y >= 0 && dot.y <= 4000000)
        .find(|dot| !hits(dot, &all))
        .map(|dot| dot.x * 4000000 + dot.y)
        .unwrap();
    println!("{:?}", part2);
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

    fn len(&self, that: &Dot) -> isize {
        (that.x - self.x).abs() + (that.y - self.y).abs()
    }
}

fn hits(dot: &Dot, all: &[(Sensor, Beacon)]) -> bool {
    all.iter().all(|(_, beacon)| &beacon.0 != dot)
        && all.iter().any(|(sensor, beacon)| {
            let len = sensor.0.len(&beacon.0);
            sensor.0.len(dot) <= len
        })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Sensor(Dot);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Beacon(Dot);

fn bound(sensor: &Sensor, beacon: &Beacon) -> (Dot, Dot) {
    let len = sensor.0.len(&beacon.0);
    let min = Dot::of(sensor.0.x - len, sensor.0.y - len);
    let max = Dot::of(sensor.0.x + len, sensor.0.y + len);
    (min, max)
}

fn frontier(sensor: &Sensor, beacon: &Beacon) -> HashSet<Dot> {
    let (min, max) = bound(sensor, beacon);
    let east = Dot::of(min.x - 1, sensor.0.y);
    let west = Dot::of(max.x + 1, sensor.0.y);
    let north = Dot::of(sensor.0.x, min.y - 1);
    let south = Dot::of(sensor.0.x, max.y + 1);

    vec![
        diagonal(&east, &north),
        diagonal(&north, &west),
        diagonal(&west, &south),
        diagonal(&south, &east),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn diagonal(a: &Dot, b: &Dot) -> Vec<Dot> {
    assert!((a.x - b.x).abs() == (a.y - b.y).abs());
    let len = (a.x - b.x).abs();
    let dx = (b.x - a.x) / len;
    let dy = (b.y - a.y) / len;
    (0..len)
        .into_iter()
        .map(|i| Dot::of(a.x + i * dx, a.y + i * dy))
        .collect()
}

fn parse(line: &str) -> (Sensor, Beacon) {
    let parsed = line
        .split_ascii_whitespace()
        .into_iter()
        .filter(|line| line.starts_with("x=") || line.starts_with("y="))
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .map(|num| num.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    (
        Sensor(Dot {
            x: parsed[0],
            y: parsed[1],
        }),
        Beacon(Dot {
            x: parsed[2],
            y: parsed[3],
        }),
    )
}

#[cfg(test)]
mod day15 {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("Sensor at x=2389280, y=2368338: closest beacon is at x=2127703, y=2732666"),
            (
                Sensor(Dot {
                    x: 2389280,
                    y: 2368338
                }),
                Beacon(Dot {
                    x: 2127703,
                    y: 2732666
                })
            )
        );
    }
}
