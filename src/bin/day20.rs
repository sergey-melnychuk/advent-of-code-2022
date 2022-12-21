use advent_of_code_2022::lines;

fn main() {
    let xs = lines()
        .into_iter()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let decoded = mix(&xs, 1, 1);
    let part1 = ans(&decoded);
    println!("{}", part1);

    let key: i64 = 811589153;
    let rounds = 10;
    let decoded = mix(&xs, key, rounds);
    let part2 = ans(&decoded);
    println!("{}", part2);
}

fn ans(xs: &[i64]) -> i64 {
    let idx = xs.iter().position(|x| x == &0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|offset| (offset + idx) % xs.len())
        .map(|i| xs[i])
        .sum()
}

// Inspired by:
// https://github.com/wilkotom/AoC2022/blob/main/day20/src/main.rs
fn mix(xs: &[i64], key: i64, rounds: usize) -> Vec<i64> {
    let mut ret = xs
        .iter()
        .copied()
        .map(|x| x * key)
        .enumerate()
        .collect::<Vec<_>>();

    for _ in 0..rounds {
        for (i, x) in xs.iter().map(|x| *x * key).enumerate() {
            let idx = ret.iter().position(|item| item == &(i, x)).unwrap();
            ret.remove(idx);

            let idx = (idx as i64 + x).rem_euclid(ret.len() as i64) as usize;
            if idx == 0 {
                ret.push((i, x));
            } else {
                ret.insert(idx, (i, x));
            }
        }
    }

    ret.into_iter().map(|(_, x)| x).collect()
}
