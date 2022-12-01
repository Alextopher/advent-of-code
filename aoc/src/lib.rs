pub mod iterstuff;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn get_lines(f: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(Result::unwrap)
}
