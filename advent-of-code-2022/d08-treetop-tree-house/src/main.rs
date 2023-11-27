use std::time::Instant;

use aoc::{get_input, get_lines};
use itertools::Itertools;

#[allow(clippy::needless_range_loop)]

fn solution(content: &str) -> (usize, i32) {
    let mut time = Instant::now();

    let lines = get_lines(content);
    println!("Read file {:?}", time.elapsed());
    time = Instant::now();

    // need to read a square input
    let mut forest: Vec<Vec<(i32, bool, i32)>> = lines
        .map(|line| {
            line.chars()
                .map(|c| (c as i32 - '0' as i32, false, 0))
                .collect()
        })
        .collect();

    let width = forest[0].len();
    let height = forest.len();

    println!("Parse file {:?}", time.elapsed());
    time = Instant::now();

    for x in 0..width {
        let mut last = -1;
        for y in 0..height {
            if forest[y][x].0 > last {
                last = forest[y][x].0;
                forest[y][x].1 = true;
            }
        }

        // loop backwards
        last = -1;
        for y in (0..height).rev() {
            if forest[y][x].0 > last {
                last = forest[y][x].0;
                forest[y][x].1 = true;
            }
        }
    }

    // other way
    for y in 0..height {
        let mut last = -1;
        for x in 0..width {
            if forest[y][x].0 > last {
                last = forest[y][x].0;
                forest[y][x].1 = true;
            }
        }

        // loop backwards
        last = -1;
        for x in (0..width).rev() {
            if forest[y][x].0 > last {
                last = forest[y][x].0;
                forest[y][x].1 = true;
            }
        }
    }

    // count the number of visible trees
    let count = forest
        .iter()
        .map(|row| row.iter().filter(|(_, visible, _)| *visible).count())
        .sum::<usize>();

    println!("{} {:?}", count, time.elapsed());
    time = Instant::now();

    // for each tree
    for (x, y) in (1..width - 1).cartesian_product(1..height - 1) {
        // count visible trees in each direction
        let h = forest[y][x].0;

        // Count the number of trees that can be seen from the current tree
        let mut up = 0;
        for y2 in (0..y).rev() {
            if forest[y2][x].0 >= h {
                up += 1;
                break;
            }
            up += 1;
        }

        let mut down = 0;
        for y2 in y + 1..height {
            if forest[y2][x].0 >= h {
                down += 1;
                break;
            }
            down += 1;
        }

        let mut left = 0;
        for x2 in (0..x).rev() {
            if forest[y][x2].0 >= h {
                left += 1;
                break;
            }
            left += 1;
        }

        let mut right = 0;
        for x2 in x + 1..width {
            if forest[y][x2].0 >= h {
                right += 1;
                break;
            }
            right += 1;
        }

        // println!("({}, {}) -> ({}, {}, {}, {})", x, y, up, down, left, right);
        forest[y][x].2 = up * down * left * right;
    }

    // print the max number of trees
    let best = forest
        .iter()
        .map(|row| row.iter().map(|(_, _, count)| count).max().unwrap())
        .max()
        .unwrap();

    println!("{} {:?}", best, time.elapsed());

    (count, *best)
}

fn main() {
    solution(get_input!(2022, 8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(solution(get_input!(2022, 8)), (1690, 535680));
    }

    #[test]
    fn test_example() {
        assert_eq!(solution(include_str!("../example.txt")), (21, 8));
    }
}
