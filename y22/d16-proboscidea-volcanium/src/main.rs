#![feature(portable_simd)]

use std::collections::HashMap;

// Create a graph of the network
struct Node {
    id: String,
    neighbors: Vec<String>,
    rate: u16,
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

// Precomputes the distances between all nodes
// Use Floyd-Warshall algorithm
fn floyd_warshall(nodes: &HashMap<String, Node>) -> HashMap<(String, String), usize> {
    let mut distances = HashMap::new();

    for node in nodes.values() {
        for neighbor in &node.neighbors {
            distances.insert((node.id.clone(), neighbor.clone()), 1);
        }
    }

    for node in nodes.keys() {
        distances.insert((node.clone(), node.clone()), 0);
    }

    const MAX: usize = usize::MAX / 2;

    for k in nodes.keys() {
        for i in nodes.keys() {
            for j in nodes.keys() {
                let ij = distances.get(&(i.clone(), j.clone())).unwrap_or(&MAX);
                let ik = distances.get(&(i.clone(), k.clone())).unwrap_or(&MAX);
                let kj = distances.get(&(k.clone(), j.clone())).unwrap_or(&MAX);

                if ik + kj < *ij {
                    distances.insert((i.clone(), j.clone()), ik + kj);
                }
            }
        }
    }

    distances
}
// Graph reduction (reduce the graph down to only have the nodes that we care about)
fn reduce(distances: &mut HashMap<(String, String), usize>, nodes: &[String]) {
    // Remove edges that don't lead to a node in the list
    distances.retain(|(a, b), _| nodes.contains(a) && nodes.contains(b));
}

// Convert the graph into a matrix (backed by a 1d vector)
fn to_graph(
    order: &[String],
    nodes: HashMap<String, Node>,
    distances: HashMap<(String, String), usize>,
) -> Graph {
    // Sort nodes by name
    let mut matrix = vec![0; order.len() * order.len()];

    for (i, a) in order.iter().enumerate() {
        for (j, b) in order.iter().enumerate() {
            let distance = distances.get(&(a.clone(), b.clone())).unwrap();
            matrix[i * order.len() + j] = *distance;
        }
    }

    let rates = order.iter().map(|id| nodes.get(id).unwrap().rate).collect();

    Graph::new(rates, matrix)
}

use std::simd::cmp::SimdPartialEq;
use std::simd::u8x16;

// The graph only has at most 16 nodes, which I can represent as 4 bits per node
// Since a path is at most 16 nodes long, I can represent it as a (16*4) 64 bit integer
//
// In addition to storing the path, I also store some metadata
//
// - The length (4 bits)
// - The remaining time (8 bits)
// - The reward (16 bits)
//
// I will raise the length to 8 bits to make it easier to work with
#[derive(Debug, Clone)]
struct Path {
    path: u8x16,
    length: u8,
    time: u16,
    reward: u16,
}

impl Path {
    fn new(time: u16) -> Self {
        Self {
            path: u8x16::splat(0xFF),
            length: 0,
            time,
            reward: 0,
        }
    }

    /// Checks if the path contains a node
    fn contains(&self, node: u8) -> bool {
        debug_assert!(node < 255);
        let pattern_vector = u8x16::splat(node);
        let mask = self.path.simd_eq(pattern_vector);
        mask.any()
    }

    /// Returns the last node visited by the path
    fn last(&self) -> u8 {
        debug_assert!(self.length > 0);
        let index = self.length - 1;
        self.path.as_array()[index as usize]
    }

    /// Tries to add a node to the path
    ///
    /// Returns true if the node was added, false if it ran out of time
    fn add(&mut self, node: u8, cost: u16, rate: u16) {
        debug_assert!(node < 255);
        debug_assert!(self.length < 16);

        // Check if this node is already present
        debug_assert!(!self.contains(node));

        self.path.as_mut_array()[self.length as usize] = node;
        self.length += 1;

        debug_assert!(self.time >= cost);
        self.time -= cost;
        self.reward += rate * self.time;
    }

    /// Returns an iterator of next paths that can be reached from this path
    fn next_paths(&self, graph: &Graph) -> Vec<Path> {
        let mut paths = Vec::with_capacity(graph.len());

        for node in 0..graph.len() {
            if self.contains(node as u8) {
                continue;
            }

            let cost = graph.get_distance(self.last() as usize, node) + 1;
            if cost as u16 > self.time {
                continue;
            }

            let mut path = self.clone();
            path.add(node as u8, cost as u16, graph.get_rate(node));
            paths.push(path);
        }

        paths
    }

    /// Returns the reward of the path
    fn get_reward(&self) -> u16 {
        self.reward
    }

    /// Returns the difference in length between this path and another path
    fn length_diff(&self, other: &Path) -> u8 {
        self.length.abs_diff(other.length)
    }
}

// Final reduced graph with precomputed distances
#[derive(Debug)]
struct Graph {
    rates: Vec<u16>,
    distances: Vec<usize>,
}

impl Graph {
    fn new(rates: Vec<u16>, distances: Vec<usize>) -> Self {
        Self { rates, distances }
    }

    fn len(&self) -> usize {
        self.rates.len()
    }

    fn get_distance(&self, a: usize, b: usize) -> usize {
        self.distances[a * self.rates.len() + b]
    }

    fn get_rate(&self, i: usize) -> u16 {
        self.rates[i]
    }

    /// Returns an upper bound on the possible future reward of this path
    fn get_upper_bound(&self, path: &Path) -> u16 {
        let mut time = path.time;
        let mut reward = path.reward;

        // Create a vector of tuples (node, rate)
        let mut nodes: Vec<(usize, u16)> = (0..self.len())
            .map(|node| (node, self.get_rate(node)))
            .collect();

        // Filter out nodes that are already in the path
        nodes.retain(|(node, _)| !path.contains(*node as u8));

        // Sort nodes by rate in descending order
        nodes.sort_by(|a, b| b.1.cmp(&a.1));

        for (_, rate) in nodes {
            reward += rate * time;
            time -= 1;

            if time == 0 {
                break;
            }
        }

        reward
    }

    fn part1(&self) -> u16 {
        // Assumptions:
        // - The start node is always 0
        // - The maximum length is 30
        // - Each city costs 1 just to visit plus the distance to reach it
        // - If you can make a valid path longer while still keeping to valid, it will always be better than its shorter parent

        let mut max = 0;

        // A double buffer to hold the current and next paths
        let mut working = vec![];
        let mut next: Vec<Path> = vec![];

        // Start with the first node
        let mut path = Path::new(30);
        path.add(0, 0, 0);
        working.push(path);

        loop {
            println!("Working: {}", working.len());

            // For each path, try to add a node to it
            for path in working.drain(..) {
                let children = path.next_paths(self);

                if children.is_empty() {
                    if path.get_reward() > max {
                        max = path.get_reward();
                    }
                } else {
                    next.extend(children.into_iter());
                }
            }

            // If there are no new paths, we are done
            if next.is_empty() {
                break;
            }

            // Swap the buffers
            std::mem::swap(&mut working, &mut next);
        }

        max
    }

    // Part 2 is similar, but there are 2 agents running simultaneously and they mutually cannot visit the same node
    fn part2(&self) -> u16 {
        let mut max = 0;

        // A double buffer to hold the current and next paths
        let mut working: Vec<(Path, Path)> = vec![];
        let mut next: Vec<(Path, Path)> = vec![];

        // Start with the first node
        let mut path = Path::new(26);
        path.add(0, 0, 0);

        working.push((path.clone(), path));

        loop {
            println!("Working: {}", working.len());

            // For each path, try to add nodes to it
            //
            // path1 and path2 much be disjoint past the first node
            for (path1, path2) in working.drain(..) {
                let mut added = false;

                let children1 = path1.next_paths(self);
                let children2 = path2.next_paths(self);

                if children1.is_empty() && children2.is_empty() {
                    if path1.get_reward() + path2.get_reward() > max {
                        max = path1.get_reward() + path2.get_reward();
                    }
                    continue;
                }

                // Calculate the upper bound for each path
                let ub1 = self.get_upper_bound(&path1);
                let ub2 = self.get_upper_bound(&path2);

                // (path1, children2)
                for child2 in &children2 {
                    if !path1.contains(child2.last())
                        && self.get_upper_bound(child2) + ub1 > max
                        && child2.length_diff(&path2) < 1
                    {
                        next.push((path1.clone(), child2.clone()));
                        added = true;
                    }
                }

                // (children1, path2)
                for child1 in &children1 {
                    if !path2.contains(child1.last())
                        && self.get_upper_bound(child1) + ub2 > max
                        && child1.length_diff(&path1) < 1
                    {
                        next.push((child1.clone(), path2.clone()));
                        added = true;
                    }
                }

                // (children1 x children2)
                for child1 in children1 {
                    for child2 in children2.clone() {
                        if !child1.contains(child2.last())
                            && !child2.contains(child1.last())
                            && self.get_upper_bound(&child1) + self.get_upper_bound(&child2) > max
                            && child1.length_diff(&child2) < 1
                        {
                            next.push((child1.clone(), child2.clone()));
                            added = true;
                        }
                    }
                }

                if !added && path1.get_reward() + path2.get_reward() > max {
                    max = path1.get_reward() + path2.get_reward();
                }
            }

            // If there are no new paths, we are done
            if next.is_empty() {
                break;
            }

            // Swap the buffers
            std::mem::swap(&mut working, &mut next);
        }

        max
    }
}

fn make_graph(input: &str) -> Graph {
    let nodes: HashMap<String, Node> = input
        .lines()
        .map(Node::from)
        .map(|n| (n.id.clone(), n))
        .collect();

    let mut distances = floyd_warshall(&nodes);

    // We only care about "AA" and nodes that have positive flow rates
    let mut order = nodes
        .iter()
        .filter(|(_, node)| node.rate > 0)
        .map(|(id, _)| id.clone())
        .collect::<Vec<_>>();
    order.push("AA".to_string());
    order.sort();

    // Remove useless nodes. Thus reducing the graph to a smaller number of nodes
    reduce(&mut distances, &order);

    to_graph(&order, nodes, distances)
}

fn main() {
    let g = make_graph(aoc::input_str!(2022, 16));

    let start = std::time::Instant::now();
    println!("Part 1: {}", g.part1());
    println!("Time: {}ms", start.elapsed().as_millis());

    let start = std::time::Instant::now();
    println!("Part 2: {}", g.part2());
    println!("Time: {}ms", start.elapsed().as_millis());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let node = Node::from(input);

        assert_eq!(node.id, "AA");
        assert_eq!(node.rate, 0);
        assert_eq!(node.neighbors, vec!["DD", "II", "BB"]);
    }

    // Test path contains code
    #[test]
    fn test_path() {
        let mut path = Path::new(30);

        println!("{:?}", path);
        for i in 0..16 {
            assert!(!path.contains(i));
        }

        path.add(0, 0, 0);
        println!("{:?}", path);
        for i in 0..16 {
            assert_eq!(path.contains(i), i == 0);
        }

        path.add(1, 0, 0);
        println!("{:?}", path);
        for i in 0..16 {
            assert_eq!(path.contains(i), i == 0 || i == 1);
        }

        path.add(4, 0, 0);
        println!("{:?}", path);
        for i in 0..16 {
            assert_eq!(path.contains(i), i == 0 || i == 1 || i == 4);
        }
    }

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        let graph = make_graph(input);
        assert_eq!(graph.part1(), 1651);
        assert_eq!(graph.part2(), 1707);
    }

    #[test]
    fn test_part1() {
        let input = aoc::input_str!(2022, 16);
        let graph = make_graph(input);
        assert_eq!(graph.part1(), 1944);
    }

    // #[test]
    // fn test_part2() {
    //     let input = aoc::input_str!(2022, 16);
    //     let graph = make_graph(input);
    //     assert_eq!(graph.part2(), 2679);
    // }
}
