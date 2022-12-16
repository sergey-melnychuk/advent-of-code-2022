use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    time::Instant,
};

use advent_of_code_2022::lines;

fn main() {
    let valves: HashMap<Name, Valve> = lines()
        .into_iter()
        .map(|line| parse(&line))
        .map(|valve| (valve.name, valve))
        .collect::<HashMap<_, _>>();

    let at = Instant::now();
    let time: isize = 30;
    let index = index(&valves);
    let (part1, _) = flow(&index, &valves, &['A', 'A'], HashSet::new(), time, vec![]);
    println!("{} ({} ms)", part1, at.elapsed().as_millis());

    // Inspired by: https://github.com/tipa16384/adventofcode/blob/main/2022/puzzle16.py

    let at = Instant::now();
    let time: isize = 26;
    let (cost1, path) = flow(&index, &valves, &['A', 'A'], HashSet::new(), time, vec![]);
    let open = path
        .into_iter()
        .map(|(name, _)| name)
        .collect::<HashSet<_>>();
    let (cost2, _) = flow(&index, &valves, &['A', 'A'], open, time, vec![]);
    println!("{} ({} ms)", cost1 + cost2, at.elapsed().as_millis());
}

type Name = [char; 2];

#[derive(Debug, Clone)]
struct Valve {
    name: Name,
    rate: isize,
    path: Vec<Name>,
}

fn index(valves: &HashMap<Name, Valve>) -> HashMap<Name, HashMap<Name, isize>> {
    valves
        .iter()
        .map(|(name, _)| {
            let dist = scan(valves, name)
                .into_iter()
                .filter(|(n, _)| valves[n].rate > 0)
                .collect();
            (*name, dist)
        })
        .collect()
}

fn flow(
    index: &HashMap<Name, HashMap<Name, isize>>,
    valves: &HashMap<Name, Valve>,
    at: &Name,
    open: HashSet<Name>,
    time: isize,
    path: Vec<(Name, isize)>,
) -> (isize, Vec<(Name, isize)>) {
    if time <= 0 {
        return (0, path);
    }

    let valve = &valves[at];
    let score = valve.rate * time.max(0);

    let mut top_score = 0;
    let mut top_path = path.clone();

    for (next, dist) in &index[at] {
        if open.contains(next) || time - dist - 1 <= 0 {
            continue;
        }

        let (sub_score, sub_path) = flow(
            index,
            valves,
            next,
            extend(&open, next),
            time - dist - 1,
            append(&path, &(*next, time - dist - 1)),
        );

        if sub_score > top_score {
            top_score = sub_score;
            top_path = sub_path;
        }
    }

    (score + top_score, top_path)
}

fn scan(valves: &HashMap<Name, Valve>, at: &Name) -> HashMap<Name, isize> {
    let mut dist = HashMap::new();
    valves.keys().for_each(|name| {
        dist.insert(*name, isize::MAX);
    });
    dist.insert(*at, 0);

    let mut seen: HashSet<Name> = HashSet::new();
    seen.insert(*at);
    let mut frontier: VecDeque<Name> = VecDeque::new();
    frontier.push_back(*at);

    while !frontier.is_empty() {
        let name = frontier.pop_front().unwrap();
        seen.insert(name);

        let valve = valves.get(&name).unwrap();
        for next in &valve.path {
            if seen.contains(next) {
                continue;
            }
            let d = dist[&name] + 1;
            if dist[next] > d {
                dist.insert(*next, d);
            }
            frontier.push_back(*next);
        }
    }

    dist
}

fn append<T: Clone>(vec: &Vec<T>, item: &T) -> Vec<T> {
    let mut cloned = vec.to_owned();
    cloned.push(item.clone());
    cloned
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
