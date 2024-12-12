// Builds some kind of DFA
//
// In the example '?###???????? 3,2,1' the generate machine is
//
// \.* - beginning  (repeat . && accept #)
// ### - 3!         (accept # , accept # , accept .)
// \.+ - spacing    (repeat . && accept #)
// ##  - 2!         (accept # , accept .)
// \.+ - spacing    (repeat . && accept #)
// #   - 1!         (accept .)
// \.* - ending     (repeat .)
//
// Once the 'machine' is built running through the particular input requires keeping track of how heads on are each particular state and successfully book keeping when the next token is read.
// question marks ? split individual DFA states into 2

use aoc::{input_str, time};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Condition {
    Good,    // .
    Bad,     // #
    Unknown, // ?
}

#[derive(Debug, Clone, Copy)]
enum States {
    Spacing,   // repeat . && accept #
    AcceptDot, // accept .
    AcceptTag, // accept #
    Ending,    // repeat .
}

fn parse_line(line: &str) -> (Vec<Condition>, Vec<usize>) {
    let mut split = line.split(' ');

    let conditions = split
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            '.' => Condition::Good,
            '#' => Condition::Bad,
            '?' => Condition::Unknown,
            _ => unreachable!(),
        })
        .collect();

    let groups = split
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    (conditions, groups)
}

fn process(gears: &[Condition], groups: &[usize]) -> usize {
    // The dfa has 2 states for the start and end with \.* patterns
    // Each group G gets g = |G| states for the line of # {g}
    // Between each group 1 state in inserted for the \.+ pattern

    let mut states: Vec<States> = groups
        .iter()
        .flat_map(|g| {
            std::iter::once(States::Spacing)
                .chain(std::iter::repeat(States::AcceptTag).take(g - 1))
                .chain(std::iter::once(States::AcceptDot))
        })
        .collect();
    states.push(States::Ending);

    let mut counts = vec![0_usize; states.len() + 1];
    let mut next = counts.clone();

    // Start with 1 DFA in the starting state
    counts[0] = 1;

    for cond in gears {
        for (idx, (state, count)) in states.iter().zip(counts.iter()).enumerate() {
            match (state, cond) {
                (States::Spacing, Condition::Good) => next[idx] += count,
                (States::Spacing, Condition::Bad) => next[idx + 1] += count,
                (States::Spacing, Condition::Unknown) => {
                    next[idx] += count;
                    next[idx + 1] += count;
                }
                (States::AcceptDot, Condition::Good) => next[idx + 1] += count,
                (States::AcceptDot, Condition::Bad) => {}
                (States::AcceptDot, Condition::Unknown) => next[idx + 1] += count,
                (States::AcceptTag, Condition::Good) => {}
                (States::AcceptTag, Condition::Bad) => next[idx + 1] += count,
                (States::AcceptTag, Condition::Unknown) => next[idx + 1] += count,
                (States::Ending, Condition::Good) => next[idx] += count,
                (States::Ending, Condition::Bad) => {}
                (States::Ending, Condition::Unknown) => next[idx] += count,
            }
        }

        counts = vec![0; states.len()];
        std::mem::swap(&mut counts, &mut next);
    }

    counts[counts.len() - 2] + counts[counts.len() - 1]
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(|(gears, groups)| process(&gears, &groups))
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_line)
        .map(|(gears, groups)| {
            let mut joined_gears = gears.clone();
            for _ in 0..4 {
                joined_gears.push(Condition::Unknown);
                joined_gears.extend(gears.iter().cloned());
            }
            (joined_gears, groups.repeat(5))
        })
        .map(|(gears, groups)| process(&gears, &groups))
        .sum()
}

fn main() {
    let input = input_str!(2023, 12);
    let part1 = time("Part 1", || part1(input));
    println!("Part 1: {}", part1);
    let part2 = time("Part 2", || part2(input));
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_parsing() {
        let input = "???.### 1,1,3";
        let conditions = vec![
            Condition::Unknown,
            Condition::Unknown,
            Condition::Unknown,
            Condition::Good,
            Condition::Bad,
            Condition::Bad,
            Condition::Bad,
        ];
        let gears: Vec<usize> = vec![1, 1, 3];
        assert_eq!((conditions, gears), parse_line(input));
    }

    #[test]
    fn examples() {
        let tests = [
            ("???.### 1,1,3", 1, 1),
            (".??..??...?##. 1,1,3", 4, 16384),
            ("?#?#?#?#?#?#?#? 1,3,1,6", 1, 1),
            ("????.#...#... 4,1,1", 1, 16),
            ("????.######..#####. 1,6,5", 4, 2500),
            ("?###???????? 3,2,1", 10, 506250),
        ];

        for (input, p1, p2) in tests {
            assert_eq!(part1(input), p1, "{}", input);
            assert_eq!(part2(input), p2, "{}", input);
        }
    }

    #[test]
    fn answer_part1() {
        let input = input_str!(2023, 12);
        assert_eq!(part1(input), 7402);
    }

    #[test]
    fn answer_part2() {
        let input = input_str!(2023, 12);
        assert_eq!(part2(input), 3384337640277);
    }
}
