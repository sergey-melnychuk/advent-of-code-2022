use advent_of_code_2022::lines;

// Rock A, 1
// Paper B, 2
// Scissors C, 3
// (lost: 0, draw: 3, win: 6)

// X - Rock,
// Y - Paper,
// Z - Scissors,
fn play1(a: u8, b: u8) -> u64 {
    match (a, b) {
        (b'A', b'X') => 1 + 3,
        (b'A', b'Y') => 2 + 6,
        (b'A', b'Z') => 3 + 0,
        (b'B', b'X') => 1 + 0,
        (b'B', b'Y') => 2 + 3,
        (b'B', b'Z') => 3 + 6,
        (b'C', b'X') => 1 + 6,
        (b'C', b'Y') => 2 + 0,
        (b'C', b'Z') => 3 + 3,
        _ => 0
    }
}

// Rock A, 1
// Paper B, 2
// Scissor C, 3
// (lost: 0, draw: 3, win: 6)

// X - lose,
// Y - draw,
// Z - win,
fn play2(a: u8, b: u8) -> u64 {
    match (a, b) {
        (b'A', b'X') => 3 + 0,
        (b'A', b'Y') => 1 + 3,
        (b'A', b'Z') => 2 + 6,
        (b'B', b'X') => 1 + 0,
        (b'B', b'Y') => 2 + 3,
        (b'B', b'Z') => 3 + 6,
        (b'C', b'X') => 2 + 0,
        (b'C', b'Y') => 3 + 3,
        (b'C', b'Z') => 1 + 6,
        _ => 0
    }
}

fn main() {
    let input = lines()
        .into_iter()
        .map(|line| {
            let bytes = line.as_bytes();
            let a = bytes[0];
            let b = bytes[2];
            (a, b)
        })
        .collect::<Vec<_>>();

    let part1 = input.iter()
        .map(|(a, b)| play1(*a, *b))
        .sum::<u64>();
    println!("{}", part1);

    let part2 = input.iter()
        .map(|(a, b)| play2(*a, *b))
        .sum::<u64>();
    println!("{}", part2);

}