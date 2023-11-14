use aoc::{get_lines, GetMany};
use itertools::Itertools;
use std::collections::HashSet;

fn part1(filename: &str) -> usize {
    // okay I need a 2d array backed by HashMap
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    // start at 0,0
    let (mut head_x, mut head_y) = (0, 0);
    let (mut tail_x, mut tail_y) = (0, 0);

    grid.insert((0, 0));

    // follow the instructions
    for line in get_lines(filename) {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        chars.next(); // skip comma
        let dist = chars.collect::<String>().parse::<usize>().unwrap();

        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("invalid direction"),
        };

        for _ in 0..dist {
            // move the head a square then let the tail follow
            head_x += dx;
            head_y += dy;

            let dx: i32 = head_x - tail_x;
            let dy: i32 = head_y - tail_y;

            // if the tail is 2 squares away from the head, move it
            if dx.abs() > 1 || dy.abs() > 1 {
                tail_x += dx.signum();
                tail_y += dy.signum();

                grid.insert((tail_x, tail_y));
            }
        }
    }

    grid.len()
}

fn part2(filename: &str) -> usize {
    // Need to do the same thing but now we have 10 connected groups
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    // start at 0,0
    let mut tails = vec![(0, 0); 10];
    grid.insert(tails[0]);

    // follow the instructions
    for line in get_lines(filename) {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        chars.next(); // skip comma
        let dist = chars.collect::<String>().parse::<usize>().unwrap();

        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("invalid direction"),
        };

        for _ in 0..dist {
            // move tails[0] a square then let the rest follow
            let head = tails.get_mut(0).unwrap();
            *head = (head.0 + dx, head.1 + dy);

            // window over pairs of tails
            for (i, j) in (0..tails.len()).tuple_windows() {
                let (head, tail) = tails.get_mut_2(i, j);

                let dx: i32 = head.0 - tail.0;
                let dy: i32 = head.1 - tail.1;

                // if the tail is 2 squares away from the head, move it
                if dx.abs() > 1 || dy.abs() > 1 {
                    tail.0 += dx.signum();
                    tail.1 += dy.signum();

                    // only track the last tail
                    if j == 9 {
                        grid.insert(*tail);
                    }
                }
            }
        }
    }

    grid.len()
}

fn main() {
    println!("{}", part1("input.txt"));
    println!("{}", part2("input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(part1("input.txt"), 6037);
        assert_eq!(part2("input.txt"), 2485);
    }

    #[test]
    fn test_example() {
        assert_eq!(part1("example_1.txt"), 13);
        assert_eq!(part2("example_2.txt"), 36);
    }
}
