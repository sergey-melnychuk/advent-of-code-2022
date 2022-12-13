use std::cmp::Ordering;

use serde::Deserialize;

use advent_of_code_2022::lines;

fn main() {
    let pairs: Vec<Vec<Node>> = lines()
        .split(|line| line.is_empty())
        .into_iter()
        .map(|pair| pair.iter().map(|line| Node::from_str(line)).collect())
        .collect();

    let part1 = pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            let a = &pair[0];
            let b = &pair[1];
            if a < b {
                Some(idx + 1)
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("{}", part1);

    let mut packets: Vec<Node> = pairs
        .into_iter()
        .flat_map(|pair| pair.into_iter())
        .collect();

    let a = Node::Vec(vec![Node::Val(2)]);
    let b = Node::Vec(vec![Node::Val(6)]);
    packets.push(a.clone());
    packets.push(b.clone());
    packets.sort();

    let part2 = packets
        .iter()
        .enumerate()
        .filter_map(|(idx, packet)| {
            if packet == &a || packet == &b {
                Some(idx + 1)
            } else {
                None
            }
        })
        .product::<usize>();
    println!("{}", part2);
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Val(usize),
    Vec(Vec<Node>),
}

impl Node {
    fn from_str(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Node::Val(x), Node::Val(y)) => x.partial_cmp(y),
            (Node::Vec(x), Node::Vec(y)) => x.partial_cmp(y),
            (Node::Val(_), Node::Vec(_)) => Node::Vec(vec![self.clone()]).partial_cmp(other),
            (Node::Vec(_), Node::Val(_)) => self.partial_cmp(&Node::Vec(vec![other.clone()])),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
