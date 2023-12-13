use std::iter::repeat;

use aoc::{input_str, IterJunk};
use itertools::Itertools;

// Read in 2d array of ash and rocks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Ash,
            '#' => Self::Rock,
            _ => panic!("Invalid tile"),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Tile>,
    transposed: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let mut height = 0;
        let tiles = s
            .lines()
            .map(|s| s.chars())
            .flat_map(|line| {
                height += 1;
                line
            })
            .map(Tile::from_char)
            .collect_vec();

        let width = tiles.len() / height;

        let transposed = (0..width)
            .flat_map(|x| (0..height).zip(repeat(x)))
            .map(|(y, x)| tiles[y * width + x])
            .collect_vec();

        Self {
            tiles,
            transposed,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * self.width + x]
    }

    fn get_row(&self, y: usize) -> &[Tile] {
        &self.tiles[y * self.width..(y + 1) * self.width]
    }

    fn get_col(&self, x: usize) -> &[Tile] {
        &self.transposed[x * self.height..(x + 1) * self.height]
    }

    fn detect_mirror(&self) -> usize {
        fn is_mirror((left, right): (&[Tile], &[Tile])) -> bool {
            left.iter().rev().zip(right.iter()).all(|(l, r)| l == r)
        }

        if let Some(col) = (1..self.width).find(|&col| {
            (0..self.height)
                .map(|r| self.get_row(r).split_at(col))
                .all(is_mirror)
        }) {
            return col;
        }

        if let Some(row) = (1..self.height).find(|&row| {
            (0..self.width)
                .map(|c| self.get_col(c).split_at(row))
                .all(is_mirror)
        }) {
            return 100 * row;
        }

        panic!("No mirror found");
    }

    // In part 2, there is a second mirror that is _almost_ correct.
    // It is off by exactly one tile
    fn detect_with_smudge(&self, smudges: usize) -> usize {
        // Helper function that returns how close 2 sets of tiles are to being a mirror
        // (returns how many tiles are different)
        fn mirror_diff(left: &[Tile], right: &[Tile]) -> usize {
            left.iter()
                .rev()
                .zip(right.iter())
                .filter(|(l, r)| l != r)
                .count()
        }

        // Find a column that is almost a mirror with a difference of 1
        if let Some(col) = (1..self.width).find(|&col| {
            (0..self.height)
                .map(|r| self.get_row(r).split_at(col))
                .map(|(left, right)| mirror_diff(left, right))
                .partial_sums()
                .find_or_last(|&x| x > smudges)
                .eq(&Some(smudges))
        }) {
            return col;
        }

        // Find a row that is almost a mirror with a difference of 1
        if let Some(row) = (1..self.height).find(|&row| {
            (0..self.width)
                .map(|c| self.get_col(c).split_at(row))
                .map(|(left, right)| mirror_diff(left, right))
                .partial_sums()
                .find_or_last(|&x| x > smudges)
                .eq(&Some(smudges))
        }) {
            return 100 * row;
        }

        panic!("No mirror found");
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.get(x, y) {
                    Tile::Ash => '.',
                    Tile::Rock => '#',
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let input = input_str!(2023, 13);

    // Each map uses multiple lines, maps are separated by a blank line
    let start = std::time::Instant::now();
    let maps = input.split("\n\n").map(Map::from_str).collect_vec();
    println!("Parsed: {:?}", start.elapsed());

    // Part 1 - sum the mirror values
    let start = std::time::Instant::now();
    let sum: usize = maps.iter().map(|m| m.detect_mirror()).sum();
    println!("Part 1: {}", sum);
    println!("Part 1: {:?}", start.elapsed());

    // Part 2 - sum the mirror values with the smudge
    let start = std::time::Instant::now();
    let sum: usize = maps.iter().map(|m| m.detect_with_smudge(1)).sum();
    println!("Part 2: {}", sum);
    println!("Part 2: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        let input = "#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#.";
        let map = Map::from_str(input);

        assert_eq!(map.width, 9);
        assert_eq!(map.height, 7);

        // Detect the mirror
        assert_eq!(map.detect_mirror(), 5);

        let input = "#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#";
        let map = Map::from_str(input);

        assert_eq!(map.width, 9);
        assert_eq!(map.height, 7);

        // Detect the mirror
        assert_eq!(map.detect_mirror(), 400);
    }

    #[test]
    fn test_regressions() {
        let input = "##.#.##...####.\n#####..#...##..\n##.##.#.#####.#\n.##.#..##..##..\n###..#.#####.#.\n###..#.#####.#.\n.##.#..##..##..\n##.##.#.#####.#\n#####..#...##..\n##.#.##..#####.\n...#...#....###\n##.#.###.###..#\n..#.###.#.#....\n####...#.#.#...\n.#..#....##.#.#\n....####.###.##\n....####.###.##";
        let map = Map::from_str(input);

        // Detect the mirror (row 4)
        println!("{} {}", map.width, map.height);
        assert_eq!(map.detect_mirror(), 1600);
    }
}
