use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash, time::Instant,
};

use advent_of_code_2022::lines;

fn main() {
    let valves: HashMap<Name, Valve> = lines()
        .into_iter()
        .map(|line| parse(&line))
        .map(|valve| (valve.name, valve))
        .collect::<HashMap<_, _>>();

    let mut at = Instant::now();
    let time: isize = 30;
    let (part1, _) = flow(&valves, &['A', 'A'], HashSet::new(), time, vec![]);
    println!("{} ({} ms)", part1, at.elapsed().as_millis());

    // Inspired by: https://github.com/tipa16384/adventofcode/blob/main/2022/puzzle16.py
    at = Instant::now();
    let time: isize = 26;
    let (cost1, path) = flow(&valves, &['A', 'A'], HashSet::new(), time, vec![]);
    let open = path.into_iter()
        .map(|(name, _)| name.clone())
        .collect::<HashSet<_>>();
    let (cost2, _) = flow(&valves, &['A', 'A'], open, time, vec![]);
    println!("{} ({} ms)", cost1 + cost2, at.elapsed().as_millis());
}

type Name = [char; 2];

#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    rate: isize,
    path: Vec<Name>,
}

fn flow(valves: &HashMap<Name, Valve>, at: &Name, open: HashSet<Name>, time: isize, path: Vec<(Name, isize)>) -> (isize, Vec<(Name, isize)>) {
    next(valves, at, &open, time)
        .into_iter()
        .map(|(name, dist, cost)| {
            let open = extend(&open, &name);
            let time = time - dist - 1;
            let mut path = path.clone();
            path.push((name.clone(), time));
            let (more, next) = flow(valves, &name, open, time, path.clone());
            (cost + more, if next.is_empty() {path} else {next})
        })
        .max_by_key(|(cost, _)| *cost)
        .unwrap_or_default()
}

fn next(
    valves: &HashMap<Name, Valve>,
    at: &Name,
    open: &HashSet<Name>,
    time: isize,
) -> Vec<(Name, isize, isize)> {
    let mut next = scan(valves, at.clone())
        .into_iter()
        .filter(|(name, _)| valves.get(name).unwrap().rate > 0)
        .filter(|(name, _)| !open.contains(name))
        .filter(|(_, dist)| *dist < time)
        .map(|(name, dist)| {
            (
                name,
                dist,
                cost(time, dist, valves.get(&name).unwrap().rate),
            )
        })
        .collect::<Vec<_>>();

    next.sort_by_key(|(_, _, cost)| -cost);
    next
}

fn cost(time: isize, dist: isize, rate: isize) -> isize {
    (time - 1 - dist) * rate
}

fn scan(valves: &HashMap<Name, Valve>, at: Name) -> HashMap<Name, isize> {
    let mut dist = HashMap::new();
    valves.keys().for_each(|name| {
        dist.insert(name.clone(), isize::MAX);
    });
    dist.insert(at.clone(), 0);

    let mut seen: HashSet<Name> = HashSet::new();
    seen.insert(at.clone());
    let mut frontier: VecDeque<Name> = VecDeque::new();
    frontier.push_back(at.clone());

    while !frontier.is_empty() {
        let name = frontier.pop_front().unwrap();
        seen.insert(name.clone());

        let valve = valves.get(&name).unwrap();
        for next in &valve.path {
            if seen.contains(next) {
                continue;
            }
            let d = dist[&name] + 1;
            if dist[next] > d {
                dist.insert(next.clone(), d);
            }
            frontier.push_back(next.clone());
        }
    }

    dist
}

fn extend<T: Eq + Clone + Hash>(set: &HashSet<T>, item: &T) -> HashSet<T> {
    let mut cloned = set.to_owned();
    cloned.insert(item.clone());
    cloned
}

fn parse(line: &str) -> Valve {
    let names = line
        .split_ascii_whitespace()
        .filter(|chunk| chunk.chars().take(2).all(|c| c.is_ascii_uppercase()))
        .map(|chunk| {
            let chars = chunk.chars().take(2).collect::<Vec<_>>();
            chars.try_into().unwrap()
        })
        .collect::<Vec<_>>();

    let rate: isize = line
        .split_ascii_whitespace()
        .find(|chunk| chunk.starts_with("rate="))
        .map(|chunk| {
            chunk
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .map(|rate| rate.parse().unwrap())
        .unwrap_or_default();

    let name = names[0];
    let path = names.into_iter().skip(1).collect();

    Valve { name, rate, path }
}

#[cfg(test)]
mod day16 {
    use super::*;

    #[test]
    fn test_parse() {
        let valve = parse("Valve AA has flow rate=10; tunnels lead to valves XU, JH, CD, WY, HK");
        assert_eq!(valve.name, ['A', 'A']);
        assert_eq!(valve.rate, 10);
        assert_eq!(
            valve.path,
            vec![['X', 'U'], ['J', 'H'], ['C', 'D'], ['W', 'Y'], ['H', 'K']]
        )
    }
}
