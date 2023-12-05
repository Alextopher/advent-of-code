use std::collections::VecDeque;

use aoc::input_str;

fn parse(line: &str) -> usize {
    let numbers: Vec<usize> = line
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse().unwrap_or(0))
        .collect();

    let split_index = numbers.iter().position(|&n| n == 0).unwrap();
    let (winning, rest) = numbers.split_at(split_index);

    rest.iter().filter(|n| winning.contains(n)).count()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse)
        .map(|n| (1 << n) >> 1)
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let mut sum = 0;

    let mut queue = VecDeque::new();
    for line in input.lines() {
        let num_cards = queue.pop_front().unwrap_or(1);
        sum += num_cards;

        let count = parse(line);

        // Make sure the queue has enough space for the next `count` cards
        if queue.len() < count {
            queue.resize(count, 1);
        }

        queue.iter_mut().take(count).for_each(|n| *n += num_cards);
    }

    sum
}

fn main() {
    let input = input_str!(2023, 4);

    let start = std::time::Instant::now();

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));

    println!("Time: {}µs", start.elapsed().as_micros());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_part1() {
        let tests = [
            ("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4),
            ("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2),
            ("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2),
            ("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1),
            ("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0),
            ("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0),
        ];

        for (input, expected) in tests.iter() {
            assert_eq!(parse(input), *expected);
        }
    }

    #[test]
    fn example_part2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input), 30);
    }

    #[test]
    fn verify_part1() {
        let input = input_str!(2023, 4);
        assert_eq!(part1(input), 26218);
    }

    #[test]
    fn verify_part2() {
        let input = input_str!(2023, 4);
        assert_eq!(part2(input), 9997537);
    }
}
