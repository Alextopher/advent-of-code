mod part1;
mod part2;

use aoc::input_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let input = input_str!(2023, 7);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1::part1(input));
    println!("Time: {:?}", time.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        assert_eq!(part1::part1(input), 6440);
    }

    #[test]
    fn verify() {
        let input = input_str!(2023, 7);
        assert_eq!(part1::part1(input), 251927063);
    }
}
