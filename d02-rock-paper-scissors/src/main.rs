use itertools::Itertools;

use aoc::get_lines;

fn main() {
    let strat: Vec<(i32, i32)> = get_lines("input.txt")
        .map(|l| (l.chars().nth(0).unwrap(), l.chars().nth(2).unwrap()))
        .map(|(a, b)| (a.into() - 'A'.into(), b.into() - 'X'.into()))
        .collect();

    let conversion = HashMap::from([('X', RPS::Rock), ('Y', RPS::Paper), ('Z', RPS::Scissors)]);

    let s: i32 = strat
        .iter()
        .map(|(a, b)| (*a, *conversion.get(&b).unwrap()))
        .map(|(a, b)| (b - a) % 3)
        .sum();

    println!("{}", s);

    let conversion = HashMap::from([('X', XYZ::Lose), ('Y', XYZ::Draw), ('Z', XYZ::Win)]);

    let s: i32 = strat
        .iter()
        .map(|(a, b)| (*a, *conversion.get(&b).unwrap()))
        .map(|(a, outcome)| score(a, fix(outcome, a)))
        .sum();

    println!("{}", s);
}
