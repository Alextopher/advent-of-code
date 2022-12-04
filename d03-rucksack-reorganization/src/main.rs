use aoc::get_lines;
use itertools::Itertools;

fn get_mask(s: &str) -> u64 {
    let mut mask = 0;

    s.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c as i32 - 'A' as i32 + 27
            } else {
                c as i32 - 'a' as i32 + 1
            }
        })
        .for_each(|b| mask |= 1 << b);

    mask
}

fn main() {
    let ans: u32 = get_lines("input.txt")
        .map(|s| {
            let split = s.split_at(s.len() / 2);
            get_mask(split.0) & get_mask(split.1)
        })
        .map(u64::trailing_zeros)
        .sum();

    println!("{ans}");

    let ans: u32 = get_lines("input.txt")
        .chunks(3)
        .into_iter()
        .map(|k| {
            let mut mask = u64::MAX;
            k.for_each(|s| mask &= get_mask(&s));
            mask
        })
        .map(u64::trailing_zeros)
        .sum();

    println!("{ans}");
}
