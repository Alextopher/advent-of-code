use std::time::Instant;

use aoc::*;

fn solution(line: &str) -> (usize, usize) {
    let mut time = Instant::now();

    // find the index of first window of 4 unique characters
    let part1 = line
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, w)| lowercase_all_unique(w.iter().cloned()))
        .unwrap()
        .0;

    // print the answer and the time it took to find it
    println!("{} {:?}", part1 + 4, time.elapsed());

    // groups of 14 characters
    time = Instant::now();
    let part2 = line
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, w)| lowercase_all_unique(w.iter().cloned()))
        .unwrap()
        .0;

    // print the answer and the time it took to find it
    println!("{} {:?}", part2 + 14, time.elapsed());

    (part1 + 4, part2 + 14)
}

// Since every character is lowercase ASCII we can use a bitset to check if all characters are unique
fn lowercase_all_unique<T: Iterator<Item = u8>>(iter: T) -> bool {
    let mut set = 0;
    for c in iter {
        let bit = 1 << (c - b'a');
        if set & bit != 0 {
            return false;
        }
        set |= bit;
    }
    true
}

fn main() {
    solution(get_lines(get_input!(2022, 6)).next().unwrap().as_str());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(
            solution(get_lines(get_input!(2022, 6)).next().unwrap().as_str()),
            (1198, 3120)
        );
    }

    #[test]
    fn test_examples() {
        assert_eq!(solution("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), (7, 19));
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz"), (5, 23));
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg"), (6, 23));
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), (10, 29));
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), (11, 26));
    }
}
