use aoc::input_str;

// 
fn parse_round(line: &str) -> (usize, usize, usize) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for color in line.split(',').map(str::trim) {
        let (num, color) = color.split_at(color.find(' ').unwrap());
        let num = num.parse().unwrap();

        match color.trim() {
            "red" => red = num,
            "green" => green = num,
            "blue" => blue = num,
            _ => panic!("Unknown color: {}", color),
        }
    }

    (red, green, blue)
}

fn parse_game(line: &str) -> Vec<(usize, usize, usize)> {
    // Remove `Game N: `
    let colon = line.find(':').unwrap();
    line[colon + 1..]
        .split(';')
        .map(str::trim)
        .map(parse_round)
        .collect()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_game)
        .zip(1..)
        .filter(|(rounds, _)| !rounds.iter().any(|(r, g, b)| *r > 12 || *g > 13 || *b > 14))
        .map(|(_, i)| i)
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_game)
        .map(|rounds| {
            rounds.iter().fold((0, 0, 0), |(mr, mg, mb), (r, g, b)| {
                (mr.max(*r), mg.max(*g), mb.max(*b))
            })
        })
        .map(|(r, g, b)| r * g * b)
        .sum()
}

fn main() {
    let input = input_str!(2023, 2);
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(part1(input), 8);
    }
}
