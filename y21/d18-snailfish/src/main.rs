use std::cell::RefCell;
use std::rc::{Rc, Weak};

use aoc::input_str;
use itertools::Itertools;

#[derive(Debug)]
enum SnailfishValue {
    Pair {
        left: Rc<RefCell<SnailfishNumber>>,
        right: Rc<RefCell<SnailfishNumber>>,
    },
    Value(u32),
}

impl SnailfishValue {
    fn is_pair(&self) -> bool {
        matches!(self, SnailfishValue::Pair { .. })
    }

    fn is_value(&self) -> bool {
        matches!(self, SnailfishValue::Value(_))
    }

    fn as_pair(&self) -> Option<(&Rc<RefCell<SnailfishNumber>>, &Rc<RefCell<SnailfishNumber>>)> {
        match self {
            SnailfishValue::Pair { left, right } => Some((left, right)),
            _ => None,
        }
    }

    fn as_value(&self) -> Option<u32> {
        match self {
            SnailfishValue::Value(val) => Some(*val),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct SnailfishNumber {
    value: SnailfishValue,
    parent: Weak<RefCell<SnailfishNumber>>,
    is_left_child: Option<bool>,
}

impl SnailfishNumber {
    fn new_value(val: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(SnailfishNumber {
            value: SnailfishValue::Value(val),
            parent: Weak::new(),
            is_left_child: None,
        }))
    }

    fn new_pair(left: Rc<RefCell<Self>>, right: Rc<RefCell<Self>>) -> Rc<RefCell<Self>> {
        let pair = Rc::new(RefCell::new(SnailfishNumber {
            value: SnailfishValue::Pair {
                left: left.clone(),
                right: right.clone(),
            },
            parent: Weak::new(),
            is_left_child: None,
        }));

        // Set parent pointers and left/right child flags
        left.borrow_mut().parent = Rc::downgrade(&pair);
        left.borrow_mut().is_left_child = Some(true);
        right.borrow_mut().parent = Rc::downgrade(&pair);
        right.borrow_mut().is_left_child = Some(false);

        pair
    }

    fn magnitude(&self) -> u32 {
        match &self.value {
            SnailfishValue::Pair { left, right } => {
                3 * left.borrow().magnitude() + 2 * right.borrow().magnitude()
            }
            SnailfishValue::Value(value) => *value,
        }
    }

    // Find the nearest left neighbor (a Value node to the left of this one)
    fn find_left_neighbor(&self) -> Option<Rc<RefCell<SnailfishNumber>>> {
        // First, go up until we find a node where we came from the right
        let mut current = self.parent.upgrade()?;
        let mut came_from_left = self.is_left_child?;

        while came_from_left {
            let parent = current.borrow().parent.upgrade()?;
            came_from_left = current.borrow().is_left_child?;
            current = parent;
        }

        // Now go down to the left subtree and find the rightmost value
        if let SnailfishValue::Pair { left, .. } = &current.borrow().value {
            let mut node = left.clone();

            // Keep going right until we find a value
            loop {
                let is_value = matches!(&node.borrow().value, SnailfishValue::Value(_));
                if is_value {
                    return Some(node);
                }

                let next_node = {
                    let borrowed = node.borrow();
                    match &borrowed.value {
                        SnailfishValue::Pair { right, .. } => right.clone(),
                        _ => unreachable!(),
                    }
                };
                node = next_node;
            }
        }

        None
    }

    // Find the nearest right neighbor (a Value node to the right of this one)
    fn find_right_neighbor(&self) -> Option<Rc<RefCell<SnailfishNumber>>> {
        // First, go up until we find a node where we came from the left
        let mut current = self.parent.upgrade()?;
        let mut came_from_right = !self.is_left_child?;

        while came_from_right {
            let parent = current.borrow().parent.upgrade()?;
            came_from_right = !current.borrow().is_left_child?;
            current = parent;
        }

        // Now go down to the right subtree and find the leftmost value
        if let SnailfishValue::Pair { right, .. } = &current.borrow().value {
            let mut node = right.clone();

            // Keep going left until we find a value
            loop {
                let is_value = matches!(&node.borrow().value, SnailfishValue::Value(_));
                if is_value {
                    return Some(node);
                }

                let next_node = {
                    let borrowed = node.borrow();
                    match &borrowed.value {
                        SnailfishValue::Pair { left, .. } => left.clone(),
                        _ => unreachable!(),
                    }
                };
                node = next_node;
            }
        }

        None
    }
}

fn explode(node: Rc<RefCell<SnailfishNumber>>, depth: usize) -> bool {
    if depth == 4 && node.borrow().value.is_pair() {
        assert!(
            node.borrow()
                .value
                .as_pair()
                .unwrap()
                .0
                .borrow()
                .value
                .is_value()
        );
        assert!(
            node.borrow()
                .value
                .as_pair()
                .unwrap()
                .1
                .borrow()
                .value
                .is_value()
        );

        let (left, right) = match &node.borrow().value {
            SnailfishValue::Pair { left, right } => {
                (left.borrow().magnitude(), right.borrow().magnitude())
            }
            _ => unreachable!(),
        };

        if let Some(left_neighbor) = node.borrow().find_left_neighbor() {
            // assert it's a value
            assert!(left_neighbor.borrow().value.is_value());
            let value = left_neighbor.borrow().value.as_value().unwrap() + left;
            left_neighbor.borrow_mut().value = SnailfishValue::Value(value);
        }

        if let Some(right_neighbor) = node.borrow().find_right_neighbor() {
            // assert it's a value
            assert!(right_neighbor.borrow().value.is_value());
            let value = right_neighbor.borrow().value.as_value().unwrap() + right;
            right_neighbor.borrow_mut().value = SnailfishValue::Value(value);
        }

        // this node needs to be removed
        node.borrow_mut().value = SnailfishValue::Value(0);

        return true;
    }

    match &node.borrow().value {
        SnailfishValue::Pair { left, right } => {
            explode(left.clone(), depth + 1) || explode(right.clone(), depth + 1)
        }
        SnailfishValue::Value(_) => false,
    }
}

fn split(node: Rc<RefCell<SnailfishNumber>>) -> bool {
    if node.borrow().value.is_value() {
        let n = node.borrow().value.as_value().unwrap();
        if n >= 10 {
            let left_val = SnailfishNumber::new_value(n / 2);
            let right_val = SnailfishNumber::new_value(n.div_ceil(2));

            node.borrow_mut().value = SnailfishValue::Pair {
                left: left_val.clone(),
                right: right_val.clone(),
            };

            // Set up parent pointers for the new children
            left_val.borrow_mut().parent = Rc::downgrade(&node);
            left_val.borrow_mut().is_left_child = Some(true);
            right_val.borrow_mut().parent = Rc::downgrade(&node);
            right_val.borrow_mut().is_left_child = Some(false);

            true
        } else {
            false
        }
    } else {
        let n = node.borrow();
        let (left, right) = n.value.as_pair().unwrap();
        split(left.clone()) || split(right.clone())
    }
}

impl std::fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            SnailfishValue::Pair { left, right } => {
                write!(f, "[{},{}]", left.borrow(), right.borrow())
            }
            SnailfishValue::Value(val) => write!(f, "{}", val),
        }
    }
}

fn add(
    left: Rc<RefCell<SnailfishNumber>>,
    right: Rc<RefCell<SnailfishNumber>>,
) -> Rc<RefCell<SnailfishNumber>> {
    let result = SnailfishNumber::new_pair(left, right);
    // println!("start {}", result.borrow());
    loop {
        if explode(result.clone(), 0) {
            // println!("after explode {}", result.borrow());
            continue;
        }

        if split(result.clone()) {
            // println!("after split {}", result.borrow());
            continue;
        }

        break;
    }
    result
}

// parses a snailfish number
fn parse(line: &str) -> Rc<RefCell<SnailfishNumber>> {
    let chars: Vec<char> = line.chars().collect();
    let mut stack: Vec<Rc<RefCell<SnailfishNumber>>> = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '[' => {
                i += 1;
            }
            ']' => {
                let right = stack.pop().expect("Expected right element");
                let left = stack.pop().expect("Expected left element");
                let pair = SnailfishNumber::new_pair(left, right);
                stack.push(pair);
                i += 1;
            }
            ',' => {
                i += 1;
            }
            '0'..='9' => {
                let mut val = 0u32;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    val = val * 10 + chars[i].to_digit(10).unwrap();
                    i += 1;
                }
                stack.push(SnailfishNumber::new_value(val));
            }
            _ => {
                panic!("Unexpected character: {}", chars[i]);
            }
        }
    }

    stack.pop().expect("Expected exactly one element remaining")
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(parse)
        .reduce(add)
        .unwrap()
        .borrow()
        .magnitude()
}

fn part2(input: &str) -> u32 {
    let forward = input
        .lines()
        .combinations(2)
        .map(|pair| {
            let left = parse(pair[0]);
            let right = parse(pair[1]);
            let sum = add(left, right);
            sum.borrow().magnitude()
        })
        .max()
        .unwrap();

    let reverse = input
        .lines()
        .combinations(2)
        .map(|pair| {
            let left = parse(pair[0]);
            let right = parse(pair[1]);
            let sum = add(right, left);
            sum.borrow().magnitude()
        })
        .max()
        .unwrap();

    forward.max(reverse)
}

fn main() {
    let input = input_str!(2021, 18);

    let start = std::time::Instant::now();
    println!("Part 1: {}", part1(input));
    println!("Time: {:?}", start.elapsed());

    let start = std::time::Instant::now();
    println!("Part 2: {}", part2(input));
    println!("Time: {:?}", start.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explode() {
        let tests = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
        ];

        for (input, expected) in tests {
            println!("Testing {}", input);
            let input = parse(input);
            assert!(explode(input.clone(), 0));
            assert_eq!(input.borrow().to_string(), expected)
        }
    }

    #[test]
    fn test_split() {
        let tests = [
            (
                "[[[[0,7],4],[15,[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
                "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            ),
        ];

        for (input, expected) in tests {
            println!("Testing {}", input);
            let input = parse(input);
            assert!(split(input.clone()));
            assert_eq!(input.borrow().to_string(), expected)
        }
    }

    #[test]
    fn test_add() {
        let left = parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]");
        let right = parse("[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]");
        let result = add(left, right);
        assert_eq!(
            result.borrow().to_string(),
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
        );
    }

    #[test]
    fn test_add_2() {
        let left = parse("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let right = parse("[1,1]");
        let result = add(left, right);
        assert_eq!(
            result.borrow().to_string(),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
    }

    #[test]
    fn test_partial_sums() {
        let items = [
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
            "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
            "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
            "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
            "[7,[5,[[3,8],[1,4]]]]",
            "[[2,[2,2]],[8,[8,1]]]",
            "[2,9]",
            "[1,[[[9,3],9],[[9,0],[0,7]]]]",
            "[[[5,[7,4]],7],1]",
            "[[[[4,2],2],6],[8,7]]",
        ];

        let sums = [
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]",
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]",
            "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]",
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]",
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]",
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]",
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ];

        let mut acc = items[0];
        for (i, item) in items.iter().skip(1).enumerate() {
            // print
            println!("\t{}\n+\t{}", acc, item);

            // parse acc and item
            let (left, right) = (parse(acc), parse(item));
            let sum = add(left, right);
            println!("=\t{}", sum.borrow());
            println!();
            assert_eq!(sum.borrow().to_string(), sums[i]);
            acc = sums[i];
        }
    }

    #[test]
    fn test_example() {
        let example = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\n[[[5,[2,8]],4],[5,[[9,9],0]]]\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\n[[[[5,4],[7,7]],8],[[8,3],8]]\n[[9,3],[[9,9],[6,[4,9]]]]\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
        assert_eq!(part1(example), 4140);
    }
}
