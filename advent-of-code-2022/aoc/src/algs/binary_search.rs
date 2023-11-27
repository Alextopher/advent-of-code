//! Module contains generic binary searching methods

use num_traits::Num;
use std::cmp::Ordering;

/// Arbitrary binary search algorithm.
///
/// The index is generic over [`num_traits::Num`].
///
/// # Arguments
/// t - target value
/// f - function that returns value at "index" i
/// start - start "index"
/// end - end "index"
///
/// # Returns
/// Ok(i) - if target value is found at "index" i
/// Err(i) - if target value is not found, but it should be at "index" i
///
/// ```
/// use aoc::algs::binary_search;
///
/// let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// assert_eq!(binary_search(&5, |i: usize| v[i], 0, v.len() - 1), Ok(4));
/// assert_eq!(binary_search(&0, |i: usize| v[i], 0, v.len() - 1), Err(0));
/// assert_eq!(binary_search(&11, |i: usize| v[i], 0, v.len() - 1), Err(9));
/// ```
pub fn binary_search<N, T, F>(t: &T, f: F, mut start: N, mut end: N) -> Result<N, N>
where
    N: Num + PartialOrd + Copy,
    T: std::cmp::PartialOrd,
    F: Fn(N) -> T,
{
    let two = N::one() + N::one();

    while start < end {
        let mid = start + (end - start) / two;
        let cmp = f(mid).partial_cmp(t).unwrap();

        if cmp == Ordering::Less {
            start = mid + N::one();
        } else if cmp == Ordering::Greater {
            end = mid;
        } else {
            return Ok(mid);
        }
    }

    Err(start)
}
