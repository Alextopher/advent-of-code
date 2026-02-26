use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use aoc::input_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

fn next_items(Item(v, dist): Item) -> [Item; 3] {
    let delta = v.dir.delta();

    [
        Item(
            Vertex {
                pos: (v.pos.0 + delta.0, v.pos.1 + delta.1),
                dir: v.dir,
            },
            dist + 1,
        ),
        Item(
            Vertex {
                pos: v.pos,
                dir: v.dir.rotate_left(),
            },
            dist + 1000,
        ),
        Item(
            Vertex {
                pos: v.pos,
                dir: v.dir.rotate_right(),
            },
            dist + 1000,
        ),
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vertex {
    pos: (i32, i32),
    dir: Direction,
}

impl Vertex {
    fn new(pos: (i32, i32), dir: Direction) -> Self {
        Vertex { pos, dir }
    }
}

#[derive(Debug, Clone, Copy)]
struct Item(Vertex, i32);

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.1.cmp(&self.1)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

fn part1(input: &str) -> i32 {
    let (mut start, mut end) = (None, None);

    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c);
            if c == 'S' {
                start = Some((x as i32, y as i32));
            } else if c == 'E' {
                end = Some((x as i32, y as i32));
            }
        }
    }

    let start = Vertex::new(start.unwrap(), Direction::East);
    let end = end.unwrap();

    let mut queue = BinaryHeap::<Item>::new();
    let mut dist = HashMap::<Vertex, i32>::new();

    dist.insert(start, 0);
    queue.push(Item(start, 0));

    while let Some(item) = queue.pop() {
        if item.0.pos == end {
            return item.1;
        }

        let dist_u = dist[&item.0];
        if dist_u < item.1 {
            continue;
        }

        for next in next_items(item) {
            if next.0.pos != item.0.pos && map.get(&next.0.pos).copied().unwrap() == '#' {
                continue;
            }

            if next.1 < dist.get(&next.0).copied().unwrap_or(i32::MAX) {
                dist.insert(next.0, next.1);
                queue.push(next);
            }
        }
    }

    panic!("No path found");
}

fn part2(input: &str, cost: i32) -> usize {
    let (mut start, mut end) = (None, None);

    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, y as i32), c);
            if c == 'S' {
                start = Some((x as i32, y as i32));
            } else if c == 'E' {
                end = Some((x as i32, y as i32));
            }
        }
    }

    let start = Vertex::new(start.unwrap(), Direction::East);
    let end = end.unwrap();

    let mut queue = BinaryHeap::<Item>::new();
    let mut dist = HashMap::<Vertex, i32>::new();
    let mut prev = HashMap::<Vertex, Vec<Vertex>>::new();

    dist.insert(start, 0);
    queue.push(Item(start, 0));

    while let Some(item) = queue.pop() {
        let dist_u = dist[&item.0];
        if dist_u < item.1 {
            continue;
        }

        for next in next_items(item) {
            if next.0.pos != item.0.pos && map.get(&next.0.pos).copied().unwrap() == '#' {
                continue;
            }

            if next.1 <= dist.get(&next.0).copied().unwrap_or(i32::MAX) {
                dist.insert(next.0, next.1);
                queue.push(next);
                prev.entry(next.0).or_default().push(item.0);
            }
        }
    }

    // expand prev starting from end
    let mut visited = HashSet::new();
    let mut stack: Vec<Vertex> = prev
        .keys()
        .filter(|&k| k.pos == end && dist[k] == cost)
        .copied()
        .collect();

    while let Some(vertex) = stack.pop() {
        if visited.insert(vertex)
            && let Some(next) = prev.get(&vertex)
        {
            stack.extend(next);
        }
    }

    visited.iter().map(|v| v.pos).collect::<HashSet<_>>().len()
}

fn main() {
    let input = input_str!(2024, 16);

    let start = std::time::Instant::now();
    let cost = part1(input);
    println!("Part 1: {}", cost);
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input, cost));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = include_str!("../example.txt");
        assert_eq!(part1(input), 7036);
    }

    #[test]
    fn test_part2_example() {
        let input = include_str!("../example.txt");
        let cost = part1(input);
        assert_eq!(part2(input, cost), 45);
    }
}
