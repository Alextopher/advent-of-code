//! `iterstuff` implements some useful iterator adapters.
use std::cmp::Ordering;

use binary_heap_plus::BinaryHeap;

impl<T: ?Sized> IterJunk for T where T: Iterator {}

pub trait IterJunk: Iterator {
    /// `k_smallest_by` is a function that returns the k smallest elements of an iterator **unsorted**
    /// according to a comparator function.
    ///
    /// If there are less than k elements in the iterator, it returns all of them.
    ///
    /// Only requires O(k) space, and O(n log k) time.
    ///
    /// # Examples
    ///
    /// ```
    /// use aoc::iterstuff::IterJunk;
    /// let v: Vec<i32> = vec![-5, 1, -3, 2, 4];
    ///
    /// let smallest: Vec<i32> = v.into_iter().k_smallest_by(3, |a, b| a.abs().cmp(&b.abs())).collect();
    ///
    /// // smallest might not be sorted, but it will be the 3 smallest elements
    /// assert!(smallest.contains(&1));
    /// assert!(smallest.contains(&2));
    /// assert!(smallest.contains(&-3));
    /// ```
    fn k_smallest_by<F>(mut self, k: usize, cmp: F) -> std::vec::IntoIter<<Self as Iterator>::Item>
    where
        Self: Sized,
        F: Fn(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        // Keep a max heap of the smallest k elements
        let mut heap = BinaryHeap::with_capacity_by(k, &cmp);
        heap.extend(self.by_ref().take(k));

        for x in self {
            let mut top = heap.peek_mut().unwrap();
            if cmp(&x, &top) == Ordering::Less {
                *top = x;
            }
        }

        heap.into_vec().into_iter()
    }

    /// `k_largest_by` is a function that returns the k largest elements of an iterator **unsorted**
    /// according to a comparator function. If there are less than k elements in the iterator, it
    /// returns all of them.
    ///
    /// Only requires O(k) space, and O(n log k) time.
    ///
    /// # Examples
    /// ```
    /// use aoc::iterstuff::IterJunk;
    /// let v: Vec<i32> = vec![-5, 1, 3, 2, -4];
    /// let largest: Vec<i32> = v.into_iter().k_largest_by(3, |a, b| a.abs().cmp(&b.abs())).collect();
    ///
    /// // largest might not be sorted, but it will be the 3 largest elements
    /// assert!(largest.contains(&-5));
    /// assert!(largest.contains(&-4));
    /// assert!(largest.contains(&3));
    /// ```
    fn k_largest_by<F>(self, k: usize, cmp: F) -> std::vec::IntoIter<<Self as Iterator>::Item>
    where
        Self: Sized,
        F: Fn(&Self::Item, &Self::Item) -> std::cmp::Ordering,
    {
        self.k_smallest_by(k, |a, b| cmp(b, a))
    }

    /// `k_smallest` is a function that returns the k smallest elements of an iterator **unsorted**.
    /// If there are less than k elements in the iterator, it returns all of them.
    ///
    /// Only requires O(k) space, and O(n log k) time.
    ///
    /// # Examples
    /// ```
    /// use aoc::iterstuff::IterJunk;
    /// let v = vec![-5, 1, -3, 2, 4];
    /// let smallest: Vec<i32> = v.into_iter().k_smallest(3).collect();
    ///
    /// // smallest might not be sorted, but it will be the 3 smallest elements
    /// assert!(smallest.contains(&-5));
    /// assert!(smallest.contains(&-3));
    /// assert!(smallest.contains(&1));
    /// ```
    fn k_smallest(self, k: usize) -> std::vec::IntoIter<<Self as Iterator>::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.k_smallest_by(k, |a, b| a.cmp(b))
    }

    /// `k_largest` is a function that returns the k largest elements of an iterator **unsorted**.
    /// If there are less than k elements in the iterator, it returns all of them.
    ///
    /// Only requires O(k) space, and O(n log k) time.
    ///
    /// # Examples
    /// ```
    /// use aoc::iterstuff::IterJunk;
    /// let v = vec![-5, 1, 3, 2, -4];
    /// let largest: Vec<i32> = v.into_iter().k_largest(3).collect();
    ///
    /// // largest might not be sorted, but it will be the 3 largest elements
    /// assert!(largest.contains(&3));
    /// assert!(largest.contains(&2));
    /// assert!(largest.contains(&1));
    /// ```
    fn k_largest(self, k: usize) -> std::vec::IntoIter<<Self as Iterator>::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.k_largest_by(k, |a, b| a.cmp(b))
    }

    /// `k_smallest_by_key` is a function that returns the k smallest elements of an iterator **unsorted**
    /// according to a key function. If there are less than k elements in the iterator, it returns all of them.
    ///
    /// Only requires O(k) space, and O(n log k) time.
    ///
    /// # Examples
    /// ```
    /// use aoc::iterstuff::IterJunk;
    /// let v: Vec<i32> = vec![-5, 1, -3, 2, 4];
    /// let smallest: Vec<i32> = v.into_iter().k_smallest_by_key(3, |x| x.abs()).collect();
    ///
    /// // smallest might not be sorted, but it will be the 3 smallest elements
    /// assert!(smallest.contains(&1));
    /// assert!(smallest.contains(&2));
    /// assert!(smallest.contains(&-3));
    /// ```
    fn k_smallest_by_key<F, K>(self, k: usize, f: F) -> std::vec::IntoIter<<Self as Iterator>::Item>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> K,
        K: Ord,
    {
        self.k_smallest_by(k, |a, b| f(a).cmp(&f(b)))
    }

    /// `k_largest_by_key` is a function that returns the k largest elements of an iterator **unsorted**
    /// according to a key function. If there are less than k elements in the iterator, it returns all of them.
    ///
    /// Only requires O(k) space, and O(n log k) time.
    ///
    /// # Examples
    /// ```
    /// use aoc::iterstuff::IterJunk;
    /// let v: Vec<i32> = vec![-5, 1, 3, 2, -4];
    /// let largest: Vec<i32> = v.into_iter().k_largest_by_key(3, |x| x.abs()).collect();
    ///
    /// // largest might not be sorted, but it will be the 3 largest elements
    /// assert!(largest.contains(&-5));
    /// assert!(largest.contains(&-4));
    /// assert!(largest.contains(&3));
    /// ```
    fn k_largest_by_key<F, K>(self, k: usize, f: F) -> std::vec::IntoIter<<Self as Iterator>::Item>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> K,
        K: Ord,
    {
        self.k_largest_by(k, |a, b| f(a).cmp(&f(b)))
    }
}
