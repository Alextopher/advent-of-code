use aoc::input_str;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    Filled,
    Unknown,
}

impl Cell {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Filled,
            '?' => Self::Unknown,
            _ => panic!("Invalid character"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Row {
    cells: Vec<Cell>,
    clues: Vec<usize>,
}

impl Row {
    fn from_str(s: &str) -> Self {
        // split at spaces
        let mut groups = s.split_whitespace();

        // parse cells
        let cells = groups
            .next()
            .unwrap()
            .chars()
            .map(Cell::from_char)
            .collect();

        // parse clues
        let clues = groups
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        Self { cells, clues }
    }

    fn duplicate(&mut self) {
        self.cells = self
            .cells
            .iter()
            .cycle()
            .take(self.cells.len() * 5)
            .cloned()
            .collect();

        self.clues = self
            .clues
            .iter()
            .cycle()
            .take(self.clues.len() * 5)
            .cloned()
            .collect();
    }

    // Checks if a set of spaces works for this row.
    fn check(&self, spaces: &[usize]) -> bool {
        let mut spaces = spaces.iter();
        let clues = self.clues.iter();

        // Run the first space manually.
        let mut index = spaces.next().cloned().unwrap();
        if self.cells[0..index].iter().any(|c| *c == Cell::Filled) {
            return false;
        }

        // Loop over the rest of pairs of spaces and clues.
        debug_assert_eq!(spaces.len(), clues.len());

        for (&clue, &space) in clues.zip(spaces) {
            // Check that the clues are filled.
            if self.cells[index..index + clue]
                .iter()
                .any(|c| *c == Cell::Empty)
            {
                return false;
            }

            index += clue;

            // Check that the spaces are empty.
            if self.cells[index..index + space]
                .iter()
                .any(|c| *c == Cell::Filled)
            {
                return false;
            }

            index += space;
        }

        // If all tests passed, then this is a valid solution.
        true
    }

    // Part 1, find number of possible solutions.
    fn part1(&self) -> usize {
        // empty squares + filled squares = total squares
        //
        // or we can rearrange to:
        //
        // empty squares = total squares - filled squares
        let empty_squares = self.cells.len() - self.clues.iter().sum::<usize>();

        // We start by giving each clue a single empty square to its sides.
        let mut spaces = vec![0; self.clues.len() + 1];
        // The middle spaces are at least length 1
        let range = 1..=self.clues.len() - 1;
        spaces[range].iter_mut().for_each(|s| *s = 1);

        // In total we have empty_squares - self.clues.len() spaces to
        // distribute among all the spaces.

        let left_over = empty_squares - (self.clues.len() - 1);

        let set = (0..spaces.len()).collect_vec();
        set.into_iter()
            .combinations_with_replacement(left_over)
            .par_bridge()
            .map(|s| {
                let mut spaces = spaces.clone();
                for i in s.iter() {
                    spaces[*i] += 1;
                }
                spaces
            })
            .filter(|s| {
                // Check that the spaces work.
                self.check(s)
            })
            .count()
    }
}

fn main() {
    let input = input_str!(2023, 12);
    let rows = input.lines().map(Row::from_str).collect::<Vec<_>>();

    println!("Part 1: {}", rows.iter().map(Row::part1).sum::<usize>());

    // Part 2: Duplicated cells and clues by 5
    println!(
        "Part 2: {}",
        rows.into_par_iter()
            .enumerate()
            .map(|(i, mut row)| {
                row.duplicate();
                let result = row.part1();
                println!("{}: {}", i, result);
                result
            })
            .sum::<usize>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_column() {
        // ???.### 1,1,3
        let row = Row::from_str("???.### 1,1,3");
        assert_eq!(
            row.cells,
            vec![
                Cell::Unknown,
                Cell::Unknown,
                Cell::Unknown,
                Cell::Empty,
                Cell::Filled,
                Cell::Filled,
                Cell::Filled
            ]
        );
        assert_eq!(row.clues, vec![1, 1, 3]);
    }

    #[test]
    fn test_example() {
        let input = "???.### 1,1,3\n.??..??...?##. 1,1,3\n?#?#?#?#?#?#?#? 1,3,1,6\n????.#...#... 4,1,1\n????.######..#####. 1,6,5\n?###???????? 3,2,1";
        let rows = input.lines().map(Row::from_str).collect::<Vec<_>>();

        assert_eq!(rows[0].part1(), 1);
        assert_eq!(rows[1].part1(), 4);
        assert_eq!(rows[2].part1(), 1);
        assert_eq!(rows[3].part1(), 1);
        assert_eq!(rows[4].part1(), 4);
        assert_eq!(rows[5].part1(), 10);
    }
}
