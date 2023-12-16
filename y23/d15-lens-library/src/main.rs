use std::collections::VecDeque;

use aoc::input_str;

fn hash(s: &str) -> u8 {
    s.bytes().fold(0, |acc, b| (acc + b) * 17)
}

fn part1(input: &str) -> u32 {
    input
        .split(',')
        .map(|s| s.trim())
        .map(hash)
        .map(|h| h as u32)
        .sum()
}

#[derive(Debug, Clone, Copy, Default)]
struct Value<'a> {
    label: &'a str,
    value: u8,
}

struct Hashmap<'a> {
    buckets: Vec<VecDeque<Value<'a>>>,
}

impl<'a> Hashmap<'a> {
    fn new() -> Self {
        Self {
            buckets: vec![VecDeque::new(); 256],
        }
    }

    fn get_bucket(&mut self, label: &'a str) -> &mut VecDeque<Value<'a>> {
        &mut self.buckets[hash(label) as usize]
    }

    fn insert(&mut self, label: &'a str, value: u8) {
        let bucket = self.get_bucket(label);
        if let Some(v) = bucket.iter_mut().find(|v| v.label == label) {
            v.value = value;
        } else {
            bucket.push_back(Value { label, value });
        }
    }

    fn remove(&mut self, label: &'a str) {
        let bucket = self.get_bucket(label);
        if let Some(pos) = bucket.iter().position(|v| v.label == label) {
            bucket.remove(pos);
        }
    }

    fn score(&self) -> u32 {
        self.buckets
            .iter()
            .zip(1..)
            .flat_map(|(b, i)| {
                b.iter()
                    .zip(1..)
                    .map(move |(v, j)| (v.value as u32) * i * j)
            })
            .sum::<u32>()
    }
}

fn part2(input: &str) -> u32 {
    let instructions = input.split(',').map(|s| s.trim());

    // Instruction is either
    // - "label=value" (set label)
    // or
    // - "label-" (remove label)
    let mut hashmap = Hashmap::new();
    for instruction in instructions {
        if let Some(label) = &instruction.strip_suffix('-') {
            hashmap.remove(label);
        } else {
            let mut parts = instruction.split('=');
            let label = parts.next().unwrap();
            let value = parts.next().unwrap().parse::<u8>().unwrap();
            hashmap.insert(label, value);
        }
    }
    hashmap.score()
}

fn main() {
    let input = input_str!(2023, 15);
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
