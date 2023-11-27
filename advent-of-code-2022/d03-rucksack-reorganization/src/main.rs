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

fn solution(filename: &str) -> (u32, u32) {
    let part1: u32 = get_lines(filename)
        .map(|s| {
            let split = s.split_at(s.len() / 2);
            get_mask(split.0) & get_mask(split.1)
        })
        .map(u64::trailing_zeros)
        .sum();

    println!("Part 1: {}", part1);

    let part2: u32 = get_lines(filename)
        .map(|s| get_mask(&s))
        .chunks(3)
        .into_iter()
        .map(|k| k.fold(u64::MAX, u64::bitand))
        .map(u64::trailing_zeros)
        .sum();

    println!("Part 2: {}", part2);
    (part1, part2)
}

fn main() {
    solution("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(solution("input.txt"), (7691, 2508));
    }

    #[test]
    fn test_example() {
        assert_eq!(solution("example.txt"), (157, 70));
    }
}
