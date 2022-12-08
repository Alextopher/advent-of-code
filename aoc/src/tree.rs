///! This is a tree data structure with backpointers. It's a helpful construction from time to time.
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Tree<T> {
    root: Node<T>,
}

#[derive(Debug)]
pub struct Node<T>(Rc<NodeInner<T>>);

#[derive(Debug)]
struct NodeInner<T> {
    inner: T,
    parent: RefCell<Weak<NodeInner<T>>>,
    children: RefCell<Vec<Rc<NodeInner<T>>>>,
}

impl<T> Tree<T> {
    pub fn new(root: T) -> Self {
        Self {
            root: Node::new(root),
        }
    }

    pub fn root(&self) -> Node<T> {
        self.root.clone()
    }

    /// Iterate over all nodes in the tree, in preorder.
    ///
    /// This uses O(depth(tree)) additional memory.
    ///
    /// ```
    /// use aoc::tree::Tree;
    ///
    /// //     1
    /// //    / \
    /// //   2   5
    /// //  / \   \
    /// // 3   4   6
    ///
    /// let mut tree : Tree<i32> = Tree::new(1);
    /// let mut root = tree.root();
    /// let mut two = root.add_child(2);
    /// let three = two.add_child(3);
    /// let four = two.add_child(4);
    /// let mut five = root.add_child(5);
    /// let six = five.add_child(6);
    ///
    /// let mut iter = tree.iter_preorder();
    ///
    /// assert_eq!(iter.next(), Some(root));
    /// assert_eq!(iter.next(), Some(two));
    /// assert_eq!(iter.next(), Some(three));
    /// assert_eq!(iter.next(), Some(four));
    /// assert_eq!(iter.next(), Some(five));
    /// assert_eq!(iter.next(), Some(six));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_preorder(&self) -> impl Iterator<Item = Node<T>> {
        self.iter_preorder_depth().map(|(node, _)| node)
    }

    /// Iterate over all nodes in the tree, in preorder tracking the depth of each node.
    ///
    /// See [`Tree::iter_preorder`] for more information.
    ///
    /// ```
    /// use aoc::tree::Tree;
    ///
    /// //     1
    /// //    / \
    /// //   2   5
    /// //  / \   \
    /// // 3   4   6
    ///
    /// let mut tree : Tree<i32> = Tree::new(1);
    /// let mut root = tree.root();
    /// let mut two = root.add_child(2);
    /// let three = two.add_child(3);
    /// let four = two.add_child(4);
    /// let mut five = root.add_child(5);
    /// let six = five.add_child(6);
    ///
    /// let mut iter = tree.iter_preorder_depth();
    /// assert_eq!(iter.next(), Some((root, 0)));
    /// assert_eq!(iter.next(), Some((two, 1)));
    /// assert_eq!(iter.next(), Some((three, 2)));
    /// assert_eq!(iter.next(), Some((four, 2)));
    /// assert_eq!(iter.next(), Some((five, 1)));
    /// assert_eq!(iter.next(), Some((six, 2)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_preorder_depth(&self) -> impl Iterator<Item = (Node<T>, usize)> {
        let mut stack = vec![(self.root(), 0)];
        std::iter::from_fn(move || {
            let (node, depth) = stack.pop()?;
            stack.extend(
                node.children()
                    .iter()
                    .cloned()
                    .rev()
                    .map(|child| (child, depth + 1)),
            );
            Some((node, depth))
        })
    }

    /// Iterate over all nodes in the tree, in postorder.
    ///
    /// Unlike iter_preorder this iterator is not lazily created. We use O(tree)
    /// additional memory to create the iterator.
    ///
    /// ```
    /// use aoc::tree::Tree;
    ///
    /// //     6
    /// //    / \
    /// //   3   5
    /// //  / \   \
    /// // 1   2   4
    ///
    /// let mut tree : Tree<i32> = Tree::new(6);
    /// let mut root = tree.root();
    /// let mut three = root.add_child(3);
    /// let one = three.add_child(1);
    /// let two = three.add_child(2);
    /// let mut five = root.add_child(5);
    /// let four = five.add_child(4);
    ///
    /// let mut iter = tree.iter_postorder();
    /// assert_eq!(iter.next(), Some(one));
    /// assert_eq!(iter.next(), Some(two));
    /// assert_eq!(iter.next(), Some(three));
    /// assert_eq!(iter.next(), Some(four));
    /// assert_eq!(iter.next(), Some(five));
    /// assert_eq!(iter.next(), Some(root));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_postorder(&self) -> impl Iterator<Item = Node<T>> {
        self.iter_postorder_depth().map(|(node, _)| node)
    }

    /// Iterate over all nodes in the tree, in postorder tracking the depth of each node.
    ///
    /// See [`Tree::iter_postorder`] for more information.
    ///
    /// ```
    /// use aoc::tree::Tree;
    ///
    /// //     6
    /// //    / \
    /// //   3   5
    /// //  / \   \
    /// // 1   2   4
    ///
    /// let mut tree : Tree<i32> = Tree::new(6);
    /// let mut root = tree.root();
    /// let mut three = root.add_child(3);
    /// let one = three.add_child(1);
    /// let two = three.add_child(2);
    /// let mut five = root.add_child(5);
    /// let four = five.add_child(4);
    ///
    /// let mut iter = tree.iter_postorder_depth();
    /// assert_eq!(iter.next(), Some((one, 2)));
    /// assert_eq!(iter.next(), Some((two, 2)));
    /// assert_eq!(iter.next(), Some((three, 1)));
    /// assert_eq!(iter.next(), Some((four, 2)));
    /// assert_eq!(iter.next(), Some((five, 1)));
    /// assert_eq!(iter.next(), Some((root, 0)));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_postorder_depth(&self) -> impl Iterator<Item = (Node<T>, usize)> {
        let mut stack = vec![(self.root(), 0)];
        let mut output = vec![];

        while let Some((node, depth)) = stack.pop() {
            output.push((node.clone(), depth));
            stack.extend(
                node.children()
                    .iter()
                    .cloned()
                    .map(|child| (child, depth + 1)),
            );
        }

        output.into_iter().rev()
    }

    // TODO: iter_inorder, iter_inorder_depth
}

impl<T> Node<T> {
    /// Create a new node with the given value and no parent or children.
    pub fn new(inner: T) -> Self {
        Self(Rc::new(NodeInner {
            inner,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        }))
    }

    /// `parent` returns the parent of this node, if it exists.
    pub fn parent(&self) -> Option<Node<T>> {
        self.0.parent.borrow().upgrade().map(Node)
    }

    /// `children` returns the children of this node.
    pub fn children(&self) -> Vec<Node<T>> {
        self.0
            .children
            .borrow()
            .iter()
            .map(|c| Node(c.clone()))
            .collect()
    }

    /// `add_child` adds a child to this node and returns a pointer to it.
    pub fn add_child(&mut self, child: T) -> Node<T> {
        let child = Rc::new(NodeInner {
            inner: child,
            parent: RefCell::new(Rc::downgrade(&self.0)),
            children: RefCell::new(Vec::new()),
        });

        self.0.children.borrow_mut().push(child.clone());
        Node(child)
    }

    /// Removes a child from the node and returns it.
    ///
    /// If the marked node is not a child of the node, returns `None`.
    ///
    /// Runs in O(n) time where n is the number of children.
    pub fn remove_child(&mut self, child: &Node<T>) -> Option<Node<T>> {
        let mut children = self.0.children.borrow_mut();
        let index = children.iter().position(|c| Rc::ptr_eq(&c, &child.0))?;

        Some(Node(children.remove(index)))
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Node<T> {}

impl<T> Clone for Node<T> {
    fn clone(&self) -> Self {
        Node(self.0.clone())
    }
}

impl<T> std::ops::Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.inner
    }
}
