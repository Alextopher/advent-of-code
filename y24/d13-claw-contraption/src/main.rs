use aoc::{input_str, time};

// Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: a: X+94, c: Y+34
// Button B: b: X+22, d: Y+67
// Prize: X=8400, Y=5400

#[derive(Debug, Clone, Copy)]
struct Problem {
    // a b c d
    matrix: [isize; 4],
    x: isize,
    y: isize,
}

impl Problem {
    fn solve_p1(&self) -> Option<(usize, usize)> {
        println!();
        let det = self.matrix[0] * self.matrix[3] - self.matrix[1] * self.matrix[2];
        println!("{}", det);
        if det == 0 {
            println!("Determinant is 0");
            return None;
        }

        let inv = [
            self.matrix[3],
            -self.matrix[1],
            -self.matrix[2],
            self.matrix[0],
        ];

        println!("{:?}", inv);

        let a = self.x * inv[0] + self.y * inv[1];
        let b = self.x * inv[2] + self.y * inv[3];

        println!("a: {}, b: {}", a, b);
        println!("a: {}, b: {}", a / det, b / det);
        println!("a: {}, b: {}", a % det, b % det);

        // 17990 - low
        if a % det == 0 && b % det == 0 {
            Some(((a / det) as usize, (b / det) as usize))
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Vec<Problem> {
    let re = regex::Regex::new(r"(\d+)").unwrap();
    let lines: Vec<_> = input.lines().collect();
    lines
        .chunks(4)
        .map(|l| {
            let cap = re.captures_iter(l[0]).collect::<Vec<_>>();

            let a = cap[0].get(0).unwrap().as_str().parse().unwrap();
            let b = cap[1].get(1).unwrap().as_str().parse().unwrap();

            let cap = re.captures_iter(l[1]).collect::<Vec<_>>();
            let c = cap[0].get(0).unwrap().as_str().parse().unwrap();
            let d = cap[1].get(1).unwrap().as_str().parse().unwrap();

            let cap = re.captures_iter(l[2]).collect::<Vec<_>>();
            let x = cap[0].get(0).unwrap().as_str().parse().unwrap();
            let y = cap[1].get(1).unwrap().as_str().parse().unwrap();

            Problem {
                matrix: [a, c, b, d],
                x,
                y,
            }
        })
        .collect()
}

fn part1(problems: &[Problem]) -> usize {
    problems
        .iter()
        .filter_map(|p| p.solve_p1())
        .map(|(a, b)| 3 * a + b)
        .sum()
}

fn part2(problems: &[Problem]) -> usize {
    problems
        .iter()
        .map(|p| Problem {
            matrix: p.matrix,
            x: p.x + 10000000000000,
            y: p.y + 10000000000000,
        })
        .filter_map(|p| p.solve_p1())
        .map(|(a, b)| 3 * a + b)
        .sum()
}

fn main() {
    let input = input_str!(2024, 13);
    let problems = time("Parsed", || parse(input));
    let part1 = time("Part 1", || part1(&problems));
    println!("Part 1: {}", part1);
    let part2 = time("Part 2", || part2(&problems));
    println!("Part 1: {}", part2);
}
