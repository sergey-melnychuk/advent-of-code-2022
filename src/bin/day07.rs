use advent_of_code_2022::lines;

#[derive(Debug)]
enum Entry {
    Dir(String, Vec<Entry>),
    File(String, usize),
}

#[allow(dead_code)]
fn dump(entry: &Entry, level: usize) {
    let offset = vec!["  "; level].join("");
    match entry {
        Entry::File(name, size) => println!("{}{} {}", offset, name, size),
        Entry::Dir(name, list) => {
            println!("{}{}", offset, name);
            for e in list {
                dump(e, level + 1);
            }
        }
    }
}

fn parse(lines: Vec<String>) -> Entry {
    let mut path: Vec<Entry> = Vec::new();
    for line in lines {
        if line.starts_with("$ cd ..") {
            let this = path.pop().unwrap();
            match path.last_mut() {
                Some(Entry::Dir(_, children)) => children.push(this),
                _ => panic!("parent is not dir or missing"),
            };
            continue;
        }
        if line.starts_with("$ cd ") {
            let name = line
                .strip_prefix("$ cd ")
                .map(|s| s.to_string())
                .unwrap_or_default();
            let entry = Entry::Dir(name, Vec::new());
            path.push(entry);
            continue;
        }
        if line.starts_with("$ ls") {
            continue;
        }

        if line.starts_with("dir") {
            // skip
        } else {
            let mut it = line.split_ascii_whitespace();
            let size: usize = it.next().unwrap().parse().unwrap();
            let name = it.next().unwrap().to_string();
            let entry = Entry::File(name, size);
            match path.last_mut() {
                Some(Entry::Dir(_, children)) => children.push(entry),
                _ => panic!("parent is not dir or missing"),
            };
        }
    }

    while path.len() > 1 {
        let entry = path.pop().unwrap();
        match path.last_mut() {
            Some(Entry::Dir(_, children)) => children.push(entry),
            _ => panic!("parent is not dir or missing"),
        };
    }
    assert_eq!(path.len(), 1);

    path.remove(0)
}

fn part1(root: &Entry) -> (usize, usize) {
    let mut sum = 0;
    let size = size1(root, &mut sum, 100000);
    (size, sum)
}

fn size1(entry: &Entry, acc: &mut usize, limit: usize) -> usize {
    match entry {
        Entry::Dir(_, list) => {
            let mut sum = 0;
            for e in list {
                let size = size1(e, acc, limit);
                sum += size;
            }
            if sum <= limit {
                *acc += sum;
            }
            sum
        }
        Entry::File(_, size) => *size,
    }
}

fn part2(root: &Entry, target: usize) -> usize {
    let mut out = usize::MAX;
    size2(root, &mut out, target);
    out
}

fn size2(entry: &Entry, out: &mut usize, target: usize) -> usize {
    match entry {
        Entry::Dir(_, list) => {
            let mut sum = 0;
            for e in list {
                let size = size2(e, out, target);
                sum += size;
            }
            if sum >= target && sum < *out {
                *out = sum;
            }
            sum
        }
        Entry::File(_, size) => *size,
    }
}

fn main() {
    let root = parse(lines());

    let (size, part1) = part1(&root);
    println!("{}", part1);

    let total: usize = 70000000;
    let unused: usize = 30000000;

    let target = unused - (total - size);
    let part2 = part2(&root, target);
    println!("{}", part2);
}
