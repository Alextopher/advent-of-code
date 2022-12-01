use aoc::{get_lines, iterstuff::IterJunk};
use itertools::Itertools;

fn main() {
    let ans: Vec<_> = get_lines("input.txt")
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
