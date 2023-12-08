use std::{collections::HashMap, ops::Add};

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

fn main() {
    let input = input_str!(2023, 8);
    let (directions, map) = parse(input);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&directions, &map));
    println!("Time: {:?}", time.elapsed());

    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(&directions, &map));
    println!("Time: {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part2() {
        let input = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)";
        let (directions, map) = parse(input);

        assert_eq!(part2(&directions, &map), 6);
    }

    #[test]
    fn verify() {
        let input = input_str!(2023, 8);
        let (directions, map) = parse(input);

        assert_eq!(part1(&directions, &map), 20659);
        assert_eq!(part2(&directions, &map), 15690466351717);
    }
}
