use std::ops::Range;

use aoc::input_str;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Symbol(char),
    Empty,
    Digit(u32),
}

impl Cell {
    fn is_digit(&self) -> bool {
        matches!(self, Cell::Digit(_))
    }

    fn is_symbol(&self) -> bool {
        matches!(self, Cell::Symbol(_))
    }

    fn unwrap_digit(&self) -> u32 {
        match self {
            Cell::Digit(d) => *d,
            _ => panic!("Cell is not a digit"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in input.lines() {
            height += 1;
            width = 0;
            for c in line.chars() {
                width += 1;
                cells.push(match c {
                    '.' => Cell::Empty,
                    '0'..='9' => Cell::Digit(c.to_digit(10).unwrap()),
                    _ => Cell::Symbol(c),
                });
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn get_coords(&self, x: usize, y: usize) -> Cell {
        self.cells[self.index(x, y)]
    }

    fn get_index(&self, index: usize) -> Cell {
        self.cells[index]
    }

    // 8-way adjacent neighbors of a cell
    fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::with_capacity(8);

        if x > 0 {
            neighbors.push((x - 1, y));

            if y > 0 {
                neighbors.push((x - 1, y - 1));
            }

            if y < self.height - 1 {
                neighbors.push((x - 1, y + 1));
            }
        }

        if x < self.width - 1 {
            neighbors.push((x + 1, y));

            if y > 0 {
                neighbors.push((x + 1, y - 1));
            }

            if y < self.height - 1 {
                neighbors.push((x + 1, y + 1));
            }
        }

        if y > 0 {
            neighbors.push((x, y - 1));
        }

        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    // 8-way adjacent cells of a number
    fn long_adjacent(&self, range: Range<usize>) -> Vec<Cell> {
        // iter<index> -> iter<(x, y)>
        let positions: Vec<_> = range.map(|i| self.coords(i)).collect();

        let mut adjacent = Vec::new();

        // First digit has (as extra) top-left, top, bottom-left
        let (x, y) = positions[0];
        if x > 0 {
            adjacent.push(self.get_coords(x - 1, y));

            if y > 0 {
                adjacent.push(self.get_coords(x - 1, y - 1));
            }

            if y < self.height - 1 {
                adjacent.push(self.get_coords(x - 1, y + 1));
            }
        }

        // Each digit has top and bottom
        for (x, y) in positions.iter().copied() {
            if y > 0 {
                adjacent.push(self.get_coords(x, y - 1));
            }

            if y < self.height - 1 {
                adjacent.push(self.get_coords(x, y + 1));
            }
        }

        // Last digit has (as extra) top-right, bottom-right, right
        let (x, y) = positions[positions.len() - 1];
        if x < self.width - 1 {
            adjacent.push(self.get_coords(x + 1, y));

            if y > 0 {
                adjacent.push(self.get_coords(x + 1, y - 1));
            }

            if y < self.height - 1 {
                adjacent.push(self.get_coords(x + 1, y + 1));
            }
        }

        adjacent
    }

    // Returns an iterator over all the numbers in the grid joins neighboring digits together
    fn numbers(&self) -> Vec<(Range<usize>, u32)> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, c)| c.is_digit())
            .fold(vec![], |mut acc, (index, c)| {
                if let Some((range, n)) = acc.last_mut() {
                    if range.end == index && self.coords(index).1 == self.coords(range.start).1 {
                        *range = range.start..index + 1;
                        *n = *n * 10 + c.unwrap_digit();
                        return acc;
                    }
                }
                acc.push((index..index + 1, c.unwrap_digit()));
                acc
            })
    }

    // Part 1
    fn part1(&self) -> u32 {
        self.numbers()
            .iter()
            .filter(|(r, _)| self.long_adjacent(r.clone()).iter().any(|c| c.is_symbol()))
            .map(|(_, n)| n)
            .sum()
    }

    // Part 2
    fn part2(&self) -> u32 {
        let numbers = self.numbers();

        // Subroutine that finds adjacent long numbers
        let adjacent_numbers = |index: usize| {
            use std::cmp::Ordering;

            let (x, y) = self.coords(index);
            self.neighbors(x, y)
                .iter()
                .filter_map(|(x, y)| {
                    // Finds the number (if any) that contains the cell at (x, y)
                    numbers
                        .binary_search_by(|(r, _)| {
                            if r.contains(&self.index(*x, *y)) {
                                Ordering::Equal
                            } else if r.start > self.index(*x, *y) {
                                Ordering::Greater
                            } else {
                                Ordering::Less
                            }
                        })
                        .ok()
                        .map(|i| numbers[i].1)
                })
                .fold(vec![], |mut acc, n| {
                    if !acc.contains(&n) {
                        acc.push(n);
                    }
                    acc
                })
        };

        self.cells
            .iter()
            .enumerate()
            .filter(|(i, _)| matches!(self.get_index(*i), Cell::Symbol('*')))
            .map(|(index, _)| adjacent_numbers(index))
            .filter_map(|nums| {
                if nums.len() == 2 {
                    Some(nums[0] * nums[1])
                } else {
                    None
                }
            })
            .sum()
    }
}

fn main() {
    let input = input_str!(2023, 3);
    let grid = Grid::new(input);

    // time
    let start = std::time::Instant::now();
    println!("Part 1: {}", grid.part1());
    println!("Part 2: {}", grid.part2());
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let inputs = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let grid = Grid::new(inputs);
        assert_eq!(grid.part1(), 4361);
        assert_eq!(grid.part2(), 467835);
    }
}
