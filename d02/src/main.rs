use std::collections::HashMap;

use aoc::get_lines;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

    fn fix(outcome: XYZ, opponent: RPS) -> RPS {
        match (outcome, opponent) {
            (XYZ::Win, RPS::Rock) => RPS::Paper,
            (XYZ::Win, RPS::Paper) => RPS::Scissors,
            (XYZ::Win, RPS::Scissors) => RPS::Rock,
            (XYZ::Draw, _) => opponent,
            (XYZ::Lose, RPS::Rock) => RPS::Scissors,
            (XYZ::Lose, RPS::Paper) => RPS::Rock,
            (XYZ::Lose, RPS::Scissors) => RPS::Paper,
        }
    }

#[derive(Debug, Clone, Copy)]
enum XYZ {
    Win,
    Draw,
    Lose,
}

fn score(other: RPS, me: RPS) -> i32 {
    match (me, other) {
        (RPS::Rock, RPS::Rock) => 1 + 3,
        (RPS::Rock, RPS::Paper) => 1,
        (RPS::Rock, RPS::Scissors) => 1 + 6,
        (RPS::Paper, RPS::Rock) => 2 + 6,
        (RPS::Paper, RPS::Paper) => 2 + 3,
        (RPS::Paper, RPS::Scissors) => 2,
        (RPS::Scissors, RPS::Rock) => 3,
        (RPS::Scissors, RPS::Paper) => 3 + 6,
        (RPS::Scissors, RPS::Scissors) => 3 + 3,
    }
}

fn main() {
    let strat: Vec<(RPS, char)> = get_lines("input.txt")
        .map(|l| (l.chars().next().unwrap(), l.chars().nth(2).unwrap()))
        .map(|(a, b)| {
            (
                match a {
                    'A' => RPS::Rock,
                    'B' => RPS::Paper,
                    'C' => RPS::Scissors,
                    _ => unreachable!(),
                },
                b,
            )
        })
        .collect();

    let conversion = HashMap::from([('X', RPS::Rock), ('Y', RPS::Paper), ('Z', RPS::Scissors)]);

    let s: i32 = strat
        .iter()
        .map(move |(a, b)| (*a, *conversion.get(&b).unwrap()))
        .map(|(a, b)| score(a, b))
        .sum();

    println!("{}", s);

    let conversion = HashMap::from([('X', XYZ::Lose), ('Y', XYZ::Draw), ('Z', XYZ::Win)]);

    let s: i32 = strat
        .iter()
        .map(|(a, b)| (*a, *conversion.get(&b).unwrap()))
        .map(|(a, outcome)| {
            let m = fix(outcome, a);
            score(a, m)
        })
        .sum();
        
    println!("{}", s);
}

