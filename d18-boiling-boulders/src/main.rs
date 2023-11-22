use std::{collections::HashSet, ops::Range};

#[derive(Debug)]
struct BoundingBox(Range<i32>, Range<i32>, Range<i32>);

impl BoundingBox {
    fn new(xs: Range<i32>, ys: Range<i32>, zs: Range<i32>) -> Self {
        Self(xs, ys, zs)
    }

    fn contains(&self, x: i32, y: i32, z: i32) -> bool {
        self.0.contains(&x) && self.1.contains(&y) && self.2.contains(&z)
    }

    fn iter(&self) -> impl Iterator<Item = (i32, i32, i32)> {
        let mut voxels = vec![];
        for i in self.0.clone() {
            for j in self.1.clone() {
                for k in self.2.clone() {
                    voxels.push((i, j, k));
                }
            }
        }

        return voxels.into_iter();
    }
}

// Boulder is made up of 3d voxel cube
#[derive(Debug)]
struct Boulder {
    // 3d array of voxels
    voxels: HashSet<(i32, i32, i32)>,
}

impl Boulder {
    // Parses a boulder from a string
    fn from(input: &str) -> Self {
        let voxels = input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();
                let z = parts.next().unwrap().parse().unwrap();
                (x, y, z)
            })
            .collect();

        Self { voxels }
    }

    // Returns true if the given voxel is solid
    fn is_solid(&self, x: i32, y: i32, z: i32) -> bool {
        self.voxels.contains(&(x, y, z))
    }

    // Returns a list of neighboring voxels
    fn neighbors(&self, x: i32, y: i32, z: i32) -> impl IntoIterator<Item = (i32, i32, i32)> {
        [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
    }

    // Returns the surface area of the boulder (part1)
    fn surface_area(&self) -> usize {
        self.voxels
            .iter()
            .map(|&(x, y, z)| self.neighbors(x, y, z))
            .flatten()
            .filter(|&(x, y, z)| !self.is_solid(x, y, z))
            .count()
    }

    // Creates a bounding box of the bolder
    fn bounding_box(&self) -> BoundingBox {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;
        let mut z_min = i32::MAX;
        let mut z_max = i32::MIN;

        for &(x, y, z) in &self.voxels {
            x_min = x_min.min(x);
            x_max = x_max.max(x);
            y_min = y_min.min(y);
            y_max = y_max.max(y);
            z_min = z_min.min(z);
            z_max = z_max.max(z);
        }

        BoundingBox::new(x_min..x_max, y_min..y_max, z_min..z_max)
    }

    // Any air voxel that has a direct path leading outside of the bounding box turns to stone
    fn fill(&mut self) {
        let bounding = self.bounding_box();

        let mut work: HashSet<(i32, i32, i32)> = bounding
            .iter()
            .filter(|&(x, y, z)| !self.is_solid(x, y, z))
            .collect();

        while let Some((x, y, z)) = work.iter().next().cloned() {
            work.remove(&(x, y, z));

            // Check if there is a path from this voxel to the outside
            let mut visited = HashSet::new();
            let mut queue = vec![(x, y, z)];
            let mut found = false;

            while let Some((x, y, z)) = queue.pop() {
                if !bounding.contains(x, y, z) {
                    found = true;
                    break;
                }

                if visited.contains(&(x, y, z)) {
                    continue;
                }

                visited.insert((x, y, z));

                queue.extend(
                    self.neighbors(x, y, z)
                        .into_iter()
                        .filter(|&(x, y, z)| !self.is_solid(x, y, z)),
                );
            }

            for voxel in visited.drain() {
                work.remove(&voxel);

                if !found {
                    self.voxels.insert(voxel);
                }
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let boulder = Boulder::from(input);
    boulder.surface_area()
}

fn part2(input: &str) -> usize {
    let mut boulder = Boulder::from(input);
    boulder.fill();
    boulder.surface_area()
}

fn main() {
    println!("{}", part1(include_str!("../input.txt")));
    println!("{}", part2(include_str!("../input.txt")));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../example.txt");
        assert_eq!(part1(input), 64);
        assert_eq!(part2(input), 58);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../input.txt")), 4242);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input.txt")), 2428);
    }
}
