use aoc::*;
use itertools::Itertools;

fn main() {
    let line = get_lines("input.txt").next().unwrap();

    // find the index of first window of 4 unique characters
    let ans = line
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, w)| w.iter().all_unique())
        .unwrap()
        .0;

    println!("{}", ans + 4);

    // groups of 14 characters
    let ans = line
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, w)| w.iter().all_unique())
        .unwrap()
        .0;

    println!("{}", ans + 14);
}
