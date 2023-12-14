use std::hash::{DefaultHasher, Hasher};

use aoc::input_str;
use itertools::Itertools;
use nohash_hasher::IntMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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

        Self {
            width: spaces.len() / height,
            spaces,
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
    }

    fn get_col(&self, x: usize) -> impl ExactSizeIterator<Item = Space> + DoubleEndedIterator + '_ {
        (0..self.height).map(move |y| self.get(x, y))
    }

    fn set_col(&mut self, x: usize, col: &[Space]) {
        debug_assert_eq!(col.len(), self.height);
        for (y, space) in col.iter().enumerate() {
            self.spaces[y * self.width + x] = *space;
        }
    }

    fn slide_north(&mut self) {
        let mut buffer = vec![Space::Empty; self.height];
        for x in 0..self.width {
            buffer.fill(Space::Empty);

            let mut next_location = 0;
            for (index, space) in self.get_col(x).enumerate() {
                if space.is_round() {
                    buffer[next_location] = Space::Round;
                    next_location += 1;
                } else if space.is_square() {
                    buffer[index] = Space::Square;
                    next_location = index + 1;
                }
            }

            self.set_col(x, &buffer);
        }
    }

    fn slide_south(&mut self) {
        let mut buffer = vec![Space::Empty; self.height];
        for x in 0..self.width {
            buffer.fill(Space::Empty);

            let mut next_location = self.height - 1;
            for (index, space) in self.get_col(x).enumerate().rev() {
                if space.is_round() {
                    buffer[next_location] = Space::Round;
                    // Known integer underflow at index 0. However, this is fine
                    // because we're not going to use the value anyway.
                    next_location = next_location.wrapping_sub(1);
                } else if space.is_square() {
                    buffer[index] = Space::Square;
                    next_location = index.wrapping_sub(1);
                }
            }

            self.set_col(x, &buffer);
        }
    }

    fn slide_west(&mut self) {
        let mut buffer = vec![Space::Empty; self.width];
        for y in 0..self.height {
            buffer.fill(Space::Empty);

            let mut next_location = 0;
            for (index, space) in self.get_row(y).iter().enumerate() {
                if space.is_round() {
                    buffer[next_location] = Space::Round;
                    next_location += 1;
                } else if space.is_square() {
                    buffer[index] = Space::Square;
                    next_location = index + 1;
                }
            }

            self.set_row(y, &buffer);
        }
    }

    fn slide_east(&mut self) {
        let mut buffer = vec![Space::Empty; self.width];
        for y in 0..self.height {
            buffer.fill(Space::Empty);

            let mut next_location = self.width - 1;
            for (index, space) in self.get_row(y).iter().enumerate().rev() {
                if space.is_round() {
                    buffer[next_location] = Space::Round;
                    // Known integer underflow at index 0. However, this is fine
                    // because we're not going to use the value anyway.
                    next_location = next_location.wrapping_sub(1);
                } else if space.is_square() {
                    buffer[index] = Space::Square;
                    next_location = index.wrapping_sub(1);
                }
            }

            self.set_row(y, &buffer);
        }
    }

    fn cycle(&mut self) {
        self.slide_north();
        self.slide_west();
        self.slide_south();
        self.slide_east();
    }

    /// Calculates the amount of load on the north beam
    fn north_load(&self) -> usize {
        (0..self.height)
            .map(|y| (self.height - y, self.get_row(y)))
            .map(|(f, row)| f * row.iter().filter(|space| space.is_round()).count())
            .sum()
    }

    fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        // SAFETY: `Space` is repr(u8), which should mean this is fine
        let bytes = unsafe {
            std::slice::from_raw_parts(self.spaces.as_ptr() as *const u8, self.spaces.len())
        };
        hasher.write(bytes);
        hasher.finish()
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
    reflector.north_load()
}

fn part2(mut reflector: Reflector) -> usize {
    let goal = 1_000_000_000;

    let mut seen = IntMap::default();
    seen.insert(reflector.hash(), 0);

    for i in 1.. {
        reflector.cycle();
        if let Some(&j) = seen.get(&reflector.hash()) {
            let remaining = (goal - i) % (i - j);
            for _ in 0..remaining {
                reflector.cycle();
            }
            break;
        }
        seen.insert(reflector.hash(), i);
    }

    reflector.north_load()
}

fn main() {
    let input = input_str!(2023, 14);

    let start = std::time::Instant::now();
    let reflector = Reflector::from_str(input);
    println!("Parse: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(reflector.clone()));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
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
