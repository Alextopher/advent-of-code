use aoc::input_str;

const NEIGHBORS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn part1(map: &FxHashSet<(isize, isize)>) -> usize {
    map.iter()
        .filter(|(x, y)| {
            NEIGHBORS
                .iter()
                .map(|(dx, dy)| (*x + *dx, *y + *dy))
                .filter(|next| map.contains(next))
                .count()
                < 4
        })
        .count()
}

fn part2(mut map: FxHashSet<(isize, isize)>) -> usize {
    let initial_length = map.len();

    loop {
        // preallocate
        let mut new_map = FxHashSet::with_capacity_and_hasher(map.len(), Default::default());

        new_map.extend(map.iter().copied().filter(|(x, y)| {
            NEIGHBORS
                .iter()
                .map(|(dx, dy)| (*x + *dx, *y + *dy))
                .filter(|next| map.contains(next))
                .count()
                >= 4
        }));

        if new_map.len() == map.len() {
            break;
        }

        map = new_map;
    }

    initial_length - map.len()
}

// part2 implemented with an indexed priority queue
use priority_queue::PriorityQueue;
use rustc_hash::{FxHashMap, FxHashSet};

// LessThan4 is a wrapper around usize that implements Ord and PartialOrd by only comparing if either value is less than 4
// For example 4 < 5, 5 == 6, 1 == 2
#[derive(Clone, Copy, Debug)]
struct LessThan4(u8, bool);

impl PartialEq for LessThan4 {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl Eq for LessThan4 {}

impl PartialOrd for LessThan4 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LessThan4 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1)
    }
}

impl std::hash::Hash for LessThan4 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl LessThan4 {
    fn new(count: usize) -> Self {
        debug_assert!(count <= 8, "Count should be <= 8, got {}", count);
        LessThan4(count as u8, count < 4)
    }

    fn bool_value(&self) -> bool {
        self.1
    }

    fn decrement(&mut self) {
        self.0 = self.0.saturating_sub(1);
        self.1 = self.0 < 4;
    }
}

fn part2_pq(map: &FxHashSet<(isize, isize)>) -> usize {
    let initial_length = map.len();

    // build pq as ((x, y), neighbors)
    let mut pq = PriorityQueue::with_capacity(initial_length);
    pq.extend(map.iter().copied().map(|(x, y)| {
        let neighbors = NEIGHBORS
            .iter()
            .map(|(dx, dy)| (x + *dx, y + *dy))
            .filter(|next| map.contains(next))
            .count();

        ((x, y), LessThan4::new(neighbors))
    }));

    while let Some(((x, y), neighbors)) = pq.pop() {
        if !neighbors.bool_value() {
            break;
        }

        // (x, y) has been removed so subtract 1 from the count of neighbors
        for (dx, dy) in NEIGHBORS {
            pq.change_priority_by(&(x + dx, y + dy), |priority| priority.decrement());
        }
    }

    initial_length - pq.len() - 1
}

fn part2_stack(map: &FxHashSet<(isize, isize)>) -> usize {
    let mut stack = Vec::with_capacity(map.len());
    let mut waiting: FxHashMap<(isize, isize), usize> = FxHashMap::default();
    waiting.reserve(map.len());

    // Compute initial neighbor counts
    for &(x, y) in map.iter() {
        let mut n = 0;
        for (dx, dy) in NEIGHBORS {
            if map.contains(&(x + dx, y + dy)) {
                n += 1;
            }
        }

        if n < 4 {
            stack.push((x, y));
        } else {
            waiting.insert((x, y), n);
        }
    }

    let initial = map.len();

    while let Some((x, y)) = stack.pop() {
        for (dx, dy) in NEIGHBORS {
            let nx = x + dx;
            let ny = y + dy;

            if let Some(n) = waiting.get_mut(&(nx, ny)) {
                *n -= 1;
                if *n < 4 {
                    stack.push((nx, ny));
                    waiting.remove(&(nx, ny));
                }
            }
        }
    }

    initial - waiting.len()
}

fn make_map(input: &str) -> FxHashSet<(isize, isize)> {
    let mut map = FxHashSet::default();
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '@' {
                map.insert((x as isize, y as isize));
            }
        }
    }
    map
}

fn main() {
    let start = std::time::Instant::now();
    let map = make_map(input_str!(2025, 4));
    println!("Make Map: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(&map));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2 pq: {}", part2_pq(&map));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2 stack: {}", part2_stack(&map));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(map));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../example.txt");
        assert_eq!(part1(&make_map(input)), 13);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../example.txt");
        assert_eq!(part2(make_map(input)), 43);
    }
}
