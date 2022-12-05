use std::cmp::Ordering;

/// GetMultMut is a trait that provides methods for getting multiple mutable references to
/// elements at specific indices in a slice.
pub trait GetMultMut<'a, T> {
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

impl<'a, T> GetMultMut<'a, T> for [T] {
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
            let ans = self.get_mut_3(i, k, j);
            (ans.0, ans.2, ans.1)
        } else if j < i && i < k {
            let ans = self.get_mut_3(j, i, k);
            (ans.1, ans.0, ans.2)
        } else if j < k && k < i {
            let ans = self.get_mut_3(j, k, i);
            (ans.1, ans.2, ans.0)
        } else if k < i && i < j {
            let ans = self.get_mut_3(k, i, j);
            (ans.2, ans.0, ans.1)
        } else if k < j && j < i {
            let ans = self.get_mut_3(k, j, i);
            (ans.2, ans.1, ans.0)
        } else {
            panic!("i, j, k are not unique");
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
        let mut v = vec![1, 2, 3, 4, 5];
        let (a, b, c) = v.get_mut_3(1, 3, 4);
        *a = 10;
        *b = 20;
        *c = 30;
        assert_eq!(v, vec![1, 10, 3, 20, 30]);

        // test another order
        let mut v = vec![0, 1, 2, 3, 4, 5];
        let (a, b, c) = v.get_mut_3(3, 1, 4);
        *a = 30;
        *b = 10;
        *c = 40;
        assert_eq!(v, vec![0, 10, 2, 30, 40, 5]);
    }
}
