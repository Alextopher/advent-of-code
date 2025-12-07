use std::collections::{HashMap, HashSet};

use aoc::input_str;
use binary_heap_plus::BinaryHeap;

fn find_shortest_path(walls: &HashSet<(u16, u16)>, width: u16) -> Option<u16> {
    // A* from (0,0) to (width,width), walls can't be traversed
    let start = (0, 0);
    let end = (width, width);

    // A* implementation
    let mut open_set = BinaryHeap::new_min();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();

    let heuristic = |pos: (u16, u16)| -> u16 { (end.0 - pos.0) + (end.1 - pos.1) };

    g_score.insert(start, 0u16);
    f_score.insert(start, heuristic(start));
    open_set.push((f_score[&start], start));

    while let Some((_, current)) = open_set.pop() {
        if current == end {
            return Some(g_score[&current]);
        }

        for neighbor in neighbors(current) {
            // Check bounds
            if neighbor.0 > width || neighbor.1 > width {
                continue;
            }

            // Check if it's a wall
            if walls.contains(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score[&current] + 1;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u16::MAX) {
                g_score.insert(neighbor, tentative_g_score);
                let f = tentative_g_score + heuristic(neighbor);
                f_score.insert(neighbor, f);
                open_set.push((f, neighbor));
            }
        }
    }

    None
}

fn part1(input: &str, n: usize, width: u16) -> Option<u16> {
    // each line is x,y
    let walls: HashSet<(u16, u16)> = input
        .lines()
        .take(n)
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    find_shortest_path(&walls, width)
}

fn part2(input: &str, width: u16) -> String {
    let coordinates: Vec<(u16, u16)> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    // Binary search to find the first blocking coordinate
    let mut left = 0;
    let mut right = coordinates.len();

    while left < right {
        let mid = (left + right) / 2;
        let walls: HashSet<(u16, u16)> = coordinates.iter().take(mid + 1).cloned().collect();

        if find_shortest_path(&walls, width).is_some() {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    let blocking_coord = coordinates[left];
    format!("{},{}", blocking_coord.0, blocking_coord.1)
}

fn neighbors((x, y): (u16, u16)) -> Vec<(u16, u16)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    neighbors.push((x + 1, y));
    neighbors.push((x, y + 1));
    neighbors
}

fn main() {
    let input = input_str!(2024, 18);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input, 1024, 70).unwrap());
    println!("Time: {:?}", time.elapsed());

    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(&input, 70));
    println!("Time: {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";
        assert_eq!(part1(&example, 12, 6), Some(22));
    }

    #[test]
    fn test_part2() {
        let example = "5,4\n4,2\n4,5\n3,0\n2,1\n6,3\n2,4\n1,5\n0,6\n3,3\n2,6\n5,1\n1,2\n5,5\n2,5\n6,5\n1,4\n0,4\n6,4\n1,1\n6,1\n1,0\n0,5\n1,6\n2,0";
        assert_eq!(part2(&example, 6), "6,1");
    }
}
