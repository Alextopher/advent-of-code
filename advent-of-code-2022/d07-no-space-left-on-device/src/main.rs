use std::{cell::RefCell, time::Instant};

use aoc::{Node, Tree};
use itertools::Itertools;

fn solution(filename: &str) -> (usize, usize) {
    let mut time = Instant::now();

    // Don't count the time it takes to read the input into memory
    let mut lines = aoc::get_lines(filename).peekable();
    println!("Read input {:?}", time.elapsed());
    time = Instant::now();

    let file_system: Tree<RefCell<(String, Option<usize>)>> =
        Tree::new(RefCell::new((String::from("/"), None)));
    let mut current_node = file_system.root();

    while let Some(line) = lines.next() {
        let split = line.split_whitespace().collect_vec();

        // Either "$ cd path" or "$ ls"
        if split[1] == "cd" {
            let path = split[2];

            if path == ".." {
                current_node = current_node.parent().unwrap();
            } else if path == "/" {
                current_node = file_system.root();
            } else {
                current_node = current_node
                    .children()
                    .into_iter()
                    .find(|n| (*n).borrow().0 == path)
                    .unwrap();
            }
        } else if split[1] == "ls" {
            // next few lines are "dir name" or "size name"
            while let Some(line) = lines.peek() {
                let split = line.split_whitespace().collect_vec();
                // If the next line is a command we're done processing this ls
                if split[0] == "$" {
                    break;
                }

                let name = split[1];
                if split[0] == "dir" {
                    current_node.add_child(RefCell::new((name.to_string(), None)));
                } else {
                    let size = split[0].parse::<usize>().unwrap();
                    let mut inner = current_node.borrow_mut();

                    match inner.1 {
                        Some(ref mut s) => *s += size,
                        None => inner.1 = Some(size),
                    }
                }

                lines.next();
            }
        } else {
            panic!("Unknown command: {}", split[1]);
        }
    }
    println!("parse: {:?}", time.elapsed());
    time = Instant::now();

    update_sizes(&mut file_system.root());
    println!("update_sizes: {:?}", time.elapsed());
    time = Instant::now();

    // Part 1 - sum the sizes of directories < 100000
    let sum = file_system
        .iter_preorder()
        .filter_map(|n| {
            let size = n.borrow().1.unwrap();
            (size < 100000).then_some(size)
        })
        .sum::<usize>();

    println!("{} {:?}", sum, time.elapsed());

    let max_size = 70000000;
    let remaining = max_size - file_system.root().borrow().1.unwrap();

    // find the smallest file such that total size - file_system > 30000000
    let smallest_file_size = file_system
        .iter_preorder()
        .map(|n| n.borrow().1.unwrap())
        .filter(|size| remaining + size > 30000000)
        .min()
        .unwrap();

    println!("{} {:?}", smallest_file_size, time.elapsed());

    (sum, smallest_file_size)
}

/// Update the sizes of all the directories to include the sizes of their children
/// + their own size
fn update_sizes(node: &mut Node<RefCell<(String, Option<usize>)>>) {
    let mut size = 0;

    for mut child in node.children() {
        update_sizes(&mut child);
        size += child.borrow().1.unwrap();
    }

    let mut inner = node.borrow_mut();
    match inner.1 {
        Some(ref mut s) => *s += size,
        None => inner.1 = Some(size),
    }
}

fn main() {
    solution("input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solution("example.txt"), (95437, 24933642));
    }

    #[test]
    fn test_input() {
        assert_eq!(solution("input.txt"), (1391690, 5469168));
    }
}
