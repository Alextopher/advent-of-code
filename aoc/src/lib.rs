pub mod get_mut;
pub mod iterstuff;
pub mod unionfind;

pub use get_mut::GetMultMut;
pub use iterstuff::IterJunk;
pub use unionfind::UnionFind;

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
