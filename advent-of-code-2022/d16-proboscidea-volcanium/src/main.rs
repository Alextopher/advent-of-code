use std::collections::HashMap;

type Distance = usize;
type Rate = u16;

// Create a graph of the network
struct Node {
    id: String,
    neighbors: Vec<String>,
    rate: Rate,
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

// Precompute the distances between all nodes
// Use Floyd-Warshall algorithm
fn floyd_warshall(nodes: &HashMap<String, Node>) -> HashMap<(String, String), Distance> {
    let mut distances = HashMap::new();

    for node in nodes.values() {
        for neighbor in &node.neighbors {
            distances.insert((node.id.clone(), neighbor.clone()), 1);
        }
    }

    for node in nodes.keys() {
        distances.insert((node.clone(), node.clone()), 0);
    }

    const MAX: Distance = Distance::MAX / 2;

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
fn reduce_graph(distances: &mut HashMap<(String, String), Distance>, nodes: &[String]) {
    // Remove edges that don't lead to a node in the list
    distances.retain(|(a, b), _| nodes.contains(a) && nodes.contains(b));
}

// Convert the graph into a matrix (backed by a 1d vector)
fn to_graph(
    order: &[String],
    nodes: HashMap<String, Node>,
    distances: HashMap<(String, String), Distance>,
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

// The graph only has at most 16 nodes, or 4 bits per node
// The length of the path is also at most 16, which means 16 * 4 = 64 bits
// We can use a u64 to represent the path

#[derive(Debug, Clone)]
struct Path {
    path: u64,
    cost: usize,
    length: usize,
}

impl Path {
    fn new() -> Self {
        Self {
            path: 0,
            cost: 0,
            length: 0,
        }
    }

    fn add(&mut self, node: usize, cost: usize) {
        debug_assert!(node < 16);
    }
}

// Final reduced graph with precomputed distances
#[derive(Debug)]
struct Graph {
    rates: Vec<Rate>,
    distances: Vec<Distance>,
}

impl Graph {
    fn new(rates: Vec<Rate>, distances: Vec<Distance>) -> Self {
        Self { rates, distances }
    }

    fn len(&self) -> usize {
        self.rates.len()
    }

    fn get_distance(&self, a: usize, b: usize) -> Distance {
        self.distances[a * self.rates.len() + b]
    }

    fn get_rate(&self, i: usize) -> Rate {
        self.rates[i]
    }

    fn part1(&self) -> usize {
        // Assumptions:
        // - The start node is always 0
        // - The maximum length is 30
        // - Each city costs 1 just to visit plus the distance to reach it
        // - If you can make a valid path longer while still keeping to valid, it will always be better than its shorter parent

        let mut paths: Vec<Path> = vec![];

        // A double buffer to hold the current and next paths
        let mut working = vec![];
        let mut next: Vec<Path> = vec![];

        // Start with the first node
        working.push(vec![0]);

        // for path in working.drain(..) {}
        todo!()
    }
}

fn part1(input: &str) -> usize {
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

    println!("{:?}", order);

    reduce_graph(&mut distances, &order);

    let g = to_graph(&order, nodes, distances);

    println!("{:?}", g.len());

    todo!()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_from() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB";
        let node = Node::from(input);

        assert_eq!(node.id, "AA");
        assert_eq!(node.rate, 0);
        assert_eq!(node.neighbors, vec!["DD", "II", "BB"]);
    }

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        assert_eq!(part1(input), 1651);
    }

    #[test]
    fn test_held_karp() {
        let rates = vec![0, 0, 0, 0];
        let distances = vec![
            0, 2, 9, 10, //
            1, 0, 6, 4, //
            15, 7, 0, 8, //
            6, 3, 12, 0, //
        ];

        let graph = Graph::new(rates, distances);
        // assert_eq!(result, 21);
    }
}
