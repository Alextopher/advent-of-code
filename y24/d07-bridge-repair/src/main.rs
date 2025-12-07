use aoc::{input_str, time};

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn part1_inner(terms: &[u64], target: u64, idx: usize, partial_sum: u64) -> bool {
    if idx == terms.len() {
        return partial_sum == target;
    }

    if partial_sum > target {
        return false;
    }

    part1_inner(terms, target, idx + 1, partial_sum + terms[idx])
        || part1_inner(terms, target, idx + 1, partial_sum * terms[idx])
}

fn part2_inner(terms: &[u64], target: u64, idx: usize, partial_sum: u64) -> bool {
    if idx == terms.len() {
        return partial_sum == target;
    }

    if partial_sum > target {
        return false;
    }

    let concat = part2_inner(terms, target, idx + 1, concat(partial_sum, terms[idx]));
    let mul = part2_inner(terms, target, idx + 1, partial_sum * terms[idx]);
    let add = part2_inner(terms, target, idx + 1, partial_sum + terms[idx]);

    concat || mul || add
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let total = parts.next().unwrap().parse::<u64>().unwrap();
            let terms = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|term| term.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (total, terms)
        })
        .filter_map(|(total, terms)| part1_inner(&terms, total, 1, terms[0]).then_some(total))
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let total = parts.next().unwrap().parse::<u64>().unwrap();
            let terms = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|term| term.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            (total, terms)
        })
        .filter_map(|(total, terms)| part2_inner(&terms, total, 1, terms[0]).then_some(total))
        .sum()
}

fn main() {
    let input = input_str!(2024, 7);

    let part1 = time("Part 1: ", || part1(input));
    println!("Part 1: {}", part1);

    let part2 = time("Part 2: ", || part2(input));
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn example() {
        let example = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        assert_eq!(part1(example), 3749);
        assert_eq!(part2(example), 11387);
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(123, 45), 12345);
        assert_eq!(concat(100, 2), 1002);
        assert_eq!(concat(999, 9), 9999);
    }
}
