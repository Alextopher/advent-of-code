use std::collections::HashMap;

use aoc::input_str;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Square,
    Round,
}

impl Space {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Square,
            'O' => Self::Round,
            _ => panic!("Invalid space"),
        }
    }

    fn as_char(&self) -> char {
        match self {
            Space::Empty => '.',
            Space::Square => '#',
            Space::Round => 'O',
        }
    }

    fn is_round(&self) -> bool {
        matches!(self, Self::Round)
    }

    fn is_square(&self) -> bool {
        matches!(self, Self::Square)
    }
}

#[derive(Debug, Clone)]
struct Reflector {
    spaces: Vec<Space>,
    transposed: Vec<Space>,
    width: usize,
    height: usize,
}

impl Reflector {
    fn from_str(input: &str) -> Self {
        let mut height = 0;
        let spaces = input
            .lines()
            .map(|s| s.chars())
            .flat_map(|line| {
                height += 1;
                line
            })
            .map(Space::from_char)
            .collect_vec();

        let width = spaces.len() / height;
        let transposed = (0..width)
            .flat_map(|x| (0..height).map(|y| spaces[y * width + x]).collect_vec())
            .collect_vec();

        Self {
            spaces,
            transposed,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Space {
        self.spaces[y * self.width + x]
    }

    fn get_row(&self, y: usize) -> &[Space] {
        &self.spaces[y * self.width..(y + 1) * self.width]
    }

    fn set_row(&mut self, y: usize, row: &[Space]) {
        debug_assert_eq!(row.len(), self.width);
        self.spaces[y * self.width..(y + 1) * self.width].copy_from_slice(row);

        for (x, space) in row.iter().enumerate() {
            self.transposed[x * self.height + y] = *space;
        }
    }

    fn get_col(&self, x: usize) -> &[Space] {
        &self.transposed[x * self.height..(x + 1) * self.height]
    }

    fn set_col(&mut self, x: usize, col: &[Space]) {
        debug_assert_eq!(col.len(), self.height);
        self.transposed[x * self.height..(x + 1) * self.height].copy_from_slice(col);

        for (y, space) in col.iter().enumerate() {
            self.spaces[y * self.width + x] = *space;
        }
    }

    fn fall_vertical(&mut self, north: bool) {
        for x in 0..self.width {
            let mut new_col = vec![Space::Empty; self.height];

            let mut col = self.get_col(x).to_vec();
            if !north {
                col.reverse();
            }

            let mut rocks = 0;
            let mut last_square = 0;
            for (i, space) in col.iter().enumerate() {
                if space.is_round() {
                    rocks += 1;
                } else if space.is_square() {
                    new_col[last_square..last_square + rocks].fill(Space::Round);
                    new_col[i] = Space::Square;
                    rocks = 0;
                    last_square = i + 1;
                }
            }

            new_col[last_square..last_square + rocks].fill(Space::Round);

            if !north {
                new_col.reverse();
            }
            self.set_col(x, &new_col);
        }
    }

    fn fall_horizontal(&mut self, west: bool) {
        for y in 0..self.height {
            let mut new_row = vec![Space::Empty; self.width];

            let mut row = self.get_row(y).to_vec();
            if !west {
                row.reverse();
            }

            let mut rocks = 0;
            let mut last_square = 0;
            for (i, space) in row.iter().enumerate() {
                if space.is_round() {
                    rocks += 1;
                } else if space.is_square() {
                    new_row[last_square..last_square + rocks].fill(Space::Round);
                    new_row[i] = Space::Square;
                    rocks = 0;
                    last_square = i + 1;
                }
            }

            new_row[last_square..last_square + rocks].fill(Space::Round);

            if !west {
                new_row.reverse();
            }
            self.set_row(y, &new_row);
        }
    }

    fn slide_north(&mut self) {
        self.fall_vertical(true);
    }

    fn slide_south(&mut self) {
        self.fall_vertical(false);
    }

    fn slide_west(&mut self) {
        self.fall_horizontal(true);
    }

    fn slide_east(&mut self) {
        self.fall_horizontal(false);
    }

    fn cycle(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
    }

    /// Calculates the amount of load on the north beam
    fn load(&self) -> usize {
        (0..self.height)
            .map(|y| {
                (self.height - y)
                    * (0..self.width)
                        .filter(|x| self.get(*x, y).is_round())
                        .count()
            })
            .sum()
    }
}

impl std::fmt::Display for Reflector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..self.height).map(|y| self.get_row(y)) {
            writeln!(f, "{}", row.iter().map(Space::as_char).collect::<String>())?;
        }

        Ok(())
    }
}

fn part1(mut reflector: Reflector) -> usize {
    reflector.slide_north();
    reflector.load()
}

fn part2(mut reflector: Reflector) -> usize {
    let goal = 1_000_000_000;

    let mut seen = HashMap::new();
    seen.insert(reflector.spaces.clone(), 0);

    for i in (0..).skip(1) {
        reflector.cycle();
        if let Some(&j) = seen.get(&reflector.spaces) {
            let remaining = (goal - i) % (i - j);
            for _ in 0..remaining {
                reflector.cycle();
            }
            break;
        }
        seen.insert(reflector.spaces.clone(), i);
    }

    reflector.load()
}

fn main() {
    let input = input_str!(2023, 14);
    let start = std::time::Instant::now();
    let reflector = Reflector::from_str(input);

    println!("Part 1: {}", part1(reflector.clone()));
    println!("Part 2: {}", part2(reflector));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....";

        let reflector = Reflector::from_str(input);
        println!("{}", reflector);

        let r1 = reflector.clone();
        assert_eq!(part1(r1), 136);

        let r2 = reflector.clone();
        assert_eq!(part2(r2), 64);
    }
}
