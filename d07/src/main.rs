use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use aoc::*;
use itertools::Itertools;

// A tree to store something that looks like a file system
#[derive(Debug)]
enum NodeInner {
    File(u32),
    Directory(RefCell<Vec<Rc<Node>>>),
}

impl NodeInner {
    fn is_directory(&self) -> bool {
        matches!(self, NodeInner::Directory(_))
    }

    fn as_directory(&self) -> Option<&RefCell<Vec<Rc<Node>>>> {
        match &self {
            NodeInner::File(_) => None,
            NodeInner::Directory(children) => Some(children),
        }
    }
}

#[derive(Debug)]
struct Node {
    inner: NodeInner,
    name: String,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    fn size(&self) -> u32 {
        match &self.inner {
            NodeInner::File(size) => *size,
            NodeInner::Directory(children) => children
                .borrow()
                .iter()
                .map(|child| child.size())
                .sum::<u32>(),
        }
    }

    /// Returns an iterator over _all_ directories in the tree
    fn all_directories(&self) -> Vec<Rc<Node>> {
        let mut stack: Vec<_> = self
            .inner
            .as_directory()
            .unwrap()
            .borrow()
            .iter()
            .filter(|child| child.inner.is_directory())
            .cloned()
            .collect();

        let mut directories = vec![];

        while let Some(node) = stack.pop() {
            directories.push(node.clone());

            if let Some(children) = node.inner.as_directory() {
                let c = children.borrow();
                stack.extend(
                    c.iter()
                        .filter(|dir_entry| dir_entry.inner.is_directory())
                        .cloned(),
                );
            }
        }

        directories
    }
}

fn main() {
    let file_system = Rc::new(Node {
        inner: NodeInner::Directory(RefCell::new(Vec::new())),
        name: String::from("/"),
        parent: RefCell::new(Weak::new()),
    });

    let mut current_dir = Rc::clone(&file_system);

    // each line is either "$ cd <path>" or "$ ls" or "<size> filename"
    // we need to read this into a tree
    for line in get_lines("input.txt") {
        // split input
        let parts = line.split_whitespace().collect_vec();

        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    // go up a directory
                    current_dir = current_dir
                        .clone()
                        .parent
                        .borrow()
                        .upgrade()
                        .expect("no parent directory");
                } else if parts[2] == "/" {
                    current_dir = Rc::clone(&file_system);
                } else {
                    // go down a directory
                    let new_dir = current_dir
                        .inner
                        .as_directory()
                        .expect("tried to go down from file")
                        .borrow()
                        .iter()
                        .find(|child| child.name == parts[2])
                        .expect("tried to go down to non-existent directory")
                        .clone();

                    current_dir = new_dir;
                }
            } else if parts[1] == "ls" {
                // do nothing
            }
        } else {
            // add file to current directory
            if parts[0] == "dir" {
                current_dir
                    .inner
                    .as_directory()
                    .unwrap()
                    .borrow_mut()
                    .push(Rc::new(Node {
                        inner: NodeInner::Directory(RefCell::new(Vec::new())),
                        name: parts[1].to_string(),
                        parent: RefCell::new(Rc::downgrade(&current_dir)),
                    }));
            } else {
                let size = parts[0].parse::<u32>().expect("invalid size");

                current_dir
                    .inner
                    .as_directory()
                    .unwrap()
                    .borrow_mut()
                    .push(Rc::new(Node {
                        inner: NodeInner::File(size),
                        name: parts[1].to_string(),
                        parent: RefCell::new(Rc::downgrade(&current_dir)),
                    }));
            }
        }
    }

    let sizes = file_system
        .all_directories()
        .into_iter()
        .map(|dir| dir.size())
        .filter(|size| *size < 100000)
        .sum::<u32>();

    println!("total size: {}", sizes);

    let max_size = 70000000;
    let remaining = max_size - file_system.size();

    // find the smallest file such that total size - file_system > 30000000
    let smallest_file_size = file_system
        .all_directories()
        .into_iter()
        .map(|dir| dir.size())
        .filter(|size| remaining + size > 30000000)
        .min();

    println!("smallest file: {}", smallest_file_size.unwrap());
}
