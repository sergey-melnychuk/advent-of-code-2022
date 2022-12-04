use advent_of_code_2022::lines;

fn main() {
    let pairs = lines().into_iter()
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();

    let part1 = pairs.iter()
        .filter(|(a, b)| contain(a, b) || contain(b, a))
        .count();
    println!("{}", part1);

    let part2 = pairs.iter()
        .filter(|(a, b)| overlap(a, b) || overlap(b, a))
        .count();
    println!("{}", part2);
}

#[derive(Debug, Eq, PartialEq)]
struct Seq {
    lo: i64,
    hi: i64,
}

fn parse_line(line: &str) -> (Seq, Seq) {
    let mut it = line.split(',');
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    (parse_seq(a), parse_seq(b))
}

fn parse_seq(line: &str) -> Seq {
    let mut it = line.split('-');
    let lo = it.next().unwrap().parse().unwrap();
    let hi = it.next().unwrap().parse().unwrap();
    Seq { lo, hi }
}

fn contain(a: &Seq, b: &Seq) -> bool {
    b.lo >= a.lo && b.hi <= a.hi
}

fn overlap(a: &Seq, b: &Seq) -> bool {
    b.lo >= a.lo && b.lo <= a.hi
}

#[cfg(test)]
mod day04 {
    use super::*;

    fn seq(lo: i64, hi: i64) -> Seq {
        Seq { lo, hi }
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse_line("2-4,6-8"), (seq(2, 4), seq(6, 8)));
    }
}