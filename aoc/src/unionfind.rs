#[derive(Debug, Clone)]
pub struct UnionFind {
    sizes: Vec<usize>,
    ids: Vec<usize>,
}

impl UnionFind {
    pub fn new(size: usize) -> Self {
        UnionFind {
            sizes: vec![1; size],
            ids: (0..size).collect(),
        }
    }

    /// number of elements in the set
    pub fn len(&self) -> usize {
        self.sizes.len()
    }

    /// find what set 'a' belongs to
    pub fn find(&mut self, a: usize) -> usize {
        let root = self.find_no_compression(a);

        // path compression
        let mut parent = a;
        while self.ids[parent] != parent {
            let tmp = self.ids[parent];
            self.ids[parent] = root;
            parent = tmp
        }

        root
    }

    /// find what set 'a' belongs to without doing path compression
    pub fn find_no_compression(&self, a: usize) -> usize {
        let mut root = a;
        while self.ids[root] != root {
            root = self.ids[root]
        }
        root
    }

    /// merge `a`'s set with `b`'s set
    pub fn union(&mut self, a: usize, b: usize) {
        let root1 = self.find(a);
        let root2 = self.find(b);

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
    }

    /// return true if `a`'s set equals `b`'s set
    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    /// returns the number of elements in `a`
    pub fn size(&mut self, a: usize) -> usize {
        let root = self.find(a);
        return self.sizes[root];
    }
}
