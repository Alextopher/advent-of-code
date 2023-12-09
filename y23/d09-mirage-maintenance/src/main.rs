use aoc::input_str;

fn diff(sequence: &[i32]) -> Vec<i32> {
    sequence.windows(2).map(|w| w[1] - w[0]).collect()
}

fn zipped(sequence: &[i32]) -> (i32, i32) {
    if sequence.iter().copied().all(|n| n == 0) {
        return (0, 0);
    }

    let (first, last) = zipped(&diff(sequence));
    (
        sequence.first().unwrap() - first,
        sequence.last().unwrap() + last,
    )
}

fn solve(input: &str) -> (i32, i32) {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|sequence| zipped(&sequence))
        .reduce(|(part2, part1), (first, last)| (part2 + first, part1 + last))
        .unwrap()
}

pub fn main() {
    let input: &str = input_str!(2023, 9);

    let time = std::time::Instant::now();
    let (part2, part1) = solve(input);

    let time = time.elapsed();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Time: {:?}", time);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // 0 3 6 9 12 15 -> -3, 18
        let numbers = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(zipped(&numbers), (-3, 18));

        // 1 3 6 10 15 21 -> 0, 28
        let numbers = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(zipped(&numbers), (0, 28));

        // 10 13 16 21 30 45 -> 5 ,68
        let numbers = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(zipped(&numbers), (5, 68));
    }

    #[test]
    fn verify() {
        let input: &str = input_str!(2023, 9);
        // part2, part1
        let expected: (i32, i32) = (1124, 1921197370);
        assert_eq!(solve(input), expected);
    }
}
