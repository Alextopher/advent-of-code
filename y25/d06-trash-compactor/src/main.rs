use aoc::input_str;

fn part1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<_>>();

    let acc = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .map(|n| (n, n))
                .collect::<Vec<_>>()
        })
        .reduce(|acc, line| {
            acc.into_iter()
                .zip(line.into_iter())
                .map(|((a1, a2), (b1, b2))| ((a1 + b1), (a2 * b2)))
                .collect::<Vec<_>>()
        })
        .unwrap();

    // using the last line we pick out either the + sum or the * product
    lines
        .last()
        .unwrap()
        .split_whitespace()
        .zip(acc.into_iter())
        .map(|(op, (sum, prod))| match op {
            "*" => prod,
            "+" => sum,
            _ => unreachable!(),
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let transposed = transpose_text(input);

    // split into 'groups' seperated by \n\n
    let mut total = 0;
    for group in transposed.split("\n\n") {
        // extract the numbers from the group
        let numbers = group
            .split(&[' ', '\n', '+', '*'])
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        if group.contains('+') {
            total += numbers.iter().sum::<u64>();
        } else {
            total += numbers.iter().product::<u64>();
        }
    }

    total
}

pub fn transpose_text(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return String::new();
    }

    // Determine max line length
    let max_len = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    // Convert to Vec<Vec<char>>, padding with spaces
    let matrix: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.resize(max_len, ' ');
            chars
        })
        .collect();

    // Transpose
    let mut out = Vec::new();
    for col in 0..max_len {
        let mut new_line = String::new();
        for row in 0..matrix.len() {
            new_line.push(matrix[row][col]);
        }
        out.push(new_line.trim_end().to_string());
    }

    out.join("\n")
}

fn main() {
    let input = input_str!(2025, 6);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Time: {:?}", time.elapsed());

    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(&input));
    println!("Time: {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r"123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 4277556);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 3263827);
    }
}
