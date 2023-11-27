use std::cell::RefCell;

use aoc::{get_lines, Node, Tree};

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a [u8]) -> Parser<'a> {
        Parser { input, pos: 0 }
    }

    fn parse_num(&mut self) -> Option<u8> {
        if self.input[self.pos].is_ascii_digit() {
            let num = self.input[self.pos] - b'0';
            self.pos += 1;
            Some(num)
        } else {
            None
        }
    }

    fn parse(&mut self) -> Node<RefCell<Option<u8>>> {
        let mut node = Node::new(RefCell::new(None));

        if let Some(num) = self.parse_num() {
            *node.borrow_mut() = Some(num);
            return node;
        } else if self.input[self.pos] == b'[' {
            self.pos += 1;
            // there are 2 children separated by a comma
            let left = self.parse();
            self.pos += 1;
            let right = self.parse();
            self.pos += 1;

            node.add_child_node(left);
            node.add_child_node(right);

            return node;
        } else {
            panic!("Unexpected character: {}", self.input[self.pos] as char);
        }
    }
}

fn main() {
    let mut numbers = get_lines("input.txt").map(|line| {
        let mut parser = Parser::new(line.as_bytes());
        parser.parse()
    });

    let n1 = numbers.next().unwrap();
    let n2 = numbers.next().unwrap();

    // Make a new node that is the concatenation of the two trees
    let mut new_node = Node::new(RefCell::new(None));
    new_node.add_child_node(n1);
    new_node.add_child_node(n2);

    let new_tree: Tree<_> = new_node.try_into().unwrap();

    new_tree.iter_postorder
}
