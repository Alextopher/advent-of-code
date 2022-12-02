use std::{cmp::Ordering, collections::VecDeque};

impl<T: ?Sized> IterJunk for T where T: Iterator {}

pub trait IterJunk: Iterator {
    fn selection_sorted(self) -> SelectionBy<Self::Item, fn(&Self::Item, &Self::Item) -> Ordering>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        SelectionBy::new(self.collect(), Self::Item::cmp)
    }

    fn selection_sorted_by<F>(self, cmp: F) -> SelectionBy<Self::Item, F>
    where
        Self: Sized,
        Self::Item: Ord,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        SelectionBy::new(self.collect(), cmp)
    }

    fn selection_sorted_by_key<K, F>(self, k: F) -> SelectionBy<K, fn(&K, &K) -> Ordering>
    where
        Self: Sized,
        K: Ord,
        F: FnMut(Self::Item) -> K,
    {
        SelectionBy::new(self.map(k).collect(), K::cmp)
    }
}

pub struct SelectionBy<Item, F> {
    head: VecDeque<Item>,
    body: Vec<Item>,
    tail: VecDeque<Item>,
    f: F,
}

impl<Item, F> SelectionBy<Item, F> {
    fn new(body: Vec<Item>, f: F) -> Self {
        Self {
            head: VecDeque::new(),
            body,
            tail: VecDeque::new(),
            f,
        }
    }
}

impl<Item, F> Iterator for SelectionBy<Item, F>
where
    Item: Ord,
    F: FnMut(&Item, &Item) -> Ordering + Copy,
{
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.head.is_empty() {
            self.head.pop_front()
        } else if !self.body.is_empty() {
            let (min_index, _) = self
                .body
                .iter()
                .enumerate()
                .min_by(|(_, v1), (_, v2)| (self.f)(*v1, *v2))?;

            Some(self.body.swap_remove(min_index))
        } else {
            self.tail.pop_front()
        }
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let a = self.head.len();
        let b = a + self.body.len();
        let c = a + b + self.tail.len();

        if n <= a {
            self.head.drain(0..n).last()
        } else if a < n && n <= b {
            let k = n - a;
            self.head.clear();

            self.body.select_nth_unstable_by(k, self.f);
            self.body.drain(0..k).last()
        } else if b < n && n <= c {
            let k = n - b;
            self.head.clear();
            self.body.clear();
    
            self.tail.drain(0..k).last()
        } else {
            self.head.clear();
            self.body.clear();
            self.tail.clear();

            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.head.len() + self.body.len() + self.tail.len();
        (len, Some(len))
    }
}

impl<Item, F> DoubleEndedIterator for SelectionBy<Item, F>
where
    Item: Ord,
    F: FnMut(&Item, &Item) -> Ordering + Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.tail.is_empty() {
            self.head.pop_back()
        } else if !self.body.is_empty() {
            let (max_index, _) = self
                .body
                .iter()
                .enumerate()
                .max_by(|(_, v1), (_, v2)| (self.f)(*v1, *v2))?;

            Some(self.body.swap_remove(max_index))
        } else {
            self.head.pop_back()
        }
    }
}

impl<Item, F> ExactSizeIterator for SelectionBy<Item, F>
where
    Item: Ord,
    F: FnMut(&Item, &Item) -> Ordering + Copy,
{
    fn len(&self) -> usize {
        let (lower, upper) = self.size_hint();
        debug_assert_eq!(upper, Some(lower));
        lower
    }
}
