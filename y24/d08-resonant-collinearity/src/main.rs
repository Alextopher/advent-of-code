use std::{
    collections::{HashMap, HashSet},
    usize,
};

use aoc::{input_str, time};

fn parse(input: &str) -> (HashMap<u8, Vec<(isize, isize)>>, (usize, usize)) {
    let mut map = HashMap::<u8, Vec<_>>::new();
    let mut bounds = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            bounds.0 = bounds.0.max(x);
            bounds.1 = bounds.1.max(y);

            if b == '.' as u8 {
                continue;
            }

            map.entry(b).or_default().push((x as isize, y as isize));
        }
    }
    (map, bounds)
}

fn in_bounds((x, y): (isize, isize), (w, h): (usize, usize)) -> bool {
    x >= 0 && y >= 0 && x <= w as isize && y <= h as isize
}

fn part1(map: &HashMap<u8, Vec<(isize, isize)>>, bounds: (usize, usize)) -> usize {
    let mut antinodes = HashSet::new();
    for antennas in map.values() {
        // for each _pair_ of antennas
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let a1 = antennas[i];
                let a2 = antennas[j];

                let dx = a2.0 - a1.0;
                let dy = a2.1 - a1.1;

                let an1 = (a1.0 - dx, a1.1 - dy);
                let an2 = (a2.0 + dx, a2.1 + dy);

                if in_bounds(an1, bounds) {
                    antinodes.insert(an1);
                }

                if in_bounds(an2, bounds) {
                    antinodes.insert(an2);
                }
            }
        }
    }

    antinodes.len()
}

fn part2(map: &HashMap<u8, Vec<(isize, isize)>>, bounds: (usize, usize)) -> usize {
    let mut antinodes = HashSet::new();
    for antennas in map.values() {
        // for each _pair_ of antennas
        for i in 0..antennas.len() {
            for j in i + 1..antennas.len() {
                let a1 = antennas[i];
                let a2 = antennas[j];

                let dx = a2.0 - a1.0;
                let dy = a2.1 - a1.1;

                let mut an1 = (a1.0, a1.1);
                while in_bounds(an1, bounds) {
                    antinodes.insert(an1);
                    an1 = (an1.0 - dx, an1.1 - dy);
                }

                let mut an2 = (a2.0, a2.1);
                while in_bounds(an2, bounds) {
                    antinodes.insert(an2);
                    an2 = (an2.0 + dx, an2.1 + dy);
                }
            }
        }
    }

    antinodes.len()
}

fn main() {
    let input = input_str!(2024, 8);

    let (map, bounds) = time("Parsed", || parse(input));
    let part1 = time("Part 1", || part1(&map, bounds));
    println!("Part 1: {}", part1);

    let part2 = time("Part 2", || part2(&map, bounds));
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_example() {
        let input = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";
        let (map, bounds) = parse(input);
        assert_eq!(part1(&map, bounds), 14);
        assert_eq!(part2(&map, bounds), 34);

        let three_a = "..........\n..........\n..........\n....a.....\n........a.\n.....a....\n..........\n..........\n..........\n..........";
        let (map, bounds) = parse(three_a);
        assert_eq!(part1(&map, bounds), 4);
    }
}
