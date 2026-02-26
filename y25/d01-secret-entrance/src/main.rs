use aoc::input_str;

fn part1(input: &str) -> i32 {
    let mut times_zero = 0;
    let mut position = 50;
    for line in input.lines() {
        let distance = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') {
            position -= distance;
        } else {
            position += distance;
        }
        position %= 100;
        if position == 0 {
            times_zero += 1;
        }
    }

    times_zero
}

fn part2(input: &str) -> i32 {
    let mut times_zero = 0;
    let mut position = 50;
    for line in input.lines() {
        let mut distance = line[1..].parse::<i32>().unwrap();
        if line.starts_with('L') {
            while distance > 0 {
                position -= 1;
                distance -= 1;
                position %= 100;
                if position == 0 {
                    times_zero += 1;
                }
            }
        } else {
            while distance > 0 {
                position += 1;
                distance -= 1;
                position %= 100;
                if position == 0 {
                    times_zero += 1;
                }
            }
        }
    }

    times_zero
}

fn main() {
    let input = input_str!(2025, 1);

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
    fn test_part1() {
        let input = input_str!(2025, 1);
        assert_eq!(part1(input), 992);
    }

    #[test]
    fn test_part2() {
        let input = input_str!(2025, 1);
        assert_eq!(part2(input), 6133);
    }

    #[test]
    fn test_example_part2() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
        assert_eq!(part2(input), 6);
    }
}
