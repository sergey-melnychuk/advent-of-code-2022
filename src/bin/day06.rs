use std::collections::HashSet;

use advent_of_code_2022::lines;

fn main() {
    let line = lines().into_iter().next().unwrap();

    println!("{}", find(&line, 4));
    println!("{}", find(&line, 14));
}

fn find(s: &str, n: usize) -> usize {
    let offset = s
        .as_bytes()
        .windows(n)
        .enumerate()
        .find(|(_, window)| unique(window) == n)
        .map(|(idx, _)| idx)
        .unwrap_or_default();

    offset + n
}

fn unique(slice: &[u8]) -> usize {
    slice.iter().collect::<HashSet<_>>().len()
}
