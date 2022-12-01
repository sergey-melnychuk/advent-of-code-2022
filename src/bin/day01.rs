use advent_of_code_2022::lines;

fn main() {
    let mut calories = lines()
        .split(|line| line.is_empty())
        .into_iter()
        .map(|seq| seq.into_iter()
            .map(|num| num.parse::<i64>().unwrap())
            .sum::<i64>()
        )
        .collect::<Vec<_>>();

    let part1 = calories.iter().max().unwrap();
    println!("{}", part1); 

    calories.sort_by_key(|x| -x);

    let part2: i64 = calories[0..3].iter().sum();
    println!("{}", part2); 
}
