use std::{collections::VecDeque, time::Instant};

use aoc::*;
use itertools::Itertools;

fn main() {
    let line = get_lines("input.txt").next().unwrap();

    // track the running time of the program
    let mut time = Instant::now();

    // find the index of first window of 4 unique characters
    let ans = line
        .as_bytes()
        .windows(4)
        .enumerate()
        .find(|(_, w)| w.iter().all_unique())
        .unwrap()
        .0;

    // print the answer and the time it took to find it
    println!("{} {:?}", ans, time.elapsed());

    // groups of 14 characters
    time = Instant::now();
    let ans = line
        .as_bytes()
        .windows(14)
        .enumerate()
        .find(|(_, w)| w.iter().all_unique())
        .unwrap()
        .0;

    // print the answer and the time it took to find it
    println!("{} {:?}", ans, time.elapsed());

    time = Instant::now();
    let ans = improved(&line, 4).unwrap();
    println!("{} {:?}", ans, time.elapsed());

    time = Instant::now();
    let ans = improved(&line, 14).unwrap();
    println!("{} {:?}", ans, time.elapsed());
}

fn improved(messgae: &str, sz: usize) -> Option<usize> {
    let mut iter = messgae.chars();

    let mut window: VecDeque<_> = iter.by_ref().take(sz).collect();
    if window.iter().all_unique() {
        return Some(sz);
    }

    let mut i = sz;
    while let Some(c) = iter.next() {
        // find if the window contains the character
        window.pop_front();
        if let Some(p) = window.iter().position(|&x| x == c) {
            // remove all characters before the character
            // and add the new characters
            window.drain(..p);
            window.push_back(c);
            window.extend(iter.by_ref().take(p));
            i += p + 1;
        } else if !window.iter().all_unique() {
            window.push_back(c);
            i += 1;
        } else {
            return Some(i + 1);
        }
    }

    None
}
