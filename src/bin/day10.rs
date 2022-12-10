use std::str::FromStr;

use advent_of_code_2022::lines;

#[derive(Debug)]
struct Cpu {
    regx: i64,
    cycles: i64,
    signal: i64,
    screen: Vec<Vec<char>>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            regx: 1,
            cycles: 1,
            signal: 0,
            screen: vec![vec!['.'; 40]; 6],
        }
    }
}

impl Cpu {
    fn op(&mut self, op: &Op) {
        match op {
            Op::Noop => {
                self.pixel();
                self.sum();
                self.cycles += 1;
            }
            Op::Addx(x) => {
                self.pixel();
                self.sum();
                self.cycles += 1;
                self.pixel();
                self.sum();
                self.cycles += 1;
                self.regx += *x;
            }
        }
    }

    fn sum(&mut self) {
        if (self.cycles - 20) % 40 == 0 {
            self.signal += self.cycles * self.regx;
        }
    }

    fn pixel(&mut self) {
        let row = if self.cycles <= 40 {
            0
        } else {
            (self.cycles - 1) / 40
        };
        let col = if self.cycles <= 40 {
            self.cycles - 1
        } else {
            (self.cycles - 1) % 40
        };

        if (self.regx - col).abs() <= 1 {
            self.screen[row as usize][col as usize] = '#';
        }
    }

    fn screen(&self) -> String {
        self.screen
            .iter()
            .map(|line| line.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[derive(Debug)]
enum Op {
    Noop,
    Addx(i64),
}

impl FromStr for Op {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            return Ok(Op::Noop);
        }
        if s.starts_with("addx") {
            let x: i64 = s
                .split_ascii_whitespace()
                .nth(1)
                .unwrap()
                .parse()
                .map_err(|e| format!("invalid i64: {}", e))?;
            return Ok(Op::Addx(x));
        }
        Err(format!("invalid op: '{}'", s))
    }
}

fn main() {
    let ops = lines()
        .into_iter()
        .map(|s| s.parse::<Op>().unwrap())
        .collect::<Vec<_>>();

    let mut cpu = Cpu::default();
    for op in &ops {
        cpu.op(op);
    }

    println!("{}", cpu.signal);
    println!("{}", cpu.screen());
}
