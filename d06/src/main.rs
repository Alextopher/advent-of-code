use std::collections::VecDeque;

use aoc::*;
use inpt::Inpt;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Inpt)]
#[inpt(regex = r"")]
struct Input {}

fn main() {
    let line = get_lines("input.txt").next().unwrap();

    // find the index of first window of 4 unique characters
    let ans = line
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .enumerate()
        .find(|(_, (a, b, c, d))| a != b && a != c && a != d && b != c && b != d && c != d)
        .unwrap();

    println!("{:?}, {:?}", ans.0 + 4, ans.1);

    // now we need groups of 14 unique characters and a different strategy
    let mut iter = line.chars();
    let mut window: VecDeque<_> = iter.by_ref().take(14).collect();

    let mut i = 15;
    while let Some(c) = iter.next() {
        window.pop_front();
        window.push_back(c);

        if window.iter().all_unique() {
            println!("{} {:?}", i, window);
            break;
        }
        i += 1
    }
}
