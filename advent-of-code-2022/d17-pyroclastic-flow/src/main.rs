#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::HashSet;

use aoc::input_str;

// >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
// blowing pattern (RIGHT, LEFT)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

fn blowing_pattern(input: &str) -> Vec<Direction> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(Direction::from)
        .collect()
}

// ####
const HORIZTONAL: [(usize, usize); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];

// .#.
// ###
// .#.
const PLUS: [(usize, usize); 5] = [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)];

// ..#
// ..#
// ###
const L: [(usize, usize); 5] = [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)];

// #
// #
// #
// #
const VERTICAL: [(usize, usize); 4] = [(0, 0), (0, 1), (0, 2), (0, 3)];

// ##
// ##
const SQUARE: [(usize, usize); 4] = [(0, 0), (1, 0), (0, 1), (1, 1)];

const CYCLE: [&[(usize, usize)]; 5] = [&HORIZTONAL, &PLUS, &L, &VERTICAL, &SQUARE];

struct Chamber {
    // (0, *) is treated as solid
    // (width + 1, *) is treated as solid
    solid: HashSet<(usize, usize)>,

    // Cache the height of the highest solid block
    highest_solid: usize,
}

impl Chamber {
    fn new() -> Self {
        Self {
            solid: HashSet::new(),
            highest_solid: 0,
        }
    }

    fn is_solid(&self, x: usize, y: usize) -> bool {
        if x == 0 || x == 8 || y == 0 {
            return true;
        }

        self.solid.contains(&(x, y))
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        !self.is_solid(x, y)
    }

    fn set_solid(&mut self, x: usize, y: usize) {
        self.solid.insert((x, y));
        self.highest_solid = self.highest_solid.max(y);
    }

    // Simulates the rock falling down 1 step
    //
    // Returns false is the rock is stuck
    fn fall(&self, rock: &mut [(usize, usize)]) -> bool {
        if rock.iter().all(|(x, y)| self.is_empty(*x, *y - 1)) {
            rock.iter_mut().for_each(|(_, y)| *y -= 1);
            return true;
        }
        false
    }

    // Moves the rock to the left if possible
    fn move_left(&self, rock: &mut [(usize, usize)]) {
        if rock.iter().all(|(x, y)| self.is_empty(*x - 1, *y)) {
            rock.iter_mut().for_each(|(x, _)| *x -= 1);
        }
    }

    // Moves the rock to the right if possible
    fn move_right(&self, rock: &mut [(usize, usize)]) {
        if rock.iter().all(|(x, y)| self.is_empty(*x + 1, *y)) {
            rock.iter_mut().for_each(|(x, _)| *x += 1);
        }
    }

    // Simulates a single rock spawning and falling down
    fn drop_rock(
        &mut self,
        pattern: &'static [(usize, usize)],
        blowing_pattern: &mut impl Iterator<Item = Direction>,
    ) {
        // Each rock appears so that its left edge is two units away from the left wall and its bottom edge is
        // three units above the highest rock in the room (or the floor, if there isn't one).
        let mut rock = pattern.to_vec();
        rock.iter_mut().for_each(|(x, y)| {
            *x += 3;
            *y += self.highest_solid + 4;
        });

        loop {
            // Move the rock left or right
            match blowing_pattern.next().unwrap() {
                Direction::Left => self.move_left(&mut rock),
                Direction::Right => self.move_right(&mut rock),
            }

            // If the rock is stuck, make it solid and return
            if !self.fall(&mut rock) {
                rock.iter().for_each(|(x, y)| self.set_solid(*x, *y));
                return;
            }
        }
    }

    fn part1(
        &mut self,
        rocks: usize,
        mut blowing_pattern: &mut impl Iterator<Item = Direction>,
    ) -> usize {
        let mut cycle = CYCLE.iter().cycle();

        for _ in 0..rocks {
            self.drop_rock(cycle.next().unwrap(), &mut blowing_pattern);
        }

        self.highest_solid
    }

    // part2 requires _a lot_ of rocks (1 trillion)
    fn part2(&mut self, rocks: usize, blowing_pattern: &[Direction]) -> usize {
        // Make part 1 basically a lazy iterator

        todo!()
    }
}

fn main() {
    let blowing_pattern = blowing_pattern(input_str!(2022, 17));
    let mut chamber = Chamber::new();

    println!(
        "{}",
        chamber.part1(2022, &mut blowing_pattern.iter().copied().cycle())
    );

    println!(
        "{}",
        chamber.part1(
            1_000_000_000_000,
            &mut blowing_pattern.iter().copied().cycle()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_test() {
        let pattern = blowing_pattern(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let mut blowing_pattern = pattern.iter().copied().cycle();

        let mut chamber = Chamber::new();

        // Drop a horizontal rock
        chamber.drop_rock(&HORIZTONAL, &mut blowing_pattern);

        // Verify the location of the rock
        assert!(chamber.is_solid(3, 1));
        assert!(chamber.is_solid(4, 1));
        assert!(chamber.is_solid(5, 1));
        assert!(chamber.is_solid(6, 1));

        // Drop a plus rock
        chamber.drop_rock(&PLUS, &mut blowing_pattern);

        // Verify the location of the rock
        assert!(chamber.is_solid(4, 2));
        assert!(chamber.is_solid(3, 3));
        assert!(chamber.is_solid(4, 3));
        assert!(chamber.is_solid(5, 3));
        assert!(chamber.is_solid(4, 4));
    }

    #[test]
    fn test_example() {
        let pattern = blowing_pattern(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        let mut blowing_pattern = pattern.iter().copied().cycle();
        let mut chamber = Chamber::new();

        assert_eq!(chamber.part1(2022, &mut blowing_pattern), 3068);
    }

    #[test]
    fn test_part1() {
        let pattern = blowing_pattern(input_str!(2022, 17));
        let mut blowing_pattern = pattern.iter().copied().cycle();
        let mut chamber = Chamber::new();

        assert_eq!(chamber.part1(2022, &mut blowing_pattern), 3202);
    }
}
