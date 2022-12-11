use advent_of_code_2022::lines;

fn main() {
    let lines = lines();
    let monkeys = lines
        .split(|line| line.is_empty())
        .map(parse)
        .collect::<Vec<_>>();

    let part1 = solve(&monkeys, 20, |x| (x as f64 / 3.0).trunc() as isize);
    println!("{}", part1);

    let k = monkeys.iter().map(|m| m.test).product::<isize>();
    let part2 = solve(&monkeys, 10000, |x| x % k);
    println!("{}", part2);
}

fn solve<F: Fn(isize) -> isize>(monkeys: &[Monkey], n: usize, f: F) -> usize {
    let mut monkeys = monkeys.to_vec();
    for _ in 0..n {
        round(&mut monkeys, &f);
    }

    monkeys.sort_by_key(|m| m.count);
    monkeys.reverse();

    monkeys[0].count * monkeys[1].count
}

fn round<F: Fn(isize) -> isize>(monkeys: &mut [Monkey], f: F) {
    let n = monkeys.len();
    for i in 0..n {
        for item in monkeys[i].round(&f) {
            monkeys[item.monkey].items.push(item.value);
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Add(isize),
    Mul(isize),
    Sqr,
}

#[derive(Debug, Clone)]
struct Monkey {
    op: Op,
    items: Vec<isize>,
    test: isize,
    target: [usize; 2],
    count: usize,
}

struct Item {
    monkey: usize,
    value: isize,
}

impl Monkey {
    fn round<F: Fn(isize) -> isize>(&mut self, f: F) -> Vec<Item> {
        let items = self
            .items
            .iter()
            .map(|item| {
                let value = f(apply(&self.op, *item));
                let target = (if value % self.test == 0 { 0 } else { 1 }) as usize;
                let monkey = self.target[target];
                Item { monkey, value }
            })
            .collect();
        self.count += self.items.len();
        self.items.clear();
        items
    }
}

fn apply(op: &Op, value: isize) -> isize {
    match op {
        Op::Add(x) => value + x,
        Op::Mul(x) => value * x,
        Op::Sqr => value * value,
    }
}

fn parse(lines: &[String]) -> Monkey {
    let items = lines[1]
        .strip_prefix("  Starting items: ")
        .unwrap_or_default()
        .split(", ")
        .into_iter()
        .map(|id| id.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut op_line = lines[2]
        .strip_prefix("  Operation: new = old ")
        .unwrap_or_default()
        .split_ascii_whitespace();
    let op = op_line.next().unwrap();
    let operand = op_line.next().unwrap();
    let is_num = operand.chars().all(|c| c.is_ascii_digit());
    let op = match (op, operand) {
        ("+", x) if is_num => Op::Add(x.parse().unwrap()),
        ("*", x) if is_num => Op::Mul(x.parse().unwrap()),
        ("*", "old") => Op::Sqr,
        _ => panic!("Unexpected op line: '{}'", lines[2]),
    };

    let test = lines[3]
        .strip_prefix("  Test: divisible by ")
        .unwrap_or_default()
        .parse::<isize>()
        .unwrap();

    let target1 = lines[4]
        .strip_prefix("    If true: throw to monkey ")
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap();

    let target2 = lines[5]
        .strip_prefix("    If false: throw to monkey ")
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap();

    Monkey {
        op,
        items,
        test,
        target: [target1, target2],
        count: 0,
    }
}
