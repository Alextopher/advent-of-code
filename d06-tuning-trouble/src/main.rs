use std::time::Instant;

use aoc::*;

fn main() {
    let line = get_lines("input.txt").next().unwrap();

    // track the running time of the program
    let mut time = Instant::now();

    // find the index of first window of 4 unique characters
    let ans = line
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, w)| lowercase_all_unique(w.iter().cloned()))
        .unwrap()
        .0;

    // print the answer and the time it took to find it
    println!("{} {:?}", ans + 4, time.elapsed());

    // groups of 14 characters
    time = Instant::now();
    let ans = line
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, w)| lowercase_all_unique(w.iter().cloned()))
        .unwrap()
        .0;

    // print the answer and the time it took to find it
    println!("{} {:?}", ans + 14, time.elapsed());
}

// Since every character is lowercase ASCII we can use a bitset to check if all characters are unique
fn lowercase_all_unique<T: Iterator<Item = u8>>(iter: T) -> bool {
    let mut set = 0;
    for c in iter {
        let bit = 1 << (c - b'a');
        if set & bit != 0 {
            return false;
        }
        set |= bit;
    }
    true
}
