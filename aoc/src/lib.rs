pub mod algs;
pub mod get_mut;
pub mod iterstuff;
pub mod stringstuff;
pub mod tree;
pub mod unionfind;

pub use aoc_macro::get_input;
pub use get_mut::GetMany;
pub use iterstuff::IterJunk;
pub use tree::{Node, Tree};
pub use unionfind::UnionFind;

use std::{fs, path::Path};

/// Read the entire file into memory as a string.
/// Then split it on newlines.
pub fn get_lines(f: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = fs::read_to_string(f).unwrap();
    file.lines()
        .map(|l| l.to_owned())
        .collect::<Vec<_>>()
        .into_iter()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input() {
        let input: &str = get_input!(2020, 1);
        assert!(input.len() > 0);
    }
}
