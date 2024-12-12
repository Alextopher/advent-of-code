use std::collections::HashSet;

use aoc::{input_str, time};

fn parse(input: &str) -> (HashSet<(usize, usize)>, (usize, usize)) {
    let map = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .filter_map(|(x, y, c)| c.eq(&'#').then_some((x, y)))
        .collect();

    let start = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| line.chars().enumerate().map(move |(x, c)| (x, y, c)))
        .find_map(|(x, y, c)| c.eq(&'^').then_some((x, y)))
        .unwrap();

    (map, start)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

fn step_forward(
    dir: Direction,
    (x, y): (usize, usize),
    bounds: (usize, usize),
) -> Option<(usize, usize)> {
    match dir {
        Direction::North => y.checked_sub(1).map(|y| (x, y)),
        Direction::East => (x + 1).le(&bounds.0).then_some((x + 1, y)),
        Direction::South => (y + 1).le(&bounds.1).then_some((x, y + 1)),
        Direction::West => x.checked_sub(1).map(|x| (x, y)),
    }
}

fn part1(map: &HashSet<(usize, usize)>, start: (usize, usize)) -> usize {
    // I only find the bounds here because I forgot to while parsing
    let bounds = map
        .iter()
        .copied()
        .reduce(|(mx, my), (x, y)| (mx.max(x), my.max(y)))
        .unwrap();
    let mut direction = Direction::North;
    let (mut x, mut y) = start;
    let mut visited = HashSet::new();

    while let Some(next) = step_forward(direction, (x, y), bounds) {
        visited.insert((x, y));
        if map.contains(&next) {
            direction = direction.turn_right();
        } else {
            (x, y) = next;
        }
    }

    visited.insert((x, y));
    visited.len()
}

fn part2(map: &HashSet<(usize, usize)>, start: (usize, usize)) -> usize {
    let bounds = map
        .iter()
        .copied()
        .reduce(|(mx, my), (x, y)| (mx.max(x), my.max(y)))
        .unwrap();

    let mut count = 0;
    for x in 0..=bounds.0 {
        for y in 0..=bounds.1 {
            let mut map = map.clone();
            map.insert((x, y));

            let mut visited = HashSet::new();
            let ((mut x, mut y), mut direction) = (start, Direction::North);
            while let Some(next) = step_forward(direction, (x, y), bounds) {
                if !visited.insert((x, y, direction)) {
                    count += 1;
                    break;
                }
                if map.contains(&next) {
                    direction = direction.turn_right();
                } else {
                    (x, y) = next
                }
            }
        }
    }

    count
}

fn main() {
    let input = input_str!(2024, 6);

    let (map, start) = time("Parsed", || parse(input));
    let part1 = time("Part 1", || part1(&map, start));
    println!("Part 1: {}", part1);

    // 674 low
    let part2 = time("Part 2", || part2(&map, start));
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_example() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let (map, start) = parse(input);
        assert_eq!(part1(&map, start), 41)
    }

    #[test]
    fn part2_example() {
        let input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let (map, start) = parse(input);
        assert_eq!(part2(&map, start), 6)
    }
}
