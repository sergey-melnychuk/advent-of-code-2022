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

    println!("{}", part2(&lines));
}

// Brute-force enumeration of subsets-of-3 to assign valid item type
fn part2(lines: &[String]) -> usize {
    let n = lines.len();
    let mut code = vec!['?'; n];

    for x in 0..(n - 2) {
        if code[x] != '?' {
            continue;
        }
        for y in (x + 1)..n {
            if code[y] != '?' {
                continue;
            }
            for z in (y + 1)..n {
                if code[z] != '?' {
                    continue;
                }
                let overlap = and3(&lines[x], &lines[y], &lines[z]);
                if overlap.len() == 1 {
                    let w = overlap[0];
                    if code[x] == '?' && code[y] == '?' && code[z] == '?' {
                        code[x] = w;
                        code[y] = w;
                        code[z] = w;
                    }
                }
            }
        }
    }

    // println!("code:\n{:?}", code);
    // let mut map: HashMap<char, Vec<String>> = HashMap::new();
    // for (i, x) in code.iter().enumerate() {
    //     map.entry(*x).or_default().push(lines[i].to_string());
    // }
    // println!("{:#?}", map);

    if code.contains(&'?') {
        panic!("Unassigned items detected")
    }

    code.iter().map(|chr| priority(*chr)).sum::<usize>() / 3
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
