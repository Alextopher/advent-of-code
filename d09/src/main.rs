use aoc::{get_lines, GetMultMut};
use itertools::Itertools;
use std::{
    cmp::{max, min},
    collections::HashSet,
};

fn main() {
    // okay I need a 2d array backed by HashMap
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    // start at 0,0
    let (mut head_x, mut head_y) = (0, 0);
    let (mut tail_x, mut tail_y) = (0, 0);

    grid.insert((0, 0));

    // follow the instructions
    // R 4
    // U 4
    // L 3
    // D 1
    for line in get_lines("example.txt") {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        chars.next(); // skip comma
        let dist = chars.collect::<String>().parse::<usize>().unwrap();

        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("invalid direction"),
        };

        for _ in 0..dist {
            // move the head a square then let the tail follow
            head_x += dx;
            head_y += dy;

            let dx: i32 = head_x - tail_x;
            let dy: i32 = head_y - tail_y;

            // if the tail is 2 squares away from the head, move it
            if dx.abs() > 1 || dy.abs() > 1 {
                tail_x += dx.signum();
                tail_y += dy.signum();

                grid.insert((tail_x, tail_y));
            }
        }
    }

    // count the number of intersections
    let count = grid.iter().count();
    println!("{}", count);

    // Need to do the same thing but now we have 10 connected groups
    let mut grid: HashSet<(i32, i32)> = HashSet::new();

    // start at 0,0
    let mut tails = vec![(0, 0); 10];
    grid.insert(tails[0]);

    // follow the instructions
    for line in get_lines("input.txt") {
        let mut chars = line.chars();
        let dir = chars.next().unwrap();
        chars.next(); // skip comma
        let dist = chars.collect::<String>().parse::<usize>().unwrap();

        let (dx, dy) = match dir {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => panic!("invalid direction"),
        };

        for _ in 0..dist {
            // move tails[0] a square then let the rest follow
            let head = tails.get_mut(0).unwrap();
            *head = (head.0 + dx, head.1 + dy);

            // window over pairs of tails
            for (i, j) in (0..tails.len()).tuple_windows() {
                let (tail, head) = tails.get_mut_2(j, i);

                let dx: i32 = head.0 - tail.0;
                let dy: i32 = head.1 - tail.1;

                // if the tail is 2 squares away from the head, move it
                if dx.abs() > 1 || dy.abs() > 1 {
                    tail.0 += dx.signum();
                    tail.1 += dy.signum();

                    // only track the last tail
                    if j == 9 {
                        println!("{} {}", tail.0, tail.1);
                        grid.insert(*tail);
                    }
                }
            }
        }
    }

    print_grid(&grid, &tails, tails[0]);
    println!("{}", grid.len());
}

fn print_grid(grid: &HashSet<(i32, i32)>, tails: &[(i32, i32)], (head_x, head_y): (i32, i32)) {
    // print the grid
    let min_x = min(*grid.iter().map(|(x, _)| x).min().unwrap(), head_x);
    let max_x = max(*grid.iter().map(|(x, _)| x).max().unwrap(), head_x);
    let min_y = min(*grid.iter().map(|(_, y)| y).min().unwrap(), head_y);
    let max_y = max(*grid.iter().map(|(_, y)| y).max().unwrap(), head_y);

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if x == head_x && y == head_y {
                print!("H");
            } else if let Some((i, _)) = tails.iter().enumerate().find(|(_, p)| **p == (x, y)) {
                print!("{}", i);
            } else if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}
