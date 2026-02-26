use aoc::input_str;

fn check_part1(line: &str) -> bool {
    let items = line.split_whitespace().collect::<Vec<&str>>();
    let (width, height) = items[0]
        .trim_end_matches(':')
        .split_once('x')
        .map(|(w, h)| (w.parse::<u32>().unwrap(), h.parse::<u32>().unwrap()))
        .unwrap();

    9 * items[1..]
        .iter()
        .map(|&item| item.parse::<u32>().unwrap())
        .sum::<u32>()
        <= width * height
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .skip(30)
        .filter(|line| check_part1(line))
        .count()
}

fn main() {
    let input = input_str!(2025, 12);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:?}", start.elapsed());
}
