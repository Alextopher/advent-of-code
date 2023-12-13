use aoc::input_str;

pub fn part1(xs: &[usize], ys: &[usize]) -> usize {
    solve(xs, 1) + solve(ys, 1)
}

pub fn part2(xs: &[usize], ys: &[usize]) -> usize {
    solve(xs, 1000000) + solve(ys, 1000000)
}

// Expands the universe along the given axis and the computes the sum of all
// unique pairs of positions.
fn solve(positions: &[usize], scale: usize) -> usize {
    let mut acc = 0;

    let mut last = positions[0];
    let new_positions = positions
        .iter()
        .map(|&p| {
            acc += (p - last).saturating_sub(1) * (scale - 1);
            last = p;
            p + acc
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for i in 0..new_positions.len() {
        for j in i + 1..new_positions.len() {
            sum += new_positions[j] - new_positions[i];
        }
    }
    sum
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .unzip()
}

fn main() {
    let input = input_str!(2023, 11);

    let time = std::time::Instant::now();
    let (mut xs, mut ys) = parse(input);

    // sort xs and ys
    xs.sort_unstable();
    ys.sort_unstable();

    println!("Part 1: {}", part1(&xs, &ys));
    println!("Part 2: {}", part2(&xs, &ys));
    println!("Time: {:?}", time.elapsed());
}
