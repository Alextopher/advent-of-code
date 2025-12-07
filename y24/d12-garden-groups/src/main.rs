use aoc::{cartesian_product, input_str, time};

pub struct Grid<'a> {
    data: &'a [u8],
    width: usize,
    height: usize,
}

impl<'a> Grid<'a> {
    fn new(data: &'a [u8], width: usize, height: usize) -> Self {
        Self {
            data,
            width,
            height,
        }
    }

    /// Counts the number of 'outside edges' for a given tile
    fn edges(&self, x: usize, y: usize) -> usize {
        let mut total = 0;
        if x == 0 || x == self.width - 1 {
            total += 1;
        }
        if y == 0 || y == self.height - 1 {
            total += 1;
        }
        total
    }

    fn neighbors(&self, x: usize, y: usize) -> [Option<(usize, usize)>; 4] {
        let mut neighbors = [None; 4];

        if x > 0 {
            neighbors[0] = Some((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors[1] = Some((x + 1, y));
        }
        if y > 0 {
            neighbors[2] = Some((x, y - 1));
        }
        if y < self.height - 1 {
            neighbors[3] = Some((x, y + 1));
        }

        neighbors
    }

    fn vertices_idx(&self, x: usize, y: usize) -> [(usize, Option<usize>); 4] {
        let idx = y * (self.width + 1) + x;

        let mut results = [
            (idx + y, None),                  // A
            (idx + y + 1, None),              // B
            (idx + y + self.width + 2, None), // C
            (idx + y + self.width + 3, None), // D
        ];

        if x > 0 && y > 0 {
            results[0].1 = Some((y - 1) * (self.width + 1) + (x - 1));
        }
        if x < self.width - 1 && y > 0 {
            results[1].1 = Some((y - 1) * (self.width + 1) + (x + 1));
        }
        if x > 0 && y < self.height - 1 {
            results[2].1 = Some((y + 1) * (self.width + 1) + (x - 1));
        }
        if x < self.width - 1 && y < self.height - 1 {
            results[3].1 = Some((y + 1) * (self.width + 1) + (x + 1));
        }

        results
    }
}

fn parse(input: &str) -> Grid {
    let height = input.bytes().filter(|b| *b == b'\n').count();
    let width = input.lines().next().unwrap().len();
    let grid = Grid::new(input.as_bytes(), width, height);
    grid
}

fn part1(grid: &Grid) -> usize {
    let mut visited = vec![false; (grid.width + 1) * grid.height];
    let mut total_cost = 0;

    for (x, y) in cartesian_product(0..grid.width, 0..grid.height) {
        let idx = y * (grid.width + 1) + x;
        if visited[idx] {
            continue;
        }

        let crop = grid.data[idx];
        let (mut area, mut perimeter) = (0, 0);
        // invariant: stack only contains tiles of the same crop
        let mut stack = vec![(x, y)];
        while let Some((x, y)) = stack.pop() {
            let idx = y * (grid.width + 1) + x;
            if visited[idx] {
                continue;
            }
            visited[idx] = true;
            perimeter += grid.edges(x, y);
            area += 1;

            for (nx, ny) in grid.neighbors(x, y).into_iter().flatten() {
                let nidx = ny * (grid.width + 1) + nx;
                if crop == grid.data[nidx] {
                    stack.push((nx, ny));
                } else {
                    perimeter += 1;
                }
            }
        }
        total_cost += area * perimeter;
        visited[idx] = true;
    }

    total_cost
}

fn part2(grid: &Grid) -> usize {
    let mut visited = vec![false; (grid.width + 1) * grid.height];
    let mut vertices = vec![0; (grid.width + 2) * (grid.height + 1)];
    let mut total_cost = 0;

    for (x, y) in cartesian_product(0..grid.width, 0..grid.height) {
        let idx = y * (grid.width + 1) + x;
        if visited[idx] {
            continue;
        }

        let crop = grid.data[idx];
        let mut area = 0;

        let mut stack = vec![(x, y)];
        while let Some((x, y)) = stack.pop() {
            let idx = y * (grid.width + 1) + x;
            if visited[idx] {
                continue;
            }

            // flip vertices if neigh
            print!("{} {} | ", x, y);
            for (idx, corner) in grid.vertices_idx(x, y) {
                print!(
                    "({} {:?} {:?} {:?}) ",
                    idx,
                    corner,
                    corner.map(|c| char::from_u32(grid.data[c] as u32).unwrap()),
                    corner.map(|c| visited[c])
                );
                if corner.map(|c| grid.data[c]) == Some(crop) {
                    vertices[idx] += 1;
                }
            }
            println!();
            visited[idx] = true;

            // debug print vertices
            for x in 0..grid.width + 1 {
                for y in 0..grid.height + 1 {
                    let idx = y * (grid.width + 2) + x;
                    if vertices[idx] != 0 {
                        print!("{} ", vertices[idx]);
                    } else {
                        print!(". ");
                    }
                }
                println!();
                print!(" ");
                for y in 0..grid.height {
                    let idx = y * (grid.width + 1) + x;
                    if visited[idx] {
                        print!("{} ", char::from_u32(grid.data[idx] as u32).unwrap());
                    } else {
                        print!("  ");
                    }
                }
                println!();
            }
            println!();
            area += 1;

            for (nx, ny) in grid.neighbors(x, y).into_iter().flatten() {
                let nidx = ny * (grid.width + 1) + nx;
                if crop == grid.data[nidx] {
                    stack.push((nx, ny));
                }
            }
        }
        let edges = vertices.iter().filter(|&v| v % 2 == 0).count();
        println!("{} {} {}", area, edges, area * edges);
        total_cost += area * edges;
        vertices.fill(0);
    }

    total_cost
}

fn main() {
    let input = input_str!(2024, 12);

    let map = time("Parsed", || parse(input));
    let part1 = time("Part 1", || part1(&map));
    println!("Part 1: {}", part1);

    let part2 = time("Part 2", || part2(&map));
    println!("Part 2: {}", part2);

    // 898582 - low
}

#[cfg(test)]
mod test {
    use crate::{parse, part1, part2};

    #[test]
    fn examples() {
        let input = include_str!("example_large.txt");
        let map = parse(input);
        let part1 = part1(&map);
        assert_eq!(part1, 1930);
        let part2 = part2(&map);
        assert_eq!(part2, 1206);
    }

    #[test]
    fn test_part2() {
        let examples = [
            // ("AAAA\nBBCD\nBBCC\nEEEC\n", 80),
            // ("EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE\n", 236),
            ("AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA\n", 268),
        ];
        for (input, expected) in examples {
            let map = parse(input);
            let part2 = part2(&map);
            assert_eq!(part2, expected);
            println!()
        }
    }

    #[test]
    fn test_4_corners() {
        let tests = [((0, 0), 10)];

        for ((x, y), w) in tests {
            // original
            let idx = y * w + x;

            // let nidx = (x + dx) * (w + 1) + y + dy;
            let tl = y * (w + 1) + x;
            let tr = y * (w + 1) + (x + 1);
            let bl = (y + 1) * (w + 1) + x;
            let br = (y + 1) * (w + 1) + (x + 1);

            assert_eq!(tl, idx + y);
            assert_eq!(tr, idx + y + 1);
            assert_eq!(bl, idx + y + w + 1);
            assert_eq!(br, idx + y + w + 2);
        }
    }
}
