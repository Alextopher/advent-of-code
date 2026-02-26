use aoc::{IterJunk, input_str};

fn part1_best(bank: &str) -> i32 {
    let (i, c1) = bank
        .chars()
        .take(bank.len() - 1)
        .enumerate()
        .max_by_key_first(|&(_, c)| c)
        .unwrap();

    let (_, c2) = bank
        .chars()
        .skip(i + 1)
        .enumerate()
        .max_by_key_first(|&(_, c)| c)
        .unwrap();

    // parse c1 and c2 to i32
    10 * (c1.to_digit(10).unwrap() as i32) + (c2.to_digit(10).unwrap() as i32)
}

fn part1(input: &str) -> i32 {
    input.lines().map(part1_best).sum()
}

// Instead of turning on 2 batteries we're turning on 12
fn part2_best(bank: &str) -> i64 {
    let bank = bank.as_bytes();

    let (mut p, mut n) = (0usize, 0i64);
    for i in 1..=12 {
        // find the largest digit in p..len - 12 + i
        let (j, c) = bank[p..bank.len() - 12 + i]
            .iter()
            .enumerate()
            .max_by_key_first(|&(_, &c)| c)
            .unwrap();

        p += j + 1;
        n = 10 * n + (c - b'0') as i64;
    }
    n
}

fn part2(input: &str) -> i64 {
    input.lines().map(part2_best).sum()
}

fn main() {
    let input = input_str!(2025, 3);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(part1_best("987654321111111"), 98);
        assert_eq!(part1_best("811111111111119"), 89);
        assert_eq!(part1_best("234234234234278"), 78);
        assert_eq!(part1_best("818181911112111"), 92);

        assert_eq!(part2_best("987654321111111"), 987654321111);
        assert_eq!(part2_best("811111111111119"), 811111111119);
        assert_eq!(part2_best("234234234234278"), 434234234278);
        assert_eq!(part2_best("818181911112111"), 888911112111);
    }
}
