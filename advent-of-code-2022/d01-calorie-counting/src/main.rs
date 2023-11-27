use aoc::{get_lines, iterstuff::IterJunk};
use itertools::Itertools;
use std::time::Instant;

fn solution(filename: &str) -> (i32, i32) {
    let lines = get_lines(filename);

    let time = Instant::now();

    let ans: Vec<i32> = lines
        .group_by(String::is_empty)
        .into_iter()
        .filter_map(|(k, v)| (!k).then_some(v))
        .map(|g| g.into_iter().map(|c| c.parse::<i32>().unwrap()).sum())
        .k_largest(3)
        .collect_vec();

    let part1 = ans.iter().max().unwrap();
    let part2 = ans.iter().sum::<i32>();
    println!("{:?}", time.elapsed());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    (*part1, part2)
}

fn main() {
    solution("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(solution("input.txt"), (69289, 205615));
    }

    #[test]
    fn test_example() {
        assert_eq!(solution("example.txt"), (24000, 45000));
    }
}
