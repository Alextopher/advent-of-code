use aoc::get_lines;
use itertools::Itertools;

fn main() {
    let ans: i32 = get_lines("input.txt")
        .map(|s| {
            let chars: Vec<char> = s.chars().collect();
            let len = chars.len();

            let first_half: String = chars.iter().take(len / 2).collect();
            let second_half: String = chars.iter().skip(len / 2).collect();

            (first_half, second_half)
        })
        .map(|(l, r)| {
            for c in l.chars() {
                if r.contains(c) {
                    if c.is_uppercase() {
                        return c as i32 - 'A' as i32 + 27;
                    } else {
                        return c as i32 - 'a' as i32 + 1;
                    }
                }
            }
            0
        })
        .sum();

    println!("{ans}");

    let ans: i32 = get_lines("input.txt").chunks(3).into_iter().map(|k| {
        let groups : Vec<String> = k.collect();
        // find element in g[0] that is in g[1] and g[2]
        for c in groups[0].chars() {
            if groups[1].contains(c) && groups[2].contains(c) {
                    if c.is_uppercase() {
                        return c as i32 - 'A' as i32 + 27;
                    } else {
                        return c as i32 - 'a' as i32 + 1;
                    }
            }
        }
        0
    }).sum();

    println!("{ans}");
}
