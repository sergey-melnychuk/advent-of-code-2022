use std::collections::{HashSet, VecDeque};

use advent_of_code_2022::lines;

fn main() {
    let costs = lines()
        .into_iter()
        .map(|line| Cost::parse(&line))
        .collect::<Vec<_>>();

    let time1 = 24;
    let part1 = costs
        .iter()
        .enumerate()
        .map(|(idx, spec)| (idx as i32 + 1) * bfs(spec, time1))
        .sum::<i32>();
    println!("{}", part1);

    let time2 = 32;
    let part2 = costs
        .iter()
        .take(3)
        .map(|spec| bfs(spec, time2))
        .product::<i32>();
    println!("{}", part2);
}

// Inspired by:
// https://github.com/wilkotom/AoC2022/blob/main/day19/src/main.rs
fn bfs(cost: &Cost, time: i32) -> i32 {
    let robots = Robots {
        ore: 1,
        cla: 0,
        obs: 0,
        geo: 0,
    };
    let spare = Spare::default();

    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((time, robots, spare));

    let max_ore = cost.max_ore();

    let mut best = 0;
    while let Some(state) = queue.pop_front() {
        let (time, robots, spare) = state;
        best = best.max(spare.geo);

        if seen.contains(&state) || time == 0 {
            continue;
        }

        if spare.geo < best {
            continue;
        }

        seen.insert(state);

        if spare.ore >= cost.geo.ore && spare.obs >= cost.geo.obs {
            let mut spare = spare.add(&robots);
            spare.ore -= cost.geo.ore;
            spare.obs -= cost.geo.obs;
            let mut robots = robots;
            robots.geo += 1;
            queue.push_back((time - 1, robots, spare));
            continue;
        }

        if spare.ore >= cost.ore && robots.ore < max_ore {
            let mut spare = spare.add(&robots);
            spare.ore -= cost.ore;
            let mut robots = robots;
            robots.ore += 1;
            queue.push_back((time - 1, robots, spare));
        }

        if spare.ore >= cost.cla && robots.cla < cost.obs.cla {
            let mut spare = spare.add(&robots);
            spare.ore -= cost.cla;
            let mut robots = robots;
            robots.cla += 1;
            queue.push_back((time - 1, robots, spare));
        }

        if spare.ore >= cost.obs.ore && spare.cla >= cost.obs.cla {
            let mut spare = spare.add(&robots);
            spare.ore -= cost.obs.ore;
            spare.cla -= cost.obs.cla;
            let mut robots = robots;
            robots.obs += 1;
            queue.push_back((time - 1, robots, spare));
        }

        queue.push_back((time - 1, robots, spare.add(&robots)));
    }
    best
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Spare {
    ore: i32,
    cla: i32,
    obs: i32,
    geo: i32,
}

impl Spare {
    fn add(self, robots: &Robots) -> Self {
        let mut this = self;
        this.ore += robots.ore;
        this.cla += robots.cla;
        this.obs += robots.obs;
        this.geo += robots.geo;
        this
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Robots {
    ore: i32,
    cla: i32,
    obs: i32,
    geo: i32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Cost {
    ore: i32,
    cla: i32,
    obs: ObsCost,
    geo: GeoCost,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct ObsCost {
    ore: i32,
    cla: i32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct GeoCost {
    ore: i32,
    obs: i32,
}

impl Cost {
    fn max_ore(&self) -> i32 {
        [self.ore, self.cla, self.obs.ore, self.geo.ore]
            .into_iter()
            .max()
            .unwrap_or_default()
    }

    fn parse(line: &str) -> Self {
        let line = {
            let mut it = line.split(':');
            it.next();
            it.next().unwrap()
        };

        let mut it = line.split('.').take(4);
        let line1 = it
            .next()
            .unwrap()
            .strip_prefix(" Each ore robot costs ")
            .unwrap();
        let cost1 = num(line1);
        let line2 = it
            .next()
            .unwrap()
            .strip_prefix(" Each clay robot costs ")
            .unwrap();
        let cost2 = num(line2);

        let line3 = it
            .next()
            .unwrap()
            .strip_prefix(" Each obsidian robot costs ")
            .unwrap();
        let mut it3 = line3.split(" and ");
        let cost31 = num(it3.next().unwrap());
        let cost32 = num(it3.next().unwrap());

        let line4 = it
            .next()
            .unwrap()
            .strip_prefix(" Each geode robot costs ")
            .unwrap();
        let mut it4 = line4.split(" and ");
        let cost41 = num(it4.next().unwrap());
        let cost42 = num(it4.next().unwrap());

        Cost {
            ore: cost1,
            cla: cost2,
            obs: ObsCost {
                ore: cost31,
                cla: cost32,
            },
            geo: GeoCost {
                ore: cost41,
                obs: cost42,
            },
        }
    }
}

fn num(line: &str) -> i32 {
    let mut it = line.split_ascii_whitespace();
    it.next().unwrap().parse().unwrap()
}

#[cfg(test)]
mod day19 {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 2 ore and 12 obsidian.";
        assert_eq!(
            Cost::parse(s),
            Cost {
                ore: 4,
                cla: 4,
                obs: ObsCost { ore: 4, cla: 20 },
                geo: GeoCost { ore: 2, obs: 12 },
            }
        );
    }
}
