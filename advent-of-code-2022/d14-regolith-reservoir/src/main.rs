use std::collections::HashSet;

use aoc::get_input;
use either::Either;
use itertools::Itertools;

// Each input line is a Path, which is a list of 2d points
struct Path(Vec<(i32, i32)>);

impl From<&str> for Path {
    fn from(input: &str) -> Self {
        let points = input
            .split(" -> ")
            .map(|point| {
                let mut coords = point.split(',');
                let x = coords.next().unwrap().parse().unwrap();
                let y = coords.next().unwrap().parse().unwrap();
                (x, y)
            })
            .collect();

        Path(points)
    }
}

impl Path {
    // Returns an iterator over the points in the path
    fn points(&self) -> Vec<(i32, i32)> {
        self.0
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|((x1, y1), (x2, y2))| ((*x1, *y1), (*x2, *y2)))
            .flat_map(|((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                    Either::Left((y1..=y2).map(move |y| (x1, y)))
                } else {
                    let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                    Either::Right((x1..=x2).map(move |x| (x, y1)))
                }
            })
            .collect()
    }
}

#[derive(Clone)]
struct Reservoir {
    solids: HashSet<(i32, i32)>,
    max_height: i32,
}

impl std::fmt::Debug for Reservoir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x) = self
            .solids
            .iter()
            .map(|(x, _)| *x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = self
            .solids
            .iter()
            .map(|(_, y)| *y)
            .minmax()
            .into_option()
            .unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.get(x, y) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Reservoir {
    fn new(paths: impl IntoIterator<Item = Path>) -> Self {
        let solids = paths
            .into_iter()
            .flat_map(|path| path.points())
            .collect::<HashSet<_>>();

        let max_height = solids.iter().map(|(_, y)| *y).max().unwrap();

        Reservoir { solids, max_height }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        self.solids.contains(&(x, y))
    }

    fn add_floor(&mut self) {
        // You don't have time to scan the floor, so assume the floor is an infinite horizontal line with a y coordinate equal to two plus the highest y coordinate of any point in your scan.
        // Infinite won't do, however since sand can move at most 1 unit left or right per tick, we can make the floor width just by reusing the max, as width
        let height = self.max_height + 2;

        let start_x = 500 - height;
        let end_x = 500 + height;

        for x in start_x..=end_x {
            self.solids.insert((x, height));
        }

        self.max_height = height;
    }

    // Simulates 1 step of falling sand
    //
    // Returns the new position of the sand, or None if the sand has come to rest
    fn step(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        // A unit of sand always falls down one step if possible
        if !self.get(x, y + 1) {
            return Some((x, y + 1));
        }

        // The unit of sand attempts to instead move diagonally one step down and to the left
        if !self.get(x - 1, y + 1) {
            return Some((x - 1, y + 1));
        }

        // If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right
        if !self.get(x + 1, y + 1) {
            return Some((x + 1, y + 1));
        }

        // Sand has come to rest
        None
    }

    // Performs 1 particle simulation
    fn drop(&mut self) -> Option<(i32, i32)> {
        let mut sand = (500, 0);

        while let Some(new_sand) = self.step(sand.0, sand.1) {
            sand = new_sand;

            // If the sand has fallen below the max height, we're done
            if sand.1 > self.max_height {
                return None;
            }
        }

        Some(sand)
    }

    // Simulates dropping sand until no more sand can come to rest
    fn simulate(&mut self) -> usize {
        let mut count = 0;

        while let Some(sand) = self.drop() {
            if self.solids.contains(&sand) {
                break;
            }

            self.solids.insert(sand);
            count += 1;
        }

        count
    }
}

fn part1(mut reservoir: Reservoir) -> usize {
    reservoir.simulate()
}

fn part2(mut reservoir: Reservoir) -> usize {
    reservoir.add_floor();
    reservoir.simulate()
}

fn main() {
    let input = get_input!(2022, 14);
    let reservoir = Reservoir::new(input.lines().map(Path::from));

    println!("{}", part1(reservoir.clone()));
    println!("{}", part2(reservoir));
}

#[cfg(test)]
mod test {
    use aoc::get_input;

    use crate::{Path, Reservoir};

    // Verify that each path is made of straight lines
    #[test]
    fn test_path() {
        let inputs = get_input!(2022, 14);
        let paths: Vec<Path> = inputs.lines().map(Path::from).collect();

        for path in paths {
            for i in 0..path.0.len() - 1 {
                let (x1, y1) = path.0[i];
                let (x2, y2) = path.0[i + 1];

                assert!(x1 == x2 || y1 == y2);
            }
        }
    }

    #[test]
    fn test_example() {
        let paths = vec![
            Path::from("498,4 -> 498,6 -> 496,6"),
            Path::from("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];

        let mut reservoir = Reservoir::new(paths);

        // The 7 listed points should definitely be solid
        assert!(reservoir.solids.contains(&(498, 4)));
        assert!(reservoir.solids.contains(&(498, 6)));
        assert!(reservoir.solids.contains(&(496, 6)));
        assert!(reservoir.solids.contains(&(503, 4)));
        assert!(reservoir.solids.contains(&(502, 4)));
        assert!(reservoir.solids.contains(&(502, 9)));

        // Part 1
        assert_eq!(reservoir.clone().simulate(), 24);

        // Part 2
        reservoir.add_floor();
        assert_eq!(reservoir.simulate(), 93);
    }

    #[test]
    fn test_part1() {
        let inputs = get_input!(2022, 14);
        let reservoir = Reservoir::new(inputs.lines().map(Path::from));

        assert_eq!(super::part1(reservoir), 1061);
    }

    #[test]
    fn test_part2() {
        let inputs = get_input!(2022, 14);
        let reservoir = Reservoir::new(inputs.lines().map(Path::from));

        assert_eq!(super::part2(reservoir), 25055);
    }
}
