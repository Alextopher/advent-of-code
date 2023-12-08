use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    rc::Rc,
};

use itertools::Itertools;

#[derive(Debug)]
enum SnailFishNumberInner {
    Leaf(u8),
    Pair(Rc<SnailFishNumber>, Rc<SnailFishNumber>),
}

#[derive(Debug)]
struct SnailFishNumber {
    inner: SnailFishNumberInner,
    parent: RefCell<Option<Rc<SnailFishNumber>>>,
}

impl SnailFishNumber {
    fn is_leaf(&self) -> bool {
        match self.inner {
            SnailFishNumberInner::Leaf(_) => true,
            _ => false,
        }
    }

    fn is_pair(&self) -> bool {
        match self.inner {
            SnailFishNumberInner::Pair(_, _) => true,
            _ => false,
        }
    }

    fn reduce(&mut self) {}

    /// Applies a reduction to the SnailFishNumber
    /// returns true if a reduction was made
    fn reduce_once(&mut self) -> bool {
        let mut pair = self.dfs_traverse_track_depth().collect_vec();

        for i in 0..pair.len() {
            let (node, depth) = pair.get_mut(i).unwrap();

            if node.is_pair() && depth == 4 {}
        }

        false
    }

    fn dfs_traverse_track_depth(&self) -> impl Iterator<Item = (&SnailFishNumber, usize)> {
        // DFS order over binary tree
        let mut stack = vec![(self, 0)];

        std::iter::from_fn(move || {
            let (node, depth) = stack.pop()?;
            match &node.inner {
                SnailFishNumberInner::Pair(l, r) => {
                    stack.push((&r, depth + 1));
                    stack.push((&l, depth + 1));
                }
                _ => {}
            }
            Some((node, depth))
        })
    }
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Parser<'a> {
        Parser { input, pos: 0 }
    }

    fn parse(&mut self, parent: Option<Rc<SnailFishNumber>>) -> Rc<SnailFishNumber> {
        if let Some(num) = self.parse_num() {
            Rc::new(SnailFishNumber {
                inner: SnailFishNumberInner::Leaf(num),
                parent: RefCell::new(parent),
            })
        } else if self.input[self.pos] == b'[' {
            self.pos += 1;
            let left = self.parse(None);
            if self.input[self.pos] == b',' {
                self.pos += 1;
                let right = self.parse(None);
                if self.input[self.pos] == b']' {
                    self.pos += 1;
                    let result = Rc::new(SnailFishNumber {
                        inner: SnailFishNumberInner::Pair(left, right),
                        parent: RefCell::new(None),
                    });

                    match &result.inner {
                        SnailFishNumberInner::Pair(l, r) => {
                            l.parent.replace(Some(result.clone()));
                            r.parent.replace(Some(result.clone()));
                        }
                        _ => {}
                    }

                    result
                } else {
                    panic!("Expected ']', found {}", self.input[self.pos])
                }
            } else {
                panic!("Expected ',', found {}", self.input[self.pos])
            }
        } else {
            panic!("Expected '[', found {}", self.input[self.pos])
        }
    }

    // number is a single digit
    fn parse_num(&mut self) -> Option<u8> {
        if self.input[self.pos].is_ascii_digit() {
            let num = self.input[self.pos] - b'0';
            self.pos += 1;
            Some(num)
        } else {
            None
        }
    }
}

// Some examples
// [1,2]
// [[1,2],3]
// [9,[8,7]]
// [[1,9],[8,5]]
// [[[[1,2],[3,4]],[[5,6],[7,8]]],9]
// [[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]
// [[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]
impl std::fmt::Display for SnailFishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.inner {
            SnailFishNumberInner::Leaf(n) => write!(f, "{}", n),
            SnailFishNumberInner::Pair(l, r) => write!(f, "[{},{}]", l, r),
        }
    }
}

fn main() {
    let lines = get_lines("input.txt");

    lines
        .map(|line| {
            let mut parser = Parser::new(line.as_bytes());
            let num = parser.parse(None);
            println!("{}", num);
        })
        .count();
}

fn get_lines(f: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(Result::unwrap)
}
