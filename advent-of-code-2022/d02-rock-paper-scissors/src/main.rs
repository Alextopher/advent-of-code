use aoc::{get_lines, stringstuff::CharExt};

// A rock paper scissors game
//
//        Rock
//      /      \
//   Paper  - Scissors
//
// We can treat RPS as a cycle, where each element is a winner over the next
// and a loser to the previous.
//
// Peraphs this function could be better optimized? It's certianly not understandable
fn solution(filename: &str) -> (i32, i32) {
    let strat: Vec<(i32, i32)> = get_lines(filename)
        .map(|l| (l.chars().next().unwrap(), l.chars().nth(2).unwrap()))
        .map(|(a, b)| (a.letter_to_num(), b.letter_to_num::<i32>() - 23))
        .collect();

    let part1: i32 = strat
        .iter()
        .map(|(a, b)| (b + 1, (a - b).rem_euclid(3)))
        .map(|(a, b)| {
            a + match b {
                0 => 3,
                1 => 0,
                2 => 6,
                _ => panic!("bad"),
            }
        })
        .sum();

    let part2: i32 = strat
        .iter()
        .map(|(a, b)| {
            (
                a,
                match b {
                    0 => 1,
                    1 => 0,
                    2 => 2,
                    _ => panic!("bad"),
                },
            )
        })
        .map(|(a, b)| {
            (a - b).rem_euclid(3)
                + 1
                + match b {
                    0 => 3,
                    1 => 0,
                    2 => 6,
                    _ => panic!("bad"),
                }
        })
        .sum();

    (part1, part2)
}

fn main() {
    solution("example.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(solution("input.txt"), (9241, 14610));
    }

    #[test]
    fn test_example() {
        assert_eq!(solution("example.txt"), (15, 12));
    }
}
