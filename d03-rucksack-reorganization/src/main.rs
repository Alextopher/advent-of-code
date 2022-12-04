use std::ops::BitAnd;

use aoc::get_lines;
use itertools::Itertools;

fn get_mask(s: &str) -> u64 {
    s.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c as i32 - 'A' as i32 + 27
            } else {
                c as i32 - 'a' as i32 + 1
            }
        })
        .fold(0, |acc, b| acc | (1 << b))
}

fn main() {
    let ans: u32 = get_lines("input.txt")
        .map(|s| {
            let split = s.split_at(s.len() / 2);
            get_mask(split.0) & get_mask(split.1)
        })
        .map(u64::trailing_zeros)
        .sum();

    println!("{ans}");

    let ans: u32 = get_lines("input.txt")
        .map(|s| get_mask(&s))
        .chunks(3)
        .into_iter()
        .map(|k| k.fold(0, u64::bitand))
        .map(u64::trailing_zeros)
        .sum();

    println!("{ans}");
}
