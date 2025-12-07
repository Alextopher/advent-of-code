use std::i32;

use aoc::{input_str, time};

const WIDTH: u32 = 101;
const HEIGHT: u32 = 103;

fn parse_input(input: &str) -> ((u32, u32), (u32, u32)) {
    // Split the input by spaces to separate "p=" and "v="
    let parts: Vec<&str> = input.split_whitespace().collect();

    // Extract and parse the "p=" part
    let p_str = parts[0].trim_start_matches("p=");
    let p: (u32, u32) = {
        let coords: Vec<u32> = p_str.split(',').map(|s| s.parse().unwrap()).collect();
        (coords[0] % WIDTH, coords[1] % HEIGHT)
    };

    // Extract and parse the "v=" part
    let v_str = parts[1].trim_start_matches("v=");
    let v: (u32, u32) = {
        let coords: Vec<i32> = v_str.split(',').map(|s| s.parse().unwrap()).collect();
        // coords are itialized as signed integers, but we need to make them positive
        // so we convert them to unsigned integers by finding the modular inverse
        let vx = if coords[0] < 0 {
            (WIDTH as i32 + coords[0]) as u32
        } else {
            coords[0] as u32
        };

        let vy = if coords[1] < 0 {
            (HEIGHT as i32 + coords[1]) as u32
        } else {
            coords[1] as u32
        };

        (vx, vy)
    };
    (p, v)
}

fn simulate_p1((x, y): (u32, u32), (vx, vy): (u32, u32)) -> (u32, u32) {
    const N: u32 = 100;
    ((x + N * vx) % WIDTH, (y + N * vy) % HEIGHT)
}

// Points between quadrants do not count
fn quadrant(x: u32, y: u32) -> Option<i32> {
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    if x < mid_x && y < mid_y {
        Some(1)
    } else if x > mid_x && y < mid_y {
        Some(2)
    } else if x < mid_x && y > mid_y {
        Some(3)
    } else if x > mid_x && y > mid_y {
        Some(4)
    } else {
        None
    }
}

fn part1(input: &str) -> usize {
    let (q1, q2, q3, q4) = input
        .lines()
        .map(|l| parse_input(l))
        .map(|(p, v)| simulate_p1(p, v))
        .filter_map(|p| quadrant(p.0, p.1))
        .fold((0, 0, 0, 0), |(q1, q2, q3, q4), quad| match quad {
            1 => (q1 + 1, q2, q3, q4),
            2 => (q1, q2 + 1, q3, q4),
            3 => (q1, q2, q3 + 1, q4),
            4 => (q1, q2, q3, q4 + 1),
            _ => unreachable!(),
        });

    q1 * q2 * q3 * q4
}

fn step((x, y): (u32, u32), (vx, vy): (u32, u32)) -> (u32, u32) {
    ((x + vx) % WIDTH, (y + vy) % HEIGHT)
}

fn part2(input: &str) -> (usize, Vec<((u32, u32), (u32, u32))>) {
    let mut robots: Vec<_> = input.lines().map(|l| parse_input(l)).collect();
    let mut min_robots = vec![];
    let mut min_score = i32::MAX;
    let mut best_i = 0;
    for i in 1..10000 {
        robots.iter_mut().for_each(|(p, v)| *p = step(*p, *v));
        let (q1, q2, q3, q4) = robots.iter().map(|(p, _)| quadrant(p.0, p.1)).fold(
            (0, 0, 0, 0),
            |(q1, q2, q3, q4), quad| match quad {
                Some(1) => (q1 + 1, q2, q3, q4),
                Some(2) => (q1, q2 + 1, q3, q4),
                Some(3) => (q1, q2, q3 + 1, q4),
                Some(4) => (q1, q2, q3, q4 + 1),
                _ => (q1, q2, q3, q4),
            },
        );
        let score = q1 * q2 * q3 * q4;

        if score < min_score {
            min_score = score;
            min_robots = robots.clone();
            best_i = i;
        }
    }

    (best_i, min_robots)
}

fn main() {
    let input = input_str!(2024, 14);

    let part1 = time("Part 1", || part1(input));
    println!("Part 1: {}", part1);

    let (seconds, robots) = time("Part 2", || part2(input));
    println!("Part 2: {}", seconds);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if robots.iter().any(|(p, _)| *p == (x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
