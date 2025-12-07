use std::collections::{HashMap, HashSet};

use aoc::input_str;

fn parse_input(input: &str) -> (usize, Vec<&str>) {
    let mut lines = input.lines();

    // Find the starting position 'S' in the first line
    let first_line = lines.next().unwrap();
    let start_pos = first_line.find('S').unwrap();
    lines.next();

    let lab_lines: Vec<&str> = lines.step_by(2).collect();

    (start_pos, lab_lines)
}

fn get_split_positions(line: &str) -> HashSet<usize> {
    line.chars()
        .enumerate()
        .filter(|&(_, c)| c == '^')
        .map(|(i, _)| i)
        .collect()
}

fn part1(input: &str) -> usize {
    let (start_pos, lab_lines) = parse_input(input);

    let mut current_beams = HashSet::new();
    current_beams.insert(start_pos);

    let mut total_splits = 0;

    for line in lab_lines {
        let split_positions = get_split_positions(line);
        let mut next_beams = HashSet::new();

        for &beam_pos in &current_beams {
            if split_positions.contains(&beam_pos) {
                total_splits += 1;
                next_beams.insert(beam_pos - 1);
                next_beams.insert(beam_pos + 1);
            } else {
                next_beams.insert(beam_pos);
            }
        }

        current_beams = next_beams;
    }

    total_splits
}

fn part2(input: &str) -> usize {
    let (start_pos, lab_lines) = parse_input(input);

    let mut beam_paths: HashMap<usize, usize> = HashMap::new();
    beam_paths.insert(start_pos, 1);

    for line in lab_lines {
        let split_positions = get_split_positions(line);
        let mut next_beam_paths: HashMap<usize, usize> = HashMap::new();

        for (&beam_pos, &path_count) in &beam_paths {
            if split_positions.contains(&beam_pos) {
                // split
                *next_beam_paths.entry(beam_pos - 1).or_insert(0) += path_count;
                *next_beam_paths.entry(beam_pos + 1).or_insert(0) += path_count;
            } else {
                // continue
                *next_beam_paths.entry(beam_pos).or_insert(0) += path_count;
            }
        }

        beam_paths = next_beam_paths;
    }

    // Sum all paths that reach the end
    beam_paths.values().sum()
}

fn main() {
    let input = input_str!(2025, 7);

    let time = std::time::Instant::now();
    println!("Part 1: {}", part1(&input));
    println!("Time: {:?}", time.elapsed());

    let time = std::time::Instant::now();
    println!("Part 2: {}", part2(&input));
    println!("Time: {:?}", time.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = input_str!(2025, 7);
        assert_eq!(part1(&input), 1550);
    }

    #[test]
    fn test_part2() {
        let input = input_str!(2025, 7);
        assert_eq!(part2(&input), 9897897326778);
    }
}
