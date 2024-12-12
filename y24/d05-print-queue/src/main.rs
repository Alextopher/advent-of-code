use std::cmp::Ordering;

use aoc::input_str;

/// Eh, every problem is a graph problem.
///
/// If (x, y) is in the matrix then X | Y, X comes before Y
#[derive(Debug)]
pub struct Graph {
    n: usize,
    matrix: Vec<bool>,
}

impl Graph {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            matrix: vec![false; (n + 1) * (n + 1)],
        }
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.n + x
    }

    pub fn insert(&mut self, x: usize, y: usize) {
        let idx = self.index(x, y);
        self.matrix[idx] = true
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        let idx = self.index(x, y);
        self.matrix[idx]
    }
}

fn main() {
    let input = input_str!(2024, 5);

    let start = std::time::Instant::now();

    let mut lines = input.lines();

    let mut counter = 0;
    let mut mapping = [None; 100];
    let mut reverse = vec![];
    let mut constraints = vec![];

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        let mut split = line.split('|');
        let x: usize = split.next().unwrap().parse().unwrap();
        let y: usize = split.next().unwrap().parse().unwrap();

        if mapping[x].is_none() {
            counter += 1;
            mapping[x] = Some(counter);
            reverse.push(x);
        }

        if mapping[y].is_none() {
            counter += 1;
            mapping[y] = Some(counter);
            reverse.push(y);
        }

        constraints.push((mapping[x].unwrap(), mapping[y].unwrap()));
    }

    debug_assert_eq!(mapping.iter().filter(|m| m.is_some()).count(), counter);
    let mut graph = Graph::new(counter);
    for (x, y) in constraints {
        graph.insert(x, y);
    }

    println!("Created graph: {:?}", start.elapsed());
    let start = std::time::Instant::now();

    let (good, bad): (Vec<_>, Vec<_>) = lines
        .map(|line| {
            line.split(',')
                .map(|i| i.parse::<usize>().unwrap())
                .map(|i| mapping[i].unwrap())
                .collect::<Vec<_>>()
        })
        .partition(|pages| pages.windows(2).all(|v| graph.contains(v[0], v[1])));

    println!("Partitioned: {:?}", start.elapsed());
    let start = std::time::Instant::now();

    let part1: usize = good
        .into_iter()
        .map(|pages| reverse[pages[pages.len() / 2] - 1])
        .sum();

    println!("Part 1: {:?}", start.elapsed());
    println!("Part 1: {}", part1);
    let start = std::time::Instant::now();

    let part2: usize = bad
        .into_iter()
        .map(|mut page| {
            page.sort_unstable_by(|x, y| match graph.contains(*x, *y) {
                true => Ordering::Greater,
                false => Ordering::Less,
            });
            page
        })
        .map(|pages| reverse[pages[pages.len() / 2] - 1])
        .sum();

    println!("Part 2: {:?}", start.elapsed());
    println!("Part 2: {}", part2);
}
