use std::cmp::Ordering;

/// GetMany is a trait that provides methods for getting multiple mutable references to
/// elements at specific indices in a slice.
pub trait GetMany<'a, T> {
    /// `get_mut_2` returns a tuple of two mutable references to the elements at indices `i` and `j`.
    ///
    /// # Panics
    ///
    /// Panics if `i` or `j` are out of bounds. Or if `i == j`.
    fn get_mut_2(&'a mut self, i: usize, j: usize) -> (&'a mut T, &'a mut T);

    /// `get_mut_3` returns a tuple of three mutable references to the elements at indices `i`, `j`, and `k`.
    ///
    /// # Panics
    ///
    /// Panics if `i`, `j`, or `k` are out of bounds. Or if `i == j` or `i == k` or `j == k`. (i.e. no duplicates)
    fn get_mut_3(&'a mut self, i: usize, j: usize, k: usize) -> (&'a mut T, &'a mut T, &'a mut T);
}

impl<'a, T> GetMany<'a, T> for [T] {
    fn get_mut_2(&'a mut self, i: usize, j: usize) -> (&'a mut T, &'a mut T) {
        match i.cmp(&j) {
            Ordering::Less => {
                let (left, right) = self.split_at_mut(j);
                (&mut left[i], &mut right[0])
            }
            Ordering::Equal => panic!("i == j"),
            Ordering::Greater => {
                let (left, right) = self.split_at_mut(i);
                (&mut right[0], &mut left[j])
            }
        }
    }

    fn get_mut_3(&'a mut self, i: usize, j: usize, k: usize) -> (&'a mut T, &'a mut T, &'a mut T) {
        if i < j && j < k {
            let (left, right) = self.split_at_mut(k);
            let (left, mid) = left.split_at_mut(j);

            (&mut left[i], &mut mid[0], &mut right[0])
        } else if i < k && k < j {
            let (i, k, j) = self.get_mut_3(i, k, j);
            (i, j, k)
        } else if j < i && i < k {
            let (j, i, k) = self.get_mut_3(j, i, k);
            (i, j, k)
        } else if j < k && k < i {
            let (j, k, i) = self.get_mut_3(j, k, i);
            (i, j, k)
        } else if k < i && i < j {
            let (k, i, j) = self.get_mut_3(k, i, j);
            (i, j, k)
        } else if k < j && j < i {
            let (k, j, i) = self.get_mut_3(k, j, i);
            (i, j, k)
        } else {
            panic!("i == j || i == k || j == k")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mut_2() {
        let mut v = vec![1, 2, 3, 4, 5];
        let (a, b) = v.get_mut_2(1, 3);
        *a = 10;
        *b = 20;
        assert_eq!(v, vec![1, 10, 3, 20, 5]);

        // test another order
        let (a, b) = v.get_mut_2(3, 1);
        *a = 30;
        *b = 40;
        assert_eq!(v, vec![1, 40, 3, 30, 5]);
    }

    #[test]
    fn test_get_mut_3() {
        let mut values = (0..10).collect::<Vec<_>>();

        // test i < j < k
        let (a, b, c) = values.get_mut_3(1, 3, 5);
        assert!(*a == 1 && *b == 3 && *c == 5);

        // test i < k < j
        let (a, b, c) = values.get_mut_3(1, 5, 3);
        assert!(*a == 1 && *b == 5 && *c == 3);

        // test j < i < k
        let (a, b, c) = values.get_mut_3(3, 1, 5);
        assert!(*a == 3 && *b == 1 && *c == 5);

        // test j < k < i
        let (a, b, c) = values.get_mut_3(3, 5, 1);
        assert!(*a == 3 && *b == 5 && *c == 1);

        // test k < i < j
        let (a, b, c) = values.get_mut_3(5, 1, 3);
        assert!(*a == 5 && *b == 1 && *c == 3);

        // test k < j < i
        let (a, b, c) = values.get_mut_3(5, 3, 1);
        assert!(*a == 5 && *b == 3 && *c == 1);
    }

    #[test]
    #[should_panic]
    fn test_get_mut_3_panic_i_eq_j() {
        let mut values = (0..10).collect::<Vec<_>>();
        values.get_mut_3(1, 1, 5);
    }

    // A bit of a fuzz
    #[test]
    fn test_get_mut_3_fuzz() {
        let mut values = (0..10).collect::<Vec<_>>();

        for i in 0..values.len() {
            for j in 0..values.len() {
                for k in 0..values.len() {
                    if i != j && i != k && j != k {
                        let (a, b, c) = values.get_mut_3(i, j, k);
                        assert!(*a == i && *b == j && *c == k);
                    }
                }
            }
        }
    }
}
