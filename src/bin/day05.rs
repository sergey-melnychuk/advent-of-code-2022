use advent_of_code_2022::lines;

fn main() {
    let lines = lines();
    let (state, moves) = parse(&lines);

    let part1 = {
        let mut state = state.clone();
        for m in &moves {
            state.apply1(m);
        }
        state.top()
    };
    println!("{}", part1);

    let part2 = {
        let mut state = state;
        for m in &moves {
            state.apply2(m);
        }
        state.top()
    };
    println!("{}", part2);
}

#[derive(Debug, Clone)]
struct State {
    stacks: Vec<Vec<char>>,
}

impl State {
    fn apply1(&mut self, m: &Move) {
        for _ in 0..m.count {
            self.move_one(m.src-1, m.dst-1)
        }
    }

    fn apply2(&mut self, m: &Move) {
        if m.count == 1 {
            self.move_one(m.src-1, m.dst-1);
        } else {
            let mut tmp = Vec::with_capacity(m.count);
            for _ in 0..m.count {
                if let Some(item) = self.stacks[m.src-1].pop() {
                    tmp.push(item);
                }
            }
            for item in tmp.into_iter().rev() {
                self.stacks[m.dst-1].push(item);
            }
        }
    }

    fn move_one(&mut self, src: usize, dst: usize) {
        if let Some(item) = self.stacks[src].pop() {
            self.stacks[dst].push(item);
        }
    }

    fn top(&self) -> String {
        self.stacks
            .iter()
            .filter_map(|stack| stack.last())
            .collect()
    }
}

struct Move {
    count: usize,
    src: usize,
    dst: usize,
}

fn parse(lines: &[String]) -> (State, Vec<Move>) {
    let mut it = lines.split(|line| line.is_empty());
    let state = it.next().unwrap();
    let state = parse_state(state);

    let moves = it.next().unwrap()
        .iter()
        .map(|line| parse_move(line))
        .collect();

    (state, moves)
}

fn parse_state(lines: &[String]) -> State {
    let indices = lines.last().unwrap()
        .chars()
        .enumerate()
        .filter(|(_, chr)| chr.is_numeric())
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); indices.len()];

    lines.iter()
        .rev().skip(1)
        .for_each(|row| {
            for (idx, pos) in indices.iter().enumerate() {
                let chr = row.chars().nth(*pos).unwrap();
                if chr != ' ' {
                    stacks[idx].push(chr);
                }
            }
        });

    State { stacks }
}

fn parse_move(line: &str) -> Move {
    let parsed = line.split_ascii_whitespace()
        .into_iter()
        .filter_map(|item| item.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let (count, src, dst) = (parsed[0], parsed[1], parsed[2]);
    Move { count, src, dst }
}
