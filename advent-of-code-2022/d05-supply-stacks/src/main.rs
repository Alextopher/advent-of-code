use std::{collections::VecDeque, time::Instant};

use aoc::*;
use inpt::Inpt;
use itertools::Itertools;

// move 4 from 9 to 6
#[derive(Debug, Clone, Copy, PartialEq, Eq, Inpt)]
#[inpt(regex = r"move (\d+) from (\d+) to (\d+)")]
struct Input {
    count: usize,
    from: usize,
    to: usize,
}

fn solution(content: &str, stacks: usize) -> (String, String) {
    let mut time = Instant::now();

    let mut lines = aoc::get_lines(content).peekable();
    println!("Read input {:?}", time.elapsed());
    time = Instant::now();

    let mut stacks_part1: Vec<VecDeque<char>> = (0..stacks).map(|_| VecDeque::new()).collect();

    //             [C]         [N] [R]
    // [J] [T]     [H]         [P] [L]
    // [F] [S] [T] [B]         [M] [D]
    // [C] [L] [J] [Z] [S]     [L] [B]
    // [N] [Q] [G] [J] [J]     [F] [F] [R]
    // [D] [V] [B] [L] [B] [Q] [D] [M] [T]
    // [B] [Z] [Z] [T] [V] [S] [V] [S] [D]
    // [W] [P] [P] [D] [G] [P] [B] [P] [V]
    //  1   2   3   4   5   6   7   8   9
    lines
        .by_ref()
        .map_while(|line| if line.is_empty() { None } else { Some(line) })
        .for_each(|line| {
            line.chars().enumerate().for_each(|(i, c)| {
                // we only care about positions 1, 5, 9, etc.
                if i % 4 == 1 && c != ' ' {
                    stacks_part1[i / 4].push_front(c);
                }
            });
        });

    // Clone the stacks for part 2
    let mut stacks_part2 = stacks_part1.clone();

    let instructions = lines
        .map(|line| inpt::inpt::<Input>(&line).unwrap())
        .collect_vec();

    println!("Parse input {:?}", time.elapsed());

    instructions.iter().for_each(|Input { count, from, to }| {
        let (from, to) = stacks_part1.get_mut_2(from - 1, to - 1);
        let idx = from.len() - count;

        to.extend(from.drain(idx..).rev());
    });

    // skim off the top of each stack
    let part1 = stacks_part1
        .iter()
        .map(|s| s.back().unwrap())
        .join("")
        .to_string();

    println!("{} {:?}", part1, time.elapsed());

    instructions.iter().for_each(|Input { count, from, to }| {
        let (from, to) = stacks_part2.get_mut_2(from - 1, to - 1);
        let idx = from.len() - count;

        to.extend(from.drain(idx..));
    });

    // skim off the top of each stack
    let part2 = stacks_part2
        .iter()
        .map(|s| s.back().unwrap())
        .join("")
        .to_string();

    println!("{} {:?}", part2, time.elapsed());

    (part1, part2)
}

fn main() {
    solution(aoc::get_input!(2022, 5), 9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            solution(aoc::get_input!(2022, 5), 9),
            ("LBLVVTVLP".to_string(), "TPFFBDRJD".to_string())
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            solution(include_str!("../example.txt"), 3),
            ("CMZ".to_string(), "MCD".to_string())
        );
    }
}
