use std::{collections::HashMap, time::Instant};

use advent_of_code_2022::lines;

fn main() {
    let at = Instant::now();
    let ctx = lines()
        .into_iter()
        .map(|line| parse(&line))
        .collect::<HashMap<_, _>>();

    let mut val = HashMap::new();
    let part1 = eval(&ctx, &mut val, &ctx["root"]).unwrap();
    println!("{} ({} ms)", part1, at.elapsed().as_millis());

    let at = Instant::now();
    let lo = 1;
    let sig_lo = sig(f(&ctx, lo));
    let hi = range(lo, 20, |x| x * 10, |x| sig_lo * sig(f(&ctx, x)) < 0);

    let part2 = bsearch(lo, hi, |x| f(&ctx, x));
    println!("{} ({} ms)", part2, at.elapsed().as_millis());
}

fn sig(a: i64) -> i64 {
    match a {
        0 => 0,
        x if x < 0 => -1,
        _ => 1,
    }
}

fn bsearch<F: Fn(i64) -> i64>(mut lo: i64, mut hi: i64, f: F) -> i64 {
    while hi > lo {
        let m = lo + (hi - lo) / 2;
        let y = f(m);
        if y == 0 {
            return m;
        }

        if y < 0 {
            hi = m - 1;
        } else {
            lo = m + 1;
        }
    }
    lo
}

fn range<F: Fn(i64) -> i64, G: Fn(i64) -> bool>(lo: i64, n: usize, step: F, stop: G) -> i64 {
    let mut hi = lo;
    for _ in 0..n {
        hi = step(hi);
        if stop(hi) {
            return hi;
        }
    }
    hi
}

fn f(ctx: &HashMap<Name, Node>, x: i64) -> i64 {
    let (lhs, rhs) = ctx["root"].args().unwrap();
    let a = eval_arg(ctx, ("humn", x), lhs);
    let b = eval_arg(ctx, ("humn", x), rhs);
    a - b
}

fn eval_arg(ctx: &HashMap<Name, Node>, arg: (&str, i64), name: &str) -> i64 {
    let mut val = HashMap::new();
    val.insert(arg.0.to_string(), arg.1);
    eval(ctx, &mut val, &ctx[name]).unwrap()
}

fn eval(ctx: &HashMap<Name, Node>, val: &mut HashMap<Name, i64>, node: &Node) -> Option<i64> {
    match node {
        Node::Val(x) => Some(*x),
        Node::Ref(r) if val.contains_key(r) => Some(val[r]),
        Node::Ref(r) if ctx.contains_key(r) => {
            let x = eval(ctx, val, &ctx[r]);
            if let Some(x) = x {
                val.insert(r.to_string(), x);
            }
            x
        }
        Node::Add(lhs, rhs) => {
            let a = eval(ctx, val, lhs.as_ref());
            let b = eval(ctx, val, rhs.as_ref());
            a.zip(b).map(|(a, b)| a + b)
        }
        Node::Sub(lhs, rhs) => {
            let a = eval(ctx, val, lhs.as_ref());
            let b = eval(ctx, val, rhs.as_ref());
            a.zip(b).map(|(a, b)| a - b)
        }
        Node::Mul(lhs, rhs) => {
            let a = eval(ctx, val, lhs.as_ref());
            let b = eval(ctx, val, rhs.as_ref());
            a.zip(b).map(|(a, b)| a * b)
        }
        Node::Div(lhs, rhs) => {
            let a = eval(ctx, val, lhs.as_ref());
            let b = eval(ctx, val, rhs.as_ref());
            a.zip(b).map(|(a, b)| a / b)
        }
        _ => None,
    }
}

type Name = String;

#[derive(Debug, Clone)]
enum Node {
    Ref(Name),
    Val(i64),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
}

impl Node {
    fn reference(&self) -> Option<&Name> {
        match self {
            Node::Ref(name) => Some(name),
            _ => None,
        }
    }

    fn args(&self) -> Option<(&Name, &Name)> {
        match self {
            Node::Add(a, b) => a.reference().zip(b.reference()),
            Node::Sub(a, b) => a.reference().zip(b.reference()),
            Node::Mul(a, b) => a.reference().zip(b.reference()),
            Node::Div(a, b) => a.reference().zip(b.reference()),
            _ => None,
        }
    }
}

fn parse(line: &str) -> (Name, Node) {
    let mut it = line.split(": ");
    let name = it.next().unwrap().to_string();

    let expr = it.next().unwrap();
    if expr.chars().all(|c| c.is_ascii_digit()) {
        let val = expr.parse::<i64>().unwrap();
        return (name, Node::Val(val));
    }

    let mut it = expr.split_ascii_whitespace();
    let lhs = it.next().unwrap();
    let lhs = if lhs.chars().all(|c| c.is_ascii_digit()) {
        Node::Val(lhs.parse::<i64>().unwrap())
    } else {
        Node::Ref(lhs.to_string())
    };
    let op = it.next().unwrap().chars().next().unwrap();
    let rhs = it.next().unwrap();
    let rhs = if rhs.chars().all(|c| c.is_ascii_digit()) {
        Node::Val(rhs.parse::<i64>().unwrap())
    } else {
        Node::Ref(rhs.to_string())
    };

    match op {
        '+' => (name, Node::Add(Box::new(lhs), Box::new(rhs))),
        '-' => (name, Node::Sub(Box::new(lhs), Box::new(rhs))),
        '*' => (name, Node::Mul(Box::new(lhs), Box::new(rhs))),
        '/' => (name, Node::Div(Box::new(lhs), Box::new(rhs))),
        _ => panic!("Unsupported op: {}", op),
    }
}
