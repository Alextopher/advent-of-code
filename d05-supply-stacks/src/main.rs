use std::collections::VecDeque;

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

fn main() {
    let mut lines = get_lines("input.txt");

    let mut stacks_part1: Vec<VecDeque<char>> = (0..9).map(|_| VecDeque::new()).collect();
    let mut stacks_part2: Vec<VecDeque<char>> = (0..9).map(|_| VecDeque::new()).collect();

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
                    stacks_part2[i / 4].push_front(c);
                }
            });
        });

    // move 4 from 9 to 6
    // move 7 from 2 to 5
    // move 3 from 5 to 2
    // move 2 from 2 to 1
    let instructions = lines
        .map(|line| inpt::inpt::<Input>(&line).unwrap())
        .collect_vec();

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
    println!("{}", part1);

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
    println!("{}", part2);
}
