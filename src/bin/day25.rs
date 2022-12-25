use advent_of_code_2022::lines;

fn main() {
    let numbers = lines();
    let sum = numbers.iter().map(|n| from_snafu(n)).sum::<i64>();
    println!("{}", into_snafu(sum));
}

/*

'2'
'1'
'0'
'-' = -1
'=' = -2

*/

fn into_snafu(mut n: i64) -> String {
    let mut ret: Vec<char> = Vec::new();

    let mut carry: bool = false;
    while n > 0 {
        if carry {
            n += 1;
            carry = false;
        }
        let r = n % 5;
        if (0..=2).contains(&r) {
            let d = char::from_digit(r as u32, 10).unwrap_or('?');
            ret.push(d);
        } else if r == 3 {
            ret.push('=');
            carry = true;
        } else if r == 4 {
            ret.push('-');
            carry = true;
        }
        n /= 5;
    }

    if carry {
        ret.push('1');
    }

    ret.into_iter().rev().collect()
}

fn from_snafu(n: &str) -> i64 {
    n.chars()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, c)| num(c) * pow5(i as u8))
        .sum()
}

fn num(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!("'{}'?", c),
    }
}

fn pow5(mut n: u8) -> i64 {
    let mut x = 1;
    while n > 0 {
        x *= 5;
        n -= 1;
    }
    x
}

#[cfg(test)]
mod day25 {
    use super::*;

    #[test]
    fn test_from_snafu() {
        for (snafu, decimal) in [
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ] {
            assert_eq!(from_snafu(snafu), decimal, "'{}' == {}", snafu, decimal);
        }
    }

    #[test]
    fn test_into_snafu() {
        for (snafu, decimal) in [
            ("1=-0-2", 1747),
            ("12111", 906),
            ("2=0=", 198),
            ("21", 11),
            ("2=01", 201),
            ("111", 31),
            ("20012", 1257),
            ("112", 32),
            ("1=-1=", 353),
            ("1-12", 107),
            ("12", 7),
            ("1=", 3),
            ("122", 37),
        ] {
            assert_eq!(into_snafu(decimal), snafu, "{} == '{}'", decimal, snafu);
        }
    }
}
