use aoc::*;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let ans: Vec<i32> = reader
        .lines()
        .map(|l| l.unwrap())
        .group_by(String::is_empty)
        .into_iter()
        .filter_map(|(k, v)| if k { None } else { Some(v) })
        .map(|g| g.into_iter().map(|c| c.parse::<i32>().unwrap()).sum())
        .selection_sorted()
        .rev()
        .take(3)
        .collect();

    println!("{}", ans[0]);
    println!("{}", ans.iter().sum::<i32>());
}
