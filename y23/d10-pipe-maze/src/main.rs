// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.
// S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

use std::collections::{HashMap, HashSet};

use aoc::input_str;
use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '|' => Cell::Vertical,
            '-' => Cell::Horizontal,
            'L' => Cell::NorthEast,
            'J' => Cell::NorthWest,
            '7' => Cell::SouthWest,
            'F' => Cell::SouthEast,
            '.' => Cell::Ground,
            'S' => Cell::Start,
            _ => panic!("Invalid cell"),
        }
    }
}

#[derive(Debug)]
struct Graph {
    // Top of the graph is (0, 0)
    nodes: HashMap<(usize, usize), Cell>,
    start: (usize, usize),
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            start: (0, 0),
        }
    }

    fn add_node(&mut self, x: usize, y: usize, cell: Cell) {
        if cell == Cell::Start {
            self.start = (x, y);
        }

        self.nodes.insert((x, y), cell);
    }

    fn get_node(&self, x: usize, y: usize) -> Option<&Cell> {
        self.nodes.get(&(x, y))
    }

    fn connected_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self.get_node(x, y) {
            Some(Cell::Vertical) => vec![(x, y - 1), (x, y + 1)],
            Some(Cell::Horizontal) => vec![(x - 1, y), (x + 1, y)],
            Some(Cell::NorthEast) => vec![(x, y - 1), (x + 1, y)],
            Some(Cell::NorthWest) => vec![(x, y - 1), (x - 1, y)],
            Some(Cell::SouthWest) => vec![(x, y + 1), (x - 1, y)],
            Some(Cell::SouthEast) => vec![(x, y + 1), (x + 1, y)],
            Some(Cell::Ground) => vec![],
            Some(Cell::Start) => vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)],
            None => vec![],
        }
    }

    // Undirected graph cycle detection
    fn find_cycle(&self) -> Vec<(usize, usize)> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut cycle: Vec<(usize, usize)> = Vec::new();
        let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

        fn dfs_cycle(
            node: (usize, usize),
            parent_node: Option<(usize, usize)>,
            graph: &Graph,
            visited: &mut HashSet<(usize, usize)>,
            parent: &mut HashMap<(usize, usize), (usize, usize)>,
            cycle: &mut Vec<(usize, usize)>,
        ) -> bool {
            visited.insert(node);
            parent.insert(node, parent_node.unwrap_or(node));

            for &neighbor in graph.connected_neighbors(node.0, node.1).iter() {
                if !visited.contains(&neighbor) {
                    if dfs_cycle(neighbor, Some(node), graph, visited, parent, cycle) {
                        cycle.push(neighbor);
                        return true;
                    }
                } else if parent.get(&node) != Some(&neighbor) {
                    cycle.push(neighbor);
                    cycle.push(node);
                    return true;
                }
            }

            false
        }

        for &node in self.nodes.keys() {
            if !visited.contains(&node)
                && dfs_cycle(node, None, self, &mut visited, &mut parent, &mut cycle)
            {
                cycle.reverse();
                return cycle;
            }
        }

        cycle
    }
}

impl std::fmt::Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x) = self
            .nodes
            .keys()
            .map(|(x, _)| x)
            .copied()
            .minmax()
            .into_option()
            .unwrap();

        let (min_y, max_y) = self
            .nodes
            .keys()
            .map(|(_, y)| y)
            .copied()
            .minmax()
            .into_option()
            .unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                match self.get_node(x, y) {
                    Some(Cell::Vertical) => write!(f, "|")?,
                    Some(Cell::Horizontal) => write!(f, "-")?,
                    Some(Cell::NorthEast) => write!(f, "L")?,
                    Some(Cell::NorthWest) => write!(f, "J")?,
                    Some(Cell::SouthWest) => write!(f, "7")?,
                    Some(Cell::SouthEast) => write!(f, "F")?,
                    Some(Cell::Ground) => write!(f, ".")?,
                    Some(Cell::Start) => write!(f, "S")?,
                    None => panic!("Invalid cell"),
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            graph.add_node(x, y, Cell::from_char(c));
        }
    }
    graph
}

fn shoelace_formula(points: &[(i64, i64)]) -> i64 {
    let n = points.len();
    let mut sum = 0;

    for i in 0..n {
        let j = (i + 1) % n;
        sum += points[i].0 * points[j].1 - points[j].0 * points[i].1;
    }

    sum = sum.abs();
    sum / 2
}

fn main() {
    let input = input_str!(2023, 10);
    let graph = parse_graph(input);

    println!("{}", graph);

    // Run the cycle finder
    let cycle = graph.find_cycle();

    println!("{}", cycle.len() / 2);

    // Find the area of the cycle
    let points: Vec<(i64, i64)> = cycle
        .iter()
        .cloned()
        .map(|(x, y)| (x as i64, y as i64))
        .collect();

    let area = shoelace_formula(&points);
    let length = cycle.len() as i64 / 2;
    let result = area - length + 1;

    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_2() {
        let input = ".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ...";
        let graph = parse_graph(input);
        let cycle = graph.find_cycle();

        println!("{}", cycle.len() / 2);

        // shoelace formula
        let points: Vec<(i64, i64)> = cycle
            .iter()
            .cloned()
            .map(|(x, y)| (x as i64, y as i64))
            .collect();

        let area = shoelace_formula(&points);

        println!("{}", area);

        println!("{}", area - (cycle.len() / 2) as i64 + 1);
    }
}
