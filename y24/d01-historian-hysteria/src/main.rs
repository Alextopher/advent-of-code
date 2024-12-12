use std::collections::HashMap;

use aoc::input_str;

fn part1(input: &str) -> i32 {
    let (mut lefts, mut rights) = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .fold((vec![], vec![]), |(mut lefts, mut rights), (l, r)| {
            lefts.push(l);
            rights.push(r);
            (lefts, rights)
        });

    lefts.sort_unstable();
    rights.sort_unstable();

    lefts
        .iter()
        .zip(rights.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part2(input: &str) -> usize {
    let (lefts, rights): (HashMap<usize, usize>, HashMap<usize, usize>) = input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();
            (
                iter.next().unwrap().parse::<usize>().unwrap(),
                iter.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .fold(
            (Default::default(), Default::default()),
            |(mut lefts, mut rights), (l, r)| {
                *lefts.entry(l).or_insert(0) += 1;
                *rights.entry(r).or_insert(0) += 1;
                (lefts, rights)
            },
        );

    lefts
        .into_iter()
        .map(|(id, l_count)| id * l_count * rights.get(&id).unwrap_or(&0))
        .sum()
}

fn main() {
    let input = input_str!(2024, 1);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn test_part2() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part2(input), 31);
    }
}
