use std::collections::HashSet;

use advent_of_code_2022::lines;

fn main() {
    let lines = lines();

    let part1 = lines
        .iter()
        .flat_map(|line| {
            let (a, b) = split(line);
            and2(a, b)
        })
        .map(priority)
        .sum::<usize>();
    println!("{}", part1);

    let part2 = lines
        .chunks(3)
        .flat_map(|chunk| and3(&chunk[0], &chunk[1], &chunk[2]))
        .map(priority)
        .sum::<usize>();
    println!("{}", part2);
}

fn split(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    s.split_at(mid)
}

fn and2(a: &str, b: &str) -> HashSet<char> {
    let a = a.chars().collect::<HashSet<_>>();
    let b = b.chars().collect::<HashSet<_>>();
    a.intersection(&b).cloned().collect()
}

fn and3(a: &str, b: &str, c: &str) -> Vec<char> {
    let a = a.chars().collect::<HashSet<_>>();
    let b = b.chars().collect::<HashSet<_>>();
    let c = c.chars().collect::<HashSet<_>>();

    let ab = a.intersection(&b).cloned().collect::<HashSet<_>>();
    ab.intersection(&c).cloned().collect()
}

fn priority(chr: char) -> usize {
    let c = chr as u8;
    if (b'a'..=b'z').contains(&c) {
        return (c - b'a' + 1) as usize;
    }
    if (b'A'..=b'Z').contains(&c) {
        return (c - b'A' + 1 + 26) as usize;
    }
    panic!("Unexpected char: {}", chr)
}
