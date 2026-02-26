#[derive(Debug, Clone)]
pub struct UnionFind {
    sizes: Vec<usize>,
    ids: Vec<usize>,

    // tracks the number of groups
    num_groups: usize,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            sizes: vec![1; size],
            ids: (0..size).collect(),
            num_groups: size,
        }
    }

    pub fn inner(&self) -> (&[usize], &[usize]) {
        (&self.sizes, &self.ids)
    }

    pub fn into_inner(self) -> (Vec<usize>, Vec<usize>) {
        (self.sizes, self.ids)
    }

    /// `len` returns the number of elements in the union find.
    pub fn len(&self) -> usize {
        self.sizes.len()
    }

    /// `is_empty` returns true if the union find contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// `find` returns the root of the element at index `i`. It also performs
    /// path compression.
    pub fn find(&mut self, i: usize) -> usize {
        let root = self.find_no_compression(i);

        // path compression
        let mut parent = i;
        while self.ids[parent] != parent {
            let tmp = self.ids[parent];
            self.ids[parent] = root;
            parent = tmp
        }

        root
    }

    /// `find_no_compression` returns the root of the element at index `i`
    pub fn find_no_compression(&self, i: usize) -> usize {
        let mut root = i;
        while self.ids[root] != root {
            root = self.ids[root]
        }
        root
    }

    /// `union` merges the sets containing `i` and `j`.
    pub fn union(&mut self, i: usize, j: usize) {
        let root1 = self.find(i);
        let root2 = self.find(j);

        if root1 == root2 {
            return;
        }

        // The add the smaller tree to the larger one
        if self.sizes[root1] > self.sizes[root2] {
            self.ids[root2] = self.ids[root1];
            self.sizes[root1] += self.sizes[root2];
        } else {
            self.ids[root1] = self.ids[root2];
            self.sizes[root2] += self.sizes[root1];
        }

        self.num_groups -= 1;
    }

    /// `connected` returns true if the elements at index `i` and `j` are in
    /// the same set.
    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// `size` returns the size of the set containing the element at index `i`.
    pub fn size(&mut self, a: usize) -> usize {
        let root = self.find(a);
        self.sizes[root]
    }

    pub fn num_groups(&self) -> usize {
        self.num_groups
    }
}
