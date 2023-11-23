use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

// Create a graph of the network
struct Node {
    id: String,
    neighbors: Vec<String>,
    rate: usize,
}

impl From<&str> for Node {
    fn from(input: &str) -> Self {
        // "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
        let parts: Vec<&str> = input.split_whitespace().collect();

        let id = parts[1].to_string();
        let rate = parts[4]
            .trim_end_matches(';')
            .split('=')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let neighbors = parts[9..]
            .iter()
            .map(|s| s.trim_end_matches(','))
            .map(|s| s.to_string())
            .collect();

        Node {
            id,
            neighbors,
            rate,
        }
    }
}

// Performance is very important here, I need to reduce the size of the graph and make precomputations for the distances between valves
//
// Valves with rate=0 are not import, they will be removed
// The graph will be _dense_, every valve will be connected to every other valve
struct InitialGraph {
    nodes: HashMap<String, Node>,
}

impl InitialGraph {
    fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    fn is_neighbor(&self, from: &str, to: &str) -> bool {
        self.nodes[from].neighbors.iter().any(|n| n == to)
    }

    fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Returns the list of non-zero valves
    fn valves(&self) -> Vec<String> {
        let mut valves: Vec<_> = self
            .nodes
            .iter()
            .filter(|(_, node)| node.rate > 0)
            .map(|(id, _)| id.clone())
            .collect();

        valves.push("AA".to_string());

        valves.sort_unstable();

        valves
    }

    /// Builds the distance graph
    ///
    /// Floyd-Warshall algorithm
    fn build_distance_graph(&self) -> DistanceGraph {
        // Reduce the number of valves
        let valves = self.valves();
        let mut graph = DistanceGraph::new(valves.iter().map(|id| self.nodes[id].rate).collect());

        // Initialize the distances
        for (i, from) in valves.iter().enumerate() {
            for (j, to) in valves.iter().enumerate() {
                if i == j {
                    continue;
                }

                if self.is_neighbor(from, to) {
                    graph.distances[i * valves.len() + j] = 1;
                } else {
                    graph.distances[i * valves.len() + j] = usize::MAX;
                }
            }
        }

        // Floyd-Warshall algorithm
        for k in 0..valves.len() {
            for i in 0..valves.len() {
                for j in 0..valves.len() {
                    if graph.distance(i, k) == usize::MAX || graph.distance(k, j) == usize::MAX {
                        continue;
                    }

                    let new_distance = graph.distance(i, k) + graph.distance(k, j);

                    if new_distance < graph.distance(i, j) {
                        graph.distances[i * valves.len() + j] = new_distance;
                    }
                }
            }
        }

        graph
    }

    /// Prints the graph as dot
    ///
    /// The graph is undirected
    fn print_dot(&self) {
        println!("graph {{");

        for (id, node) in &self.nodes {
            for neighbor in &node.neighbors {
                println!("  {} -- {}", id, neighbor);
            }
        }

        println!("}}");
    }
}

#[derive(Debug)]
struct DistanceGraph {
    // Flow rate of each node
    nodes: Vec<usize>,
    // Square matrix: nodes x nodes
    distances: Vec<usize>,
}

impl DistanceGraph {
    fn new(nodes: Vec<usize>) -> Self {
        Self {
            distances: vec![0; nodes.len() * nodes.len()],
            nodes,
        }
    }

    fn distance(&self, from: usize, to: usize) -> usize {
        self.distances[from * self.nodes.len() + to]
    }

    /// Prints the graph as dot
    fn print_dot(&self) {
        println!("graph {{");

        for (i, from) in self.nodes.iter().enumerate() {
            for (j, to) in self.nodes.iter().enumerate().skip(i) {
                if i != j && self.distance(i, j) > 0 {
                    println!("  {} -- {} [label={}]", i, j, self.distance(i, j));
                }
            }
        }

        println!("}}");
    }

    // Returns all of the possible paths starting from the given node
    //
    // The weight is the maximum distance allowed
    fn paths(&self, start: usize, weight: usize) -> Vec<Vec<usize>> {
        let mut paths = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_back((vec![start], 0));

        while let Some((path, distance)) = queue.pop_front() {
            let &last = path.last().unwrap();

            let mut added = false;
            for node in 0..self.nodes.len() {
                if path.contains(&node) {
                    continue;
                }

                let mut new_path = path.clone();
                new_path.push(node);

                let new_distance = distance + self.distance(last, node) + 1;

                if new_distance <= weight {
                    queue.push_back((new_path, new_distance));
                    added = true;
                }
            }

            if !added {
                paths.push(path);
            }
        }

        paths
    }

    // Simulates a path
    fn simulate(&self, path: &[usize], mut time: usize) -> usize {
        let mut released = 0;
        let mut last = path[0];

        for &node in path.iter().skip(1) {
            // Decrease the time
            time -= self.distance(last, node) + 1;
            // Increase the released
            released += self.nodes[node] * time;
            // Update the last node
            last = node;
        }

        released
    }
}

fn part1(input: &str) -> usize {
    let mut graph = InitialGraph::new();

    for line in input.lines() {
        let node = Node::from(line);
        graph.add_node(node);
    }

    let graph = graph.build_distance_graph();

    graph.print_dot();

    todo!();

    let paths = graph.paths(0, 30);

    println!("{}", paths.len());

    paths
        .iter()
        .map(|path| graph.simulate(path, 30))
        .max()
        .unwrap()
}

fn main() {
    let input = include_str!("../input.txt");

    println!("{}", part1(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_from_str() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let node = Node::from(input);

        assert_eq!(node.id, "AA");
        assert_eq!(node.rate, 0);
        assert_eq!(node.neighbors, vec!["DD", "II", "BB"]);

        let input = "Valve BB has flow rate=13; tunnels lead to valves CC, AA";
        let node = Node::from(input);

        assert_eq!(node.id, "BB");
        assert_eq!(node.rate, 13);
        assert_eq!(node.neighbors, vec!["CC", "AA"]);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");

        assert_eq!(part1(input), 1651);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1944);
    }
}
