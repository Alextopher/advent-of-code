fn brute_force(time: i64, distance: i64) -> u64 {
    (0..time).filter(|&t| t * (time - t) > distance).count() as u64
}

fn part1(inputs: impl Iterator<Item = (i64, i64)>) -> u64 {
    inputs
        .map(|(time, distance)| brute_force(time, distance))
        .product::<u64>()
}

fn main() {
    let input_one = &[(61, 430), (67, 1036), (75, 1307), (71, 1150)];
    let input_two = (61677571, 430103613071150);

    println!("Part 1: {}", part1(input_one.iter().copied()));
    println!("Part 2: {}", brute_force(input_two.0, input_two.1));
}
