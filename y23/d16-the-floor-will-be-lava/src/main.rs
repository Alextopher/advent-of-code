use std::collections::{HashSet, VecDeque};

use aoc::input_str;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    ForwardSlash,
    BackSlash,
    VerticalBar,
    HorizontalBar,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::ForwardSlash,
            '\\' => Self::BackSlash,
            '|' => Self::VerticalBar,
            '-' => Self::HorizontalBar,
            _ => panic!("Invalid tile: {}", c),
        }
    }

    fn is_empty(&self) -> bool {
        *self == Self::Empty
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Grid {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut height = 0;
        let tiles = input
            .lines()
            .flat_map(|line| {
                height += 1;
                line.chars().map(Tile::from_char)
            })
            .collect::<Vec<_>>();
        let width = tiles.len() / height;

        Self {
            tiles,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y * self.width + x]
    }

    fn find_up(
        &self,
        visited: &mut HashSet<(usize, usize, Direction)>,
        x: usize,
        y: usize,
    ) -> Option<(usize, usize)> {
        for y in (0..y).rev() {
            let tile = self.get(x, y);
            if !tile.is_empty() {
                return Some((x, y));
            } else {
                visited.insert((x, y, Direction::Up));
            }
        }
        None
    }

    fn find_down(
        &self,
        visited: &mut HashSet<(usize, usize, Direction)>,
        x: usize,
        y: usize,
    ) -> Option<(usize, usize)> {
        for y in y + 1..self.height {
            let tile = self.get(x, y);
            if !tile.is_empty() {
                return Some((x, y));
            } else {
                visited.insert((x, y, Direction::Down));
            }
        }
        None
    }

    fn find_left(
        &self,
        visited: &mut HashSet<(usize, usize, Direction)>,
        x: usize,
        y: usize,
    ) -> Option<(usize, usize)> {
        for x in (0..x).rev() {
            let tile = self.get(x, y);
            if !tile.is_empty() {
                return Some((x, y));
            } else {
                visited.insert((x, y, Direction::Left));
            }
        }
        None
    }

    fn find_right(
        &self,
        visited: &mut HashSet<(usize, usize, Direction)>,
        x: usize,
        y: usize,
    ) -> Option<(usize, usize)> {
        for x in x + 1..self.width {
            let tile = self.get(x, y);
            if !tile.is_empty() {
                return Some((x, y));
            } else {
                visited.insert((x, y, Direction::Right));
            }
        }
        None
    }

    fn find_direction(
        &self,
        visited: &mut HashSet<(usize, usize, Direction)>,
        x: usize,
        y: usize,
        dir: Direction,
    ) -> Option<(usize, usize)> {
        match dir {
            Direction::Up => self.find_up(visited, x, y),
            Direction::Down => self.find_down(visited, x, y),
            Direction::Left => self.find_left(visited, x, y),
            Direction::Right => self.find_right(visited, x, y),
        }
    }

    /// Simulates beam propagation, starting at (0, 0) going right
    ///
    /// There is some potential for loops, so the "visited" set keeps track of
    /// Positions _and_ directions
    fn simulate(&self, x: usize, y: usize, dir: Direction) -> usize {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((x, y, dir));

        while let Some((x, y, dir)) = queue.pop_front() {
            if !visited.insert((x, y, dir)) {
                continue;
            }

            match self.get(x, y) {
                Tile::Empty => {
                    // Continue in the same direction
                    if let Some((x, y)) = self.find_direction(&mut visited, x, y, dir) {
                        queue.push_back((x, y, dir));
                    }
                }
                Tile::ForwardSlash => {
                    // '/'
                    let next_dir = match dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };

                    if let Some((x, y)) = self.find_direction(&mut visited, x, y, next_dir) {
                        queue.push_back((x, y, next_dir));
                    }
                }
                Tile::BackSlash => {
                    // '\'
                    let next_dir = match dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };

                    if let Some((x, y)) = self.find_direction(&mut visited, x, y, next_dir) {
                        queue.push_back((x, y, next_dir));
                    }
                }
                Tile::VerticalBar => {
                    // '|'
                    // If going left or right, the continue up and down. Otherwise pass through
                    match dir {
                        Direction::Up | Direction::Down => {
                            if let Some((x, y)) = self.find_direction(&mut visited, x, y, dir) {
                                queue.push_back((x, y, dir));
                            }
                        }
                        Direction::Left | Direction::Right => {
                            if let Some((x, y)) = self.find_up(&mut visited, x, y) {
                                queue.push_back((x, y, Direction::Up));
                            }
                            if let Some((x, y)) = self.find_down(&mut visited, x, y) {
                                queue.push_back((x, y, Direction::Down));
                            }
                        }
                    }
                }
                Tile::HorizontalBar => {
                    // '-'
                    // If going up or down, then continue left and right. Otherwise pass through
                    match dir {
                        Direction::Up | Direction::Down => {
                            if let Some((x, y)) = self.find_left(&mut visited, x, y) {
                                queue.push_back((x, y, Direction::Left));
                            }
                            if let Some((x, y)) = self.find_right(&mut visited, x, y) {
                                queue.push_back((x, y, Direction::Right));
                            }
                        }
                        Direction::Left | Direction::Right => {
                            if let Some((x, y)) = self.find_direction(&mut visited, x, y, dir) {
                                queue.push_back((x, y, dir));
                            }
                        }
                    }
                }
            }
        }

        // Count the number of unique positions visited
        visited
            .iter()
            .map(|(x, y, _)| (x, y))
            .collect::<HashSet<_>>()
            .len()
    }
}

fn part1(grid: &Grid) -> usize {
    grid.simulate(0, 0, Direction::Right)
}

fn part2(grid: &Grid) -> usize {
    let mut edges = Vec::new();
    for x in 0..grid.width {
        edges.push((x, 0, Direction::Down));
        edges.push((x, grid.height - 1, Direction::Up));
    }

    for y in 0..grid.height {
        edges.push((0, y, Direction::Right));
        edges.push((grid.width - 1, y, Direction::Left));
    }

    edges
        .into_par_iter()
        .map(|(x, y, dir)| grid.simulate(x, y, dir))
        .max()
        .unwrap()
}

fn main() {
    let input = input_str!(2023, 16);

    let time = std::time::Instant::now();
    let grid = Grid::new(input);
    println!("Time to parse: {:?}", time.elapsed());

    // Part 1: Check starting at (0, 0)
    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&grid));
    println!("Time to simulate: {:?}", time.elapsed());

    // Part 2: Check starting at all edges
    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(&grid));
    println!("Time to simulate: {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;

        let grid = Grid::new(input);
        assert_eq!(grid.simulate(0, 0, Direction::Right), 46);
    }
}
