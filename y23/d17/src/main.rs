use aoc::input_str;
use smallvec::SmallVec;

fn is_straight_line(path: &[(usize, usize)]) -> bool {
    debug_assert!(path.len() == 3);

    let (x1, y1) = path[0];
    let (x2, y2) = path[1];
    let (x3, y3) = path[2];

    if x1 == x2 && x2 == x3 {
        // Vertical line
        y1 + 1 == y2 && y2 + 1 == y3
    } else if y1 == y2 && y2 == y3 {
        // Horizontal line
        x1 + 1 == x2 && x2 + 1 == x3
    } else {
        false
    }
}

struct City {
    blocks: Vec<u32>,
    height: usize,
    width: usize,
}

impl City {
    fn from_str(input: &str) -> Self {
        let mut height = 0;
        let blocks = input
            .lines()
            .flat_map(|line| {
                height += 1;
                line.chars().map(|c| c.to_digit(10).unwrap())
            })
            .collect::<Vec<_>>();

        let width = blocks.len() / height;

        Self {
            blocks,
            height,
            width,
        }
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.blocks[y * self.width + x]
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn neighbors(&self, x: usize, y: usize) -> SmallVec<[(usize, usize); 4]> {
        let mut neighbors = SmallVec::new();

        if x > 0 {
            neighbors.push((x - 1, y));
        }
        if x < self.width - 1 {
            neighbors.push((x + 1, y));
        }
        if y > 0 {
            neighbors.push((x, y - 1));
        }
        if y < self.height - 1 {
            neighbors.push((x, y + 1));
        }

        neighbors
    }

    /// Finds the shortest path from (0, 0) to (width - 1, height - 1) using dfs
    /// and preventing straight lines of length 3 or more.
    fn dfs(
        &self,
        current: (usize, usize),
        visited: &mut Vec<bool>,
        path: &mut Vec<(usize, usize)>,
        cost: u32,
    ) -> u32 {
        println!("{} {}", current.0, current.1);

        let index = self.index(current.0, current.1);
        visited[index] = true;
        path.push(current);

        // Check if the last three points are in a straight line
        if path.len() >= 3 && is_straight_line(&path[path.len() - 3..]) {
            visited[index] = false;
            return u32::MAX;
        }

        if current == (self.width - 1, self.height - 1) {
            return cost;
        }

        let mut min_cost = u32::MAX;
        for neighbor in self.neighbors(current.0, current.1) {
            let index = self.index(neighbor.0, neighbor.1);
            if !visited[index] {
                let new_cost = cost + self.get(neighbor.0, neighbor.1);
                let cost_of_path = self.dfs(neighbor, visited, path, new_cost);
                min_cost = min_cost.min(cost_of_path);
            }
        }

        path.pop();
        visited[index] = false;
        min_cost
    }
}

fn part1(city: &City) -> u32 {
    let mut visited = vec![false; city.blocks.len()];

    let mut path = Vec::new();
    city.dfs((0, 0), &mut visited, &mut path, 0)
}

fn main() {
    let input = input_str!(2023, 17);
    let city = City::from_str(input);

    println!("Part 1: {}", part1(&city));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_example() {
        let input = "2413432311323\n3215453535623\n3255245654254\n3446585845452\n4546657867536\n1438598798454\n4457876987766\n3637877979653\n4654967986887\n4564679986453\n1224686865563\n2546548887735\n4322674655533";
        let city = City::from_str(input);

        assert_eq!(part1(&city), 102);
    }
}
