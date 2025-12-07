use std::{collections::HashMap, ops::Add, process::exit, thread};

use aoc::input_str;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    name: [u8; 3],
    left: [u8; 3],
    right: [u8; 3],
}

impl Node {
    fn new(name: [u8; 3], left: [u8; 3], right: [u8; 3]) -> Self {
        Self { name, left, right }
    }

    fn name(&self) -> [u8; 3] {
        self.name
    }

    fn next(&self, direction: char) -> [u8; 3] {
        match direction {
            'L' => self.left,
            'R' => self.right,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> (String, HashMap<[u8; 3], Node>) {
    let mut lines = input.lines();

    let directions = lines.next().unwrap().to_owned();

    let mut map = HashMap::new();
    for line in lines.skip(1) {
        let bytes = line.as_bytes();

        let name = [bytes[0], bytes[1], bytes[2]];
        let left = [bytes[7], bytes[8], bytes[9]];
        let right = [bytes[12], bytes[13], bytes[14]];

        map.insert(name, Node::new(name, left, right));
    }

    (directions, map)
}

// The first optimization is to shrink the map by moving it into a vector.
struct Graph {
    // The 1 step lookup table.
    lookup: Vec<usize>,
    names: HashMap<[u8; 3], usize>,
}

impl Graph {
    fn new(map: &HashMap<[u8; 3], Node>) -> Self {
        let mut names = HashMap::new();
        for (index, name) in map.keys().enumerate() {
            names.insert(*name, index * 2);
        }

        let mut lookup = vec![0; map.len() * 2];
        for (name, node) in map.iter() {
            let index = names.get(name).unwrap();
            let left = names.get(&node.left).unwrap();
            let right = names.get(&node.right).unwrap();

            lookup[*index] = *left;
            lookup[*index + 1] = *right;
        }

        Self { names, lookup }
    }

    // Builds a step N lookup table.
    fn build_lookup(&self, steps: usize) -> Vec<usize> {
        let mut lookup = self.lookup.clone();
        for _ in 0..steps {
            lookup = lookup
                .iter()
                .copied()
                .map(|state| self.lookup[state])
                .collect();
        }
        lookup
    }

    fn parallel_run(&self, directions: &str, threads: usize) {
        // Map directions to bools.
        let directions: Vec<_> = directions
            .chars()
            .map(|direction| direction == 'R')
            .collect();

        let start = *self.names.get(b"AAA").unwrap();
        let end = *self.names.get(b"ZZZ").unwrap();

        // Build a longer lookup table
        let large = self.build_lookup(threads);
        let time = std::time::Instant::now();

        // Each thread offsets itself by its index.
        let mut handles = Vec::new();
        for index in 0..threads {
            let small = self.lookup.clone();
            let large = large.clone();
            let directions = directions.clone();

            handles.push(thread::spawn(move || {
                // Cycle the directions.
                let mut directions = directions.into_iter().cycle();
                let mut current = start;
                let mut count = 0;

                for _ in 0..index {
                    let direction = directions.next().unwrap();
                    current = small[current + direction as usize];
                    count += 1;
                }

                loop {
                    let direction = directions.next().unwrap();
                    current = large[current + direction as usize];
                    count += threads;

                    println!("Thread {} current {:?} count {}", index, current, count);

                    if current == end {
                        break;
                    }
                }

                println!("Thread {} count {}", index, count);
                println!(
                    "iterations per nanosecond: {:?}",
                    count as f64 / time.elapsed().as_nanos() as f64
                );

                exit(0);
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn run(
    directions: &str,
    map: &HashMap<[u8; 3], Node>,
    start: [u8; 3],
    is_end: impl Fn(&[u8; 3]) -> bool,
) -> usize {
    directions
        .chars()
        .cycle()
        .scan(map.get(&start).unwrap(), |current, direction| {
            *current = map.get(&current.next(direction)).unwrap();
            Some(current.name())
        })
        .take_while(|node| !is_end(node))
        .count()
        .add(1)
}

fn part1(directions: &str, map: &HashMap<[u8; 3], Node>) -> usize {
    run(directions, map, *b"AAA", |n| n == b"ZZZ")
}

fn part2(directions: &str, map: &HashMap<[u8; 3], Node>) -> usize {
    map.keys()
        .filter(|&name| name[2] == b'A')
        .map(|&start| run(directions, map, start, |name| name[2] == b'Z'))
        .fold(1, num::integer::lcm)
}

fn part1_new(directions: &str, map: &HashMap<[u8; 3], Node>) {
    let graph = Graph::new(map);

    graph.parallel_run(directions, 2);
}

fn main() {
    let input = input_str!(2023, 8);
    let (directions, map) = parse(input);
    println!("Directions length {}", directions.len());

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&directions, &map));
    println!("Time: {:?}", time.elapsed());

    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(&directions, &map));
    println!("Time: {:?}", time.elapsed());

    part1_new(&directions, &map);
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_example_part2() {
    //     let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
    //     let (directions, map) = parse(input);

    //     assert_eq!(part2(&directions, &map), 6);
    // }

    // #[test]
    // fn verify() {
    //     let input = input_str!(2023, 8);
    //     let (directions, map) = parse(input);

    //     assert_eq!(part1(&directions, &map), 20659);
    //     assert_eq!(part2(&directions, &map), 15690466351717);
    //     part1_new(&directions, &map);
    // }
}
