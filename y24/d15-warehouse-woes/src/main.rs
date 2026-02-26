use std::collections::HashMap;

use aoc::input_str;

fn part1(input: &str) -> i32 {
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let mut robot = None;
    let mut game = HashMap::new();
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            game.insert((x as i32, y as i32), c);
            if c == '@' {
                robot = Some((x as i32, y as i32));
            }
        }
    }

    let mut robot = robot.unwrap();

    for dir in instructions.chars() {
        let (dx, dy) = match dir {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => continue,
        };

        if try_basic(&mut game, robot, (dx, dy), '.') {
            robot = (robot.0 + dx, robot.1 + dy);
        }
    }

    game.iter()
        .filter(|(_, t)| **t == 'O')
        .map(|((x, y), _)| y * 100 + x)
        .sum()
}

fn part2(input: &str) -> i32 {
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let mut robot = None;
    let mut game = HashMap::new();
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (a, b) = match c {
                '#' => ('#', '#'),
                'O' => ('[', ']'),
                '.' => ('.', '.'),
                '@' => ('@', '.'),
                _ => unreachable!("unexpected tile"),
            };

            game.insert((2 * x as i32, y as i32), a);
            game.insert((2 * x as i32 + 1, y as i32), b);

            if a == '@' {
                robot = Some((2 * x as i32, y as i32));
            }
        }
    }

    let mut robot = robot.unwrap();

    for dir in instructions.chars() {
        let (dx, dy, use_advanced) = match dir {
            '^' => (0, -1, true),
            '>' => (1, 0, false),
            'v' => (0, 1, true),
            '<' => (-1, 0, false),
            _ => continue,
        };

        if use_advanced {
            if advanced_check(&game, robot, dy) {
                assert_eq!(dx, 0);
                advanced_move(&mut game, robot, dy, '.');
                robot = (robot.0 + dx, robot.1 + dy);
            }
        } else if try_basic(&mut game, robot, (dx, dy), '.') {
            robot = (robot.0 + dx, robot.1 + dy);
        }
    }

    game.iter()
        .filter(|(_, t)| **t == '[')
        .map(|((x, y), _)| y * 100 + x)
        .sum()
}

fn advanced_check(game: &HashMap<(i32, i32), char>, (x, y): (i32, i32), dy: i32) -> bool {
    match game.get(&(x, y)).copied().unwrap() {
        '#' => false,
        '.' => true,
        '[' => advanced_check(game, (x, y + dy), dy) && advanced_check(game, (x + 1, y + dy), dy),
        ']' => advanced_check(game, (x - 1, y + dy), dy) && advanced_check(game, (x, y + dy), dy),
        '@' => advanced_check(game, (x, y + dy), dy),
        _ => unreachable!(),
    }
}

fn advanced_move(game: &mut HashMap<(i32, i32), char>, (x, y): (i32, i32), dy: i32, parent: char) {
    let tile = game.get(&(x, y)).copied().unwrap();
    match tile {
        '.' => {
            game.insert((x, y), parent);
        }
        '[' => {
            assert!(game.get(&(x + 1, y)).unwrap() == &']');

            advanced_move(game, (x, y + dy), dy, '[');
            game.insert((x, y), parent);

            advanced_move(game, (x + 1, y + dy), dy, ']');
            game.insert((x + 1, y), '.');
        }
        ']' => {
            assert!(game.get(&(x - 1, y)).unwrap() == &'[');

            advanced_move(game, (x, y + dy), dy, ']');
            game.insert((x, y), parent);

            advanced_move(game, (x - 1, y + dy), dy, '[');
            game.insert((x - 1, y), '.');
        }
        '@' => {
            advanced_move(game, (x, y + dy), dy, '@');

            game.insert((x, y), parent);
        }
        _ => unreachable!(),
    };
}

fn try_basic(
    game: &mut HashMap<(i32, i32), char>,
    (x, y): (i32, i32),
    (dx, dy): (i32, i32),
    parent: char,
) -> bool {
    let tile = game.get(&(x, y)).copied().unwrap();

    // base case: if we hit a static wall or the edge of the map we fail the move
    if tile == '#' {
        return false;
    }
    // base case: if we hit an empty space we succeed the move
    if tile == '.' {
        // x, y becomes the parent tile
        game.insert((x, y), parent);
        return true;
    }

    // recursive
    if try_basic(game, (x + dx, y + dy), (dx, dy), tile) {
        game.insert((x, y), parent);
        return true;
    }

    false
}

fn main() {
    let input = input_str!(2024, 15);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_small_example() {
        // read example.txt as str
        let input = include_str!("../example_small.txt");
        assert_eq!(part1(input), 2028);
    }

    #[test]
    fn part1_large_example() {
        // read example.txt as str
        let input = include_str!("../example.txt");
        assert_eq!(part1(input), 10092);
    }

    #[test]
    fn part1_test() {
        let input = input_str!(2024, 15);
        assert_eq!(part1(input), 1465152);
    }

    #[test]
    fn part2_small_example() {
        let input = include_str!("../example_p2_small.txt");
        part2(input);
    }

    #[test]
    fn part2_large_example() {
        let input = include_str!("../example.txt");
        assert_eq!(part2(input), 9021);
    }

    #[test]
    fn part2_test() {
        let input = input_str!(2024, 15);
        assert_eq!(part2(input), 1511259);
    }
}
