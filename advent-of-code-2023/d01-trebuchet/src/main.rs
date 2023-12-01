use std::collections::HashMap;

use aoc::input_str;

fn get_lookup_table() -> HashMap<&'static str, &'static str> {
    let mut numbers = HashMap::new();

    numbers.insert("one", "1one");
    numbers.insert("two", "2two");
    numbers.insert("three", "3three");
    numbers.insert("four", "4four");
    numbers.insert("five", "5five");
    numbers.insert("six", "6six");
    numbers.insert("seven", "7seven");
    numbers.insert("eight", "8eight");
    numbers.insert("nine", "9nine");

    numbers
}

fn get_number(line: impl AsRef<str>) -> usize {
    println!("{}", line.as_ref());

    let numbers = line.as_ref().bytes().filter(u8::is_ascii_digit);

    let first = (numbers.clone().next().unwrap() - b'0') as usize;
    let last = (numbers.last().unwrap() - b'0') as usize;

    first * 10 + last
}

fn part1(input: &str) -> usize {
    input.lines().map(get_number).sum()
}

fn replace_numbers(numbers: &HashMap<&str, &str>, line: &str) -> String {
    let mut buf = String::with_capacity(line.len());
    for c in line.chars() {
        buf.push(c);

        for (word, digit) in numbers {
            if buf.ends_with(word) {
                buf.truncate(buf.len() - word.len());
                buf.push_str(digit);
            }
        }
    }
    buf
}

fn part2(input: &str) -> usize {
    let lookup_table = get_lookup_table();

    input
        .lines()
        .map(|line| replace_numbers(&lookup_table, line))
        .map(get_number)
        .sum()
}

fn main() {
    let input = input_str!(2023, 1);
    println!("{}", part1(input));
    println!("{}", part2(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_str!(2023, 1);
        assert_eq!(part1(input), 54634);
    }

    #[test]
    fn test_example() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }

    #[test]
    fn replace_numbers_test() {
        let lookup_table = get_lookup_table();

        // eighthree
        assert_eq!(get_number(replace_numbers(&lookup_table, "eightthree")), 83);
    }
}
