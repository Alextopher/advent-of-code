use std::collections::HashMap;

use aoc::{IterJunk, UnionFind, input_str};

fn dist(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> i64 {
    (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1) + (a.2 - b.2) * (a.2 - b.2)
}

fn part1(input: &str, n: usize) -> usize {
    // x,y,z
    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect::<Vec<_>>();

    // calculate the distance between every pair of points
    let mut distances: Vec<(i64, (usize, usize))> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = dist(&points[i], &points[j]);
            distances.push((dist, (i, j)));
        }
    }

    // sort the distances in ascending order
    distances.sort_unstable();

    let mut disjoint_set = UnionFind::new(points.len());

    // join the 1000 closest points
    distances.iter().take(n).for_each(|&(_, (i, j))| {
        disjoint_set.union(i, j);
    });

    // find the 3 largest groups
    let mut groups = HashMap::new();
    for i in 0..disjoint_set.len() {
        let group = disjoint_set.find(i);
        *groups.entry(group).or_insert(0) += 1;
    }

    let groups: Vec<_> = groups.into_iter().collect();
    groups
        .iter()
        .k_largest_by_key(3, |&(_, count)| count)
        .map(|(_, count)| count)
        .product()
}

fn part2(input: &str) -> Option<i64> {
    // x,y,z
    let points: Vec<(i64, i64, i64)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            (x, y, z)
        })
        .collect::<Vec<_>>();

    // calculate the distance between every pair of points
    let mut distances: Vec<(i64, (usize, usize))> = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = dist(&points[i], &points[j]);
            distances.push((dist, (i, j)));
        }
    }

    // sort the distances in ascending order
    distances.sort_unstable();

    let mut disjoint_set = UnionFind::new(points.len());

    // continue until all points are in the same group
    for (_, (i, j)) in distances {
        disjoint_set.union(i, j);
        if disjoint_set.num_groups() == 1 {
            let a = points[i];
            let b = points[j];
            return Some(a.0 * b.0);
        }
    }

    None
}

fn main() {
    let input = input_str!(2025, 8);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input, 1000));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input).unwrap());
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let example = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";
        assert_eq!(part1(example, 10), 40);
    }
}
