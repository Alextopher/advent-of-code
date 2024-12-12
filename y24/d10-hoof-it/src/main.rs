use std::collections::{HashMap, HashSet};

use aoc::{input_str, stringstuff::CharExt, time};

struct Map {
    data: HashMap<(usize, usize), u8>,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.data.get(&(x, y)).copied()
    }

    fn diff(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Option<i8> {
        debug_assert_eq!(x1.abs_diff(x2) + y1.abs_diff(y2), 1);
        self.get(x2, y2)
            .and_then(|p2| self.get(x1, y1).map(|p1| (p1, p2)))
            .map(|(p1, p2)| p2 as i8 - p1 as i8)
    }

    pub fn part1_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        [
            x.checked_sub(1).map(|x| (x, y)),
            y.checked_sub(1).map(|y| (x, y)),
            Some((x + 1, y)),
            Some((x, y + 1)),
        ]
        .into_iter()
        .flatten()
        .filter(|p2| self.diff((x, y), *p2) == Some(-1))
        .collect()
    }
}

fn part1(map: &Map) -> usize {
    // BFS from the 9s figuring out how many peaks each tile can reach. After the fact check the scores for h=0
    let mut reachable_peaks: HashMap<_, _> =
        map.data.keys().map(|k| (*k, HashSet::new())).collect();
    let mut layer: HashSet<_> = map
        .data
        .iter()
        .filter_map(|(k, v)| (*v == 9).then_some(*k))
        .collect();

    layer.iter().for_each(|&p| {
        let set = reachable_peaks.get_mut(&p).unwrap();
        set.insert(p);
    });

    let mut next_layer = HashSet::new();
    while !layer.is_empty() {
        debug_assert!(next_layer.is_empty());
        for (x, y) in layer.drain() {
            let peaks = reachable_peaks.get(&(x, y)).unwrap().clone();
            map.part1_neighbors(x, y).into_iter().for_each(|n| {
                let next_peaks = reachable_peaks.get_mut(&n).unwrap();
                next_peaks.extend(peaks.iter());
                next_layer.insert(n);
            });
        }

        std::mem::swap(&mut layer, &mut next_layer);
    }

    map.data
        .iter()
        .filter_map(|(k, height)| {
            (*height == 0)
                .then_some(k)
                .and_then(|p| reachable_peaks.get(p))
        })
        .map(|peaks| peaks.len())
        .sum()
}

fn part2(map: &Map) -> usize {
    // BFS from the 9s figuring out how many peaks each tile can reach. After the fact check the scores for h=0
    let mut trails: HashMap<_, _> = map.data.keys().map(|k| (*k, 0)).collect();
    let mut layer: HashSet<_> = map
        .data
        .iter()
        .filter_map(|(k, v)| (*v == 9).then_some(*k))
        .collect();

    layer.iter().for_each(|&p| {
        trails.insert(p, 1);
    });

    let mut next_layer = HashSet::new();
    while !layer.is_empty() {
        debug_assert!(next_layer.is_empty());
        for (x, y) in layer.drain() {
            let paths = *trails.get(&(x, y)).unwrap();
            map.part1_neighbors(x, y).into_iter().for_each(|n| {
                *trails.get_mut(&n).unwrap() += paths;
                next_layer.insert(n);
            });
        }

        std::mem::swap(&mut layer, &mut next_layer);
    }

    map.data
        .iter()
        .filter_map(|(k, height)| (*height == 0).then_some(k).and_then(|p| trails.get(p)))
        .sum()
}

fn parse(input: &str) -> Map {
    let mut data = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.chars().map(|b| b.digit_to_num::<u8>()).enumerate() {
            data.insert((x, y), b);
        }
    }
    Map { data }
}

fn main() {
    let input = input_str!(2024, 10);

    let map = time("Parsed", || parse(input));
    let part1 = time("Part 1", || part1(&map));
    println!("Part 1: {}", part1);

    let part2 = time("Part 2", || part2(&map));
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example() {
        let input =
            "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732";

        let map = parse(input);
        assert_eq!(part1(&map), 36);
        assert_eq!(part2(&map), 81);
    }
}
