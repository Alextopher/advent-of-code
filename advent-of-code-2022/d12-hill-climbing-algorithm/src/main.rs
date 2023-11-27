// Finally! BFS

use std::collections::VecDeque;

#[derive(Debug)]
struct Map {
    cells: Vec<u8>,
    height: usize,
    width: usize,
}

impl Map {
    /// Parses a map from a string.
    ///
    /// Returns the map, the start position, and the end position.
    fn from(input: &str) -> (Map, (usize, usize), (usize, usize)) {
        let mut cells = Vec::new();

        let mut start = None;
        let mut end = None;

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let cell = match ch {
                    'S' => {
                        start = Some((x, y));
                        0
                    }
                    'E' => {
                        end = Some((x, y));
                        25
                    }
                    _ => ch as u8 - b'a',
                };
                cells.push(cell);
            }
        }

        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();

        assert_eq!(cells.len(), height * width);
        (
            Self {
                cells,
                height,
                width,
            },
            start.unwrap(),
            end.unwrap(),
        )
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    // Returns the height of a given cell
    fn get(&self, x: usize, y: usize) -> u8 {
        self.cells[self.get_index(x, y)]
    }

    // Returns the neighbors of a given cell
    fn neighbors(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
        let mut neighbors = [None; 4];

        if x > 0 {
            neighbors[0] = Some((x - 1, y));
        }

        if x < self.width - 1 {
            neighbors[1] = Some((x + 1, y));
        }

        if y > 0 {
            neighbors[2] = Some((x, y - 1));
        }

        if y < self.height - 1 {
            neighbors[3] = Some((x, y + 1));
        }

        neighbors
    }

    // Returns the neighbors of a given cell that are reachable
    fn reachable_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let height = self.get(x, y);

        self.neighbors(x, y)
            .iter()
            .flatten()
            .cloned()
            .filter(|&(x, y)| self.get(x, y) <= height + 1)
            .collect()
    }

    // Returns the neighbors of a given cell that are reachable, reversed.
    fn reachable_neighbors_reversed(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let height = self.get(x, y);

        self.neighbors(x, y)
            .iter()
            .flatten()
            .cloned()
            .filter(|&(x, y)| self.get(x, y) >= height - 1)
            .collect()
    }

    // Finds the length of the shortest path from S to E
    fn length_between(&self, start: (usize, usize), end: (usize, usize)) -> Option<usize> {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.cells.len()];
        let mut parents = vec![None; self.cells.len()];

        queue.push_back(start);

        while let Some((row, col)) = queue.pop_front() {
            if (row, col) == end {
                break;
            }

            for (r, c) in self.reachable_neighbors(row, col) {
                let index = self.get_index(r, c);
                if !visited[index] {
                    visited[index] = true;
                    parents[index] = Some((row, col));
                    queue.push_back((r, c));
                }
            }
        }

        let mut length = 0;
        let mut current = end;
        while current != start {
            length += 1;
            current = parents[self.get_index(current.0, current.1)].unwrap();
        }

        Some(length)
    }

    // Finds the length of the shortest path from S to _any_ cell that matches the predicate
    fn length_to_any<F>(&self, start: (usize, usize), predicate: F) -> Option<usize>
    where
        F: Fn(usize, usize) -> bool,
    {
        let mut queue = VecDeque::new();
        let mut visited = vec![false; self.cells.len()];
        let mut parents = vec![None; self.cells.len()];
        let mut end = None;

        queue.push_back(start);

        while let Some((x, y)) = queue.pop_front() {
            if predicate(x, y) {
                end = Some((x, y));
                break;
            }

            for (r, c) in self.reachable_neighbors_reversed(x, y) {
                let index = self.get_index(r, c);
                if !visited[index] {
                    visited[index] = true;
                    parents[index] = Some((x, y));
                    queue.push_back((r, c));
                }
            }
        }

        let mut length = 1;
        let mut current = end.unwrap();
        while current != start {
            length += 1;
            current = parents[self.get_index(current.0, current.1)].unwrap();
        }

        Some(length)
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.chunks(self.width) {
            for cell in row {
                write!(f, "{}", (cell + b'a') as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(map: &Map, start: (usize, usize), end: (usize, usize)) -> usize {
    map.length_between(start, end).unwrap()
}

fn part2(map: &Map, end: (usize, usize)) -> usize {
    map.length_to_any(end, |x, y| map.get(x, y) == 1).unwrap()
}

fn main() {
    // Read the input file
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Parse the map
    let (map, start, end) = Map::from(&input);

    // Part 1
    println!("Part 1: {}", part1(&map, start, end));

    // Part 2
    println!("Part 2: {}", part2(&map, end));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

        let (map, start, end) = Map::from(input);

        assert_eq!(map.get(start.0, start.1), 0);
        assert_eq!(map.get(end.0, end.1), 25);

        // Part 1
        assert_eq!(map.length_between(start, end), Some(31));

        // Part 2
        assert_eq!(map.length_to_any(end, |x, y| map.get(x, y) == 1), Some(29));
    }
}
