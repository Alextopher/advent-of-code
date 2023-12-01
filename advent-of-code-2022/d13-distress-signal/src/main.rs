// Packet data consists of lists and integers. Each list starts with [, ends with ], and contains zero or
// more comma-separated values (either integers or other lists).

#![feature(iter_array_chunks)]

use std::cmp::Ordering;

use aoc::input_str;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Number(u32),
    StartList,
    EndList,
    Comma,
}

/// Converts a string into a vec of tokens.
fn tokenize(input: &str) -> Vec<Token> {
    // Skips the first and last characters, which are always '[' and ']'.
    let input = &input[1..input.len() - 1];

    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '[' => tokens.push(Token::StartList),
            ']' => tokens.push(Token::EndList),
            ',' => tokens.push(Token::Comma),
            '0'..='9' => {
                let mut number = c.to_digit(10).unwrap();
                while let Some('0'..='9') = chars.peek() {
                    number *= 10;
                    number += chars.next().unwrap().to_digit(10).unwrap();
                }
                tokens.push(Token::Number(number));
            }
            _ => {}
        }
    }

    tokens
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Integer(u32),
}

fn parse_list(tokens: &mut Vec<Token>) -> PacketData {
    let mut list = Vec::new();

    while let Some(token) = tokens.pop() {
        match token {
            Token::StartList => list.push(parse_list(tokens)),
            Token::EndList => return PacketData::List(list),
            Token::Number(n) => list.push(PacketData::Integer(n)),
            Token::Comma => {}
        }
    }

    PacketData::List(list)
}

fn parse(input: &str) -> PacketData {
    let tokens = tokenize(input);
    // Reverse the tokens so we can pop them off the end.
    let mut tokens = tokens.into_iter().rev().collect();
    parse_list(&mut tokens)
}

// This problem is all about ordering PacketData
impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // When comparing two values, the first value is called left and the second value is called right. Then:

        // If both values are integers, the lower integer should come first. If the left integer is lower than the right integer,
        // the inputs are in the right order. If the left integer is higher than the right integer, the inputs are not in the right order.
        // Otherwise, the inputs are the same integer; continue checking the next part of the input.
        //
        // If both values are lists, compare the first value of each list, then the second value, and so on. If the left list runs
        // out of items first, the inputs are in the right order. If the right list runs out of items first, the inputs are not in the right
        // order. If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
        //
        // If exactly one value is an integer, convert the integer to a list which contains that integer as its only value, then retry the comparison.
        // For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2); the result is then found by
        // instead comparing [0,0,0] and [2].
        match (self, other) {
            (PacketData::Integer(a), PacketData::Integer(b)) => a.cmp(b),
            (PacketData::List(a), PacketData::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.cmp(b) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => {}
                    }
                }

                // If we get here, then the lists are equal up to the length of the shorter list.
                // If the lists are the same length, then they are equal.
                // If the left list is shorter, then it comes first.
                // If the right list is shorter, then it comes first.
                a.len().cmp(&b.len())
            }
            (list, PacketData::Integer(a)) => {
                list.cmp(&PacketData::List(vec![PacketData::Integer(*a)]))
            }
            (PacketData::Integer(a), list) => {
                PacketData::List(vec![PacketData::Integer(*a)]).cmp(list)
            }
        }
    }
}

fn part1(input: &str) -> usize {
    // Filter inputs down to just include tokens
    let tokens = input.lines().filter_map(|line| {
        if line.starts_with('[') && line.ends_with(']') {
            Some(parse(line))
        } else {
            None
        }
    });

    tokens
        .array_chunks::<2>()
        .enumerate()
        .filter_map(|(i, chunk)| {
            let left = &chunk[0];
            let right = &chunk[1];

            if left < right {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    // Filter inputs down to just include tokens
    let mut tokens: Vec<PacketData> = input
        .lines()
        .filter_map(|line| {
            if line.starts_with('[') && line.ends_with(']') {
                Some(parse(line))
            } else {
                None
            }
        })
        .collect();

    // Add [[2]] and [[6]] to the list
    let two = PacketData::List(vec![PacketData::Integer(2)]);
    let six = PacketData::List(vec![PacketData::Integer(6)]);

    tokens.push(two.clone());
    tokens.push(six.clone());

    tokens.sort_unstable();

    // Find the index of [[2]] and [[6]]
    let index_2 = tokens.iter().position(|x| x == &two).unwrap();
    let index_6 = tokens.iter().position(|x| x == &six).unwrap();

    (index_2 + 1) * (index_6 + 1)
}

fn main() {
    let input = input_str!(2022, 13);
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_and_compare(input: &str, expected: PacketData) {
        let actual = parse(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parsing() {
        use PacketData::*;

        // [1,1,3,1,1]
        parse_and_compare(
            "[1,1,3,1,1]",
            List(vec![
                Integer(1),
                Integer(1),
                Integer(3),
                Integer(1),
                Integer(1),
            ]),
        );

        // [[1],[2,3,4]]
        parse_and_compare(
            "[[1],[2,3,4]]",
            List(vec![
                List(vec![Integer(1)]),
                List(vec![Integer(2), Integer(3), Integer(4)]),
            ]),
        );

        // [[1],4]
        parse_and_compare("[[1],4]", List(vec![List(vec![Integer(1)]), Integer(4)]));

        // [9]
        parse_and_compare("[9]", List(vec![Integer(9)]));

        // [[8,7,6]]
        parse_and_compare(
            "[[8,7,6]]",
            List(vec![List(vec![Integer(8), Integer(7), Integer(6)])]),
        );

        // [[[]]]
        parse_and_compare("[[[]]]", List(vec![List(vec![List(vec![])])]));

        // [1,[2,[3,[4,[5,6,0]]]],8,9]
        parse_and_compare(
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            List(vec![
                Integer(1),
                List(vec![
                    Integer(2),
                    List(vec![
                        Integer(3),
                        List(vec![
                            Integer(4),
                            List(vec![Integer(5), Integer(6), Integer(0)]),
                        ]),
                    ]),
                ]),
                Integer(8),
                Integer(9),
            ]),
        );
    }

    // Test example orderings
    #[test]
    fn test_ord() {
        // [1,1,3,1,1] < [1,1,5,1,1]
        assert!(parse("[1,1,3,1,1]") < parse("[1,1,5,1,1]"));
        // [[1],[2,3,4]] < [[1],4]
        assert!(parse("[[1],[2,3,4]]") < parse("[[1],4]"));
        // [9] > [[8,7,6]]
        assert!(parse("[9]") > parse("[[8,7,6]]"));
        // [[4,4],4,4] < [[4,4],4,4,4]
        assert!(parse("[[4,4],4,4]") < parse("[[4,4],4,4,4]"));
        //[7,7,7,7] > [7,7,7]
        assert!(parse("[7,7,7,7]") > parse("[7,7,7]"));
        // [] < [3]
        assert!(parse("[]") < parse("[3]"));
        // [[[]]] > [[]]
        assert!(parse("[[[]]]") > parse("[[]]"));
        // [1,[2,[3,[4,[5,6,7]]]],8,9] > [1,[2,[3,[4,[5,6,0]]]],8,9]
        assert!(parse("[1,[2,[3,[4,[5,6,7]]]],8,9]") > parse("[1,[2,[3,[4,[5,6,0]]]],8,9]"));
    }

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        assert_eq!(part1(input), 13);
        assert_eq!(part2(input), 140);
    }
}
