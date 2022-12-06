use aoc::{get_lines, iterstuff::IterJunk};
use itertools::Itertools;
use std::time::Instant;

fn main() {
    let lines = get_lines("input.txt");

    let time = Instant::now();

    let ans: Vec<i32> = lines
        .group_by(String::is_empty)
        .into_iter()
        .filter_map(|(k, v)| (!k).then_some(v))
        .map(|g| g.into_iter().map(|c| c.parse::<i32>().unwrap()).sum())
        .k_largest(3)
        .collect_vec();

    let part1 = ans.iter().max().unwrap();
    let part2 = ans.iter().min().unwrap();
    println!("{:?}", time.elapsed());

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
